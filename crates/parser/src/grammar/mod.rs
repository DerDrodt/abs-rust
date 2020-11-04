pub(crate) use crate::{parser::Parser, SyntaxKind, T};
use crate::{token_set::TokenSet, SyntaxKind::*};

mod annotations;
mod branches;
mod expressions;
mod gen_args;
mod items;
mod params;
mod pattern;
mod stmts;

pub use annotations::*;
pub(crate) use branches::*;
pub use expressions::*;
pub(crate) use gen_args::*;
pub use items::*;
pub use params::*;
pub(crate) use pattern::*;

pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    p.bump(T![module]);
    path(p);
    p.bump(T![;]);

    while p.at(T![import]) {
        import(p);
    }
    while p.at(T![export]) {
        export(p);
    }
    while !p.at(T!['{']) && !p.at(SyntaxKind::EOF) {
        decl(p);
    }
    if p.at(T!['{']) {
        block(p);
    }

    m.complete(p, MODULE);
}

pub(crate) fn path(p: &mut Parser) {
    let m = p.start();
    if p.at(CAP_IDENT) {
        ty_path(p);
        p.expect(T![.]);
    }
    var_name(p);
    m.complete(p, PATH);
}

pub(crate) fn ty_path(p: &mut Parser) {
    let path = p.start();
    ty_name(p);
    let mut qual = path.complete(p, TYPE_PATH);
    while p.at(T![.]) {
        let path = qual.precede(p);
        p.bump(T![.]);
        ty_name(p);
        let path = path.complete(p, TYPE_PATH);
        qual = path;
    }
}

pub(crate) fn ty(p: &mut Parser) {
    let m = p.start();
    annotations_opt(p);
    path(p);
    if p.eat(T![<]) {
        ty(p);
        while p.eat(T![,]) {
            ty(p);
        }
        p.expect(T![>]);
    }
    m.complete(p, TYPE);
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

fn var_name_r(p: &mut Parser, recovery: TokenSet) {
    if p.at(LOW_IDENT) {
        let m = p.start();
        p.bump(LOW_IDENT);
        m.complete(p, VAR_NAME);
    } else {
        p.err_recover("expected a name", recovery);
    }
}

pub(crate) fn var_name(p: &mut Parser) {
    var_name_r(p, TokenSet::EMPTY)
}

fn ty_name_r(p: &mut Parser, recovery: TokenSet) {
    if p.at(CAP_IDENT) {
        let m = p.start();
        p.bump(CAP_IDENT);
        m.complete(p, TYPE_NAME);
    } else {
        p.err_recover("expected a name", recovery);
    }
}

pub(crate) fn ty_name(p: &mut Parser) {
    ty_name_r(p, TokenSet::EMPTY)
}

fn any_name_r(p: &mut Parser, recovery: TokenSet) {
    if p.at(CAP_IDENT) {
        ty_name_r(p, recovery)
    } else if p.at(LOW_IDENT) {
        var_name_r(p, recovery)
    } else {
        p.err_recover("expected a name", recovery);
    }
}

pub(crate) fn any_name(p: &mut Parser) {
    any_name_r(p, TokenSet::EMPTY)
}
