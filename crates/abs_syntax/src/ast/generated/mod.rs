mod nodes;
mod tokens;
mod traits;

pub use nodes::*;
pub use tokens::*;
pub use traits::*;

use crate::SyntaxKind::{self, *};

use super::AstNode;

pub use {nodes::*, tokens::*};

impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        if PureExpr::can_cast(kind) {
            return true;
        }
        EffExpr::can_cast(kind)
    }

    fn cast(syntax: crate::SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(e) = PureExpr::cast(syntax) {
            Some(Expr::PureExpr(e))
        } else if let Some(e) = EffExpr::cast(syntax) {
            Some(Expr::EffExpr(e))
        } else {
            None
        }
    }

    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Expr::PureExpr(it) => it.syntax(),
            Expr::EffExpr(it) => it.syntax(),
        }
    }
}
