mod event;
mod grammar;
mod parser;
#[macro_use]
mod syntax_kind;

#[macro_use]
mod token_set;

pub use syntax_kind::SyntaxKind;

pub(crate) use token_set::TokenSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError(pub Box<String>);

/// `TokenSource` abstracts the source of the tokens parser operates on.
///
/// Hopefully this will allow us to treat text and token trees in the same way!
pub trait TokenSource {
    fn current(&self) -> Token;

    /// Lookahead n token
    fn lookahead_nth(&self, n: usize) -> Token;

    /// bump cursor to next token
    fn bump(&mut self);

    /// Is the current token a specified keyword?
    fn is_keyword(&self, kw: &str) -> bool;
}

/// `Token` abstracts the cursor of `TokenSource` operates on.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Token {
    /// What is the current token?
    pub kind: SyntaxKind,

    /// Is the current token joined to the next one (`> >` vs `>>`).
    pub is_jointed_to_next: bool,
}

/// `TreeSink` abstracts details of a particular syntax tree implementation.
pub trait TreeSink {
    /// Adds new token to the current branch.
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: SyntaxKind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    fn error(&mut self, error: ParseError);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FragmentKind {
    Path,
    Expr,
    Statement,
    Type,
    Pattern,
    Decl,
    Block,
}

fn parse_from_tokens<F>(token_source: &mut dyn TokenSource, tree_sink: &mut dyn TreeSink, f: F)
where
    F: FnOnce(&mut parser::Parser),
{
    let mut p = parser::Parser::new(token_source);
    f(&mut p);
    let events = p.finish();
    event::process(tree_sink, events);
}

/// Parse given tokens into the given sink as a rust file.
pub fn parse(token_source: &mut dyn TokenSource, tree_sink: &mut dyn TreeSink) {
    parse_from_tokens(token_source, tree_sink, grammar::root);
}

pub fn parse_fragment(
    token_source: &mut dyn TokenSource,
    tree_sink: &mut dyn TreeSink,
    fragment_kind: FragmentKind,
) {
    let parser: fn(&'_ mut parser::Parser) = match fragment_kind {
        FragmentKind::Path => grammar::fragments::path,
        FragmentKind::Expr => grammar::fragments::expr,
        FragmentKind::Statement => grammar::fragments::stmt,
        FragmentKind::Type => grammar::fragments::ty,
        FragmentKind::Pattern => grammar::fragments::pattern,
        FragmentKind::Decl => grammar::fragments::decl,
        FragmentKind::Block => grammar::fragments::block,
    };
    parse_from_tokens(token_source, tree_sink, parser)
}
