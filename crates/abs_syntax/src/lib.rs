pub mod ast;
mod parsing;
pub mod syntax_error;
pub mod syntax_node;
mod validation;

use std::{marker::PhantomData, sync::Arc};

pub use ast::{AstNode, Module};
pub use parser::{SyntaxKind, T};
pub use syntax_node::*;

pub use rowan::{SmolStr, TextRange, TextSize};

pub use syntax_error::SyntaxError;

use text_edit::Indel;

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<Vec<SyntaxError>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse {
            green: self.green.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse {
            green,
            errors: Arc::new(errors),
            _ty: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<T: AstNode> Parse<T> {
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green: self.green,
            errors: self.errors,
            _ty: PhantomData,
        }
    }

    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &*self.errors
    }

    pub fn ok(self) -> Result<T, Arc<Vec<SyntaxError>>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse {
                green: self.green,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }
}

impl Parse<Module> {
    pub fn debug_dump(&self) -> String {
        let mut buf = format!("{:#?}", self.tree().syntax());
        for err in self.errors.iter() {
            use ::std::fmt::Write as _;
            let _ = ::std::write!(buf, "error {:?}: {}\n", err.range(), err);
        }
        buf
    }

    pub fn reparse(&self, indel: &Indel) -> Parse<Module> {
        self.incremental_reparse(indel)
            .unwrap_or_else(|| self.full_reparse(indel))
    }

    fn incremental_reparse(&self, indel: &Indel) -> Option<Parse<Module>> {
        // FIXME: validation errors are not handled here
        parsing::incremental_reparse(self.tree().syntax(), indel, self.errors.to_vec()).map(
            |(green_node, errors, _reparsed_range)| Parse {
                green: green_node,
                errors: Arc::new(errors),
                _ty: PhantomData,
            },
        )
    }

    fn full_reparse(&self, indel: &Indel) -> Parse<Module> {
        let mut text = self.tree().syntax().text().to_string();
        indel.apply(&mut text);
        Module::parse(&text)
    }
}

impl Module {
    pub fn parse(text: &str) -> Parse<Module> {
        let (green, mut errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        if cfg!(debug_assertions) {
            validation::validate_block_structure(&root);
        }

        errors.extend(validation::validate(&root));

        assert_eq!(root.kind(), SyntaxKind::MODULE);
        Parse {
            green,
            errors: Arc::new(errors),
            _ty: PhantomData,
        }
    }
}

impl ast::Path {
    /// Returns `text`, parsed as a path, but only if it has no errors.
    pub fn parse(text: &str) -> Result<Self, ()> {
        parsing::parse_text_fragment(text, parser::FragmentKind::Path)
    }
}

impl ast::Pattern {
    /// Returns `text`, parsed as a pattern, but only if it has no errors.
    pub fn parse(text: &str) -> Result<Self, ()> {
        parsing::parse_text_fragment(text, parser::FragmentKind::Pattern)
    }
}

impl ast::Expr {
    /// Returns `text`, parsed as an expression, but only if it has no errors.
    pub fn parse(text: &str) -> Result<Self, ()> {
        parsing::parse_text_fragment(text, parser::FragmentKind::Expr)
    }
}

impl ast::Type {
    /// Returns `text`, parsed as an type reference, but only if it has no errors.
    pub fn parse(text: &str) -> Result<Self, ()> {
        parsing::parse_text_fragment(text, parser::FragmentKind::Type)
    }
}

/// Matches a `SyntaxNode` against an `ast` type.
///
/// # Example:
///
/// ```ignore
/// match_ast! {
///     match node {
///         ast::CallExpr(it) => { ... },
///         ast::MethodCallExpr(it) => { ... },
///         ast::MacroCall(it) => { ... },
///         _ => None,
///     }
/// }
/// ```
#[macro_export]
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { match_ast!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( ast::$ast:ident($it:ident) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = ast::$ast::cast($node.clone()) { $res } else )*
        { $catch_all }
    }};
}
