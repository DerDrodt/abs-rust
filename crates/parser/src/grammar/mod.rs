use crate::parser::Parser;
use crate::SyntaxKind::*;

mod expressions;
mod stmts;

pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    todo!();
    m.complete(p, MODULE);
}

pub(crate) fn path(p: &mut Parser) {
    todo!()
}

pub(crate) fn ty(p: &mut Parser) {
    todo!()
}

pub(crate) fn decl(p: &mut Parser) {
    todo!()
}

pub(crate) fn block(p: &mut Parser) {
    todo!()
}

pub(crate) fn pattern(p: &mut Parser) {
    todo!()
}

pub(crate) mod fragments {
    pub(crate) use super::*;

    pub(crate) fn expr(p: &mut Parser) {
        let _ = expressions::expr(p);
    }

    pub(crate) fn stmt(p: &mut Parser) {
        stmts::stmt(p);
    }
}
