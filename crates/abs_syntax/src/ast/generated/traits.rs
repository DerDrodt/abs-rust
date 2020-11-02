use crate::ast::{self, support, AstNode};

pub trait VarNameOwner: AstNode {
    fn name(&self) -> Option<ast::VarName> {
        support::child(self.syntax())
    }
}

pub trait TypeNameOwner: AstNode {
    fn name(&self) -> Option<ast::TypeName> {
        support::child(self.syntax())
    }
}

pub trait PureExprListOwner: AstNode {
    fn pure_expr_list(&self) -> Option<ast::PureExprList> {
        support::child(self.syntax())
    }
}
