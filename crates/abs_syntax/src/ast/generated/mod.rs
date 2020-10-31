mod nodes;
mod tokens;
mod traits;

pub use nodes::*;
pub use tokens::*;
pub use traits::*;

use crate::SyntaxKind::{self};

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
        Some(if PureExpr::can_cast(syntax.kind()) {
            Expr::PureExpr(PureExpr::cast(syntax)?)
        } else {
            Expr::EffExpr(EffExpr::cast(syntax)?)
        })
    }

    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Expr::PureExpr(it) => it.syntax(),
            Expr::EffExpr(it) => it.syntax(),
        }
    }
}
