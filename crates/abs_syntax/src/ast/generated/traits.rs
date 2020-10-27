use crate::ast::{self, support, AstNode};

pub trait NameOwner: AstNode {
    fn name(&self) -> Option<ast::Name> {
        support::child(self.syntax())
    }
}

pub trait PureExprListOwner: AstNode {
    fn pure_expr_list(&self) -> Option<ast::PureExprList> {
        support::child(self.syntax())
    }
}
