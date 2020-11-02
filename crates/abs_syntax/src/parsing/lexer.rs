use std::{convert::TryInto, str::Chars};

use parser::T;
use rowan::{TextRange, TextSize};

use crate::{
    SyntaxError,
    SyntaxKind::{self},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The length of the token.
    pub len: TextSize,
}

pub fn tokenizer(mut input: &str) -> impl Iterator<Item = (Token, Option<&'static str>)> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }
        let res = first_token(input);
        input = &input[res.0.len.into()..];
        Some(res)
    })
}

/// Parses the first token from the provided input string.
fn first_token(input: &str) -> (Token, Option<&'static str>) {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

/// Break a string up into its component tokens.
pub fn tokenize(text: &str) -> (Vec<Token>, Vec<SyntaxError>) {
    if text.is_empty() {
        return Default::default();
    }

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    let mut offset: usize = 0;

    for (mut tok, err_msg) in tokenizer(text) {
        let tok_len = tok.len;
        let tok_range = TextRange::at(offset.try_into().unwrap(), tok_len);

        let tok_text = &text[tok_range];
        if let Some(kw) = SyntaxKind::from_keyword(tok_text) {
            tok.kind = kw;
        }

        tokens.push(tok);

        if let Some(err_msg) = err_msg {
            errors.push(SyntaxError::new(err_msg, tok_range))
        }

        let len: usize = tok.len.try_into().unwrap();
        offset += len;
    }

    (tokens, errors)
}

fn is_whitespace(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | ' ')
}

fn is_id_start(c: char) -> bool {
    matches!(c, 'a'..='z'|'A'..='Z'|'_')
}

fn is_id_continue(c: char) -> bool {
    matches!(c, 'a'..='z'|'A'..='Z'|'_'|'0'..='9')
}

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `nth_char` method,
/// and position can be shifted forward via `bump` method.
pub(crate) struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    /// Returns nth character relative to the current cursor position.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }

    /// Peeks the next symbol from the input stream without consuming it.
    pub(crate) fn first(&self) -> char {
        self.nth_char(0)
    }

    /* /// Peeks the second symbol from the input stream without consuming it.
    pub(crate) fn second(&self) -> char {
        self.nth_char(1)
    } */

    /// Checks if there is nothing more to consume.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Returns amount of already consumed symbols.
    pub(crate) fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    /// Returns a `Chars` iterator over the remaining characters.
    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        Some(c)
    }
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> (Token, Option<&'static str>) {
        let first_char = self.bump().unwrap();

        let mut kind = match first_char {
            '/' => match self.first() {
                '/' => self.line_comment(),
                '*' => return self.block_comment(),
                _ => T![/],
            },

            c if is_whitespace(c) => self.whitespace(),

            'a'..='z' => self.ident(false),

            'A'..='Z' => self.ident(true),

            c @ '0'..='9' => return self.number(c),

            ';' => T![;],
            ',' => T![,],
            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => T![']'],
            '<' => T![<],
            '>' => T![>],
            '?' => T![?],
            '&' => T![&],
            '+' => T![+],
            '*' => T![*],
            '%' => T![%],
            '.' => T![.],
            '=' => T![=],
            '!' => T![!],
            '-' => T![-],
            ':' => T![:],
            '|' => T![|],

            '"' => {
                if !self.double_quoted_string() {
                    let tok = Token {
                        kind: SyntaxKind::STRING,
                        len: self.len_consumed().try_into().unwrap(),
                    };
                    return (
                        tok,
                        Some("Missing trailing `\"` symbol to terminate the string literal"),
                    );
                }
                SyntaxKind::STRING
            }
            _ => SyntaxKind::ERROR,
        };

        let len: TextSize = self.len_consumed().try_into().unwrap();
        let len_u32: u32 = len.into();

        if first_char == '_' && len_u32 == 1 {
            kind = SyntaxKind::UNDERSCORE
        }

        (Token { kind, len }, None)
    }

    fn line_comment(&mut self) -> SyntaxKind {
        self.bump();

        self.eat_while(|c| c != '\n');

        SyntaxKind::COMMENT
    }

    fn block_comment(&mut self) -> (Token, Option<&'static str>) {
        self.bump();

        let mut closed = false;
        while let Some(c) = self.bump() {
            match c {
                '*' if self.first() == '/' => {
                    self.bump();
                    closed = true;
                    break;
                }
                _ => (),
            }
        }

        (
            Token {
                kind: SyntaxKind::COMMENT,
                len: self.len_consumed().try_into().unwrap(),
            },
            if closed {
                None
            } else {
                Some("Missing trailing `*/` symbols to terminate the block comment")
            },
        )
    }

    fn whitespace(&mut self) -> SyntaxKind {
        self.eat_while(is_whitespace);
        SyntaxKind::WHITESPACE
    }

    fn ident(&mut self, cap: bool) -> SyntaxKind {
        self.eat_while(is_id_continue);
        if cap {
            SyntaxKind::CAP_IDENT
        } else {
            SyntaxKind::LOW_IDENT
        }
    }

    fn number(&mut self, first_digit: char) -> (Token, Option<&'static str>) {
        if first_digit != '0' {
            self.eat_decimal_digits();
        }

        match self.first() {
            '.' => {
                self.bump();
                let mut empty_exponent = false;
                match self.first() {
                    'e' | 'E' => {
                        self.bump();
                        empty_exponent = !self.eat_float_exponent();
                    }
                    _ => (),
                }
                (
                    Token {
                        kind: SyntaxKind::FLOAT_NUMBER,
                        len: self.len_consumed().try_into().unwrap(),
                    },
                    if empty_exponent {
                        Some("Missing digits after the exponent symbol")
                    } else {
                        None
                    },
                )
            }
            _ => (
                Token {
                    kind: SyntaxKind::INT_NUMBER,
                    len: self.len_consumed().try_into().unwrap(),
                },
                None,
            ),
        }
    }

    fn double_quoted_string(&mut self) -> bool {
        while let Some(c) = self.bump() {
            match c {
                '"' => return true,
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    self.bump();
                }
                _ => (),
            }
        }
        false
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    /// Eats the float exponent. Returns true if at least one digit was met,
    /// and returns false otherwise.
    fn eat_float_exponent(&mut self) -> bool {
        if self.first() == '-' || self.first() == '+' {
            self.bump();
        }
        self.eat_decimal_digits()
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}
