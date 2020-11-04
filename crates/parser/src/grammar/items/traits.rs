use crate::{
    grammar::ty_path,
    grammar::{ty_name, Parser},
    parser::Marker,
    SyntaxKind::*,
    T,
};

use super::{method_decl, method_sig};

pub(crate) fn trait_decl(p: &mut Parser, m: Marker) {
    p.bump(T![trait]);
    ty_path(p);
    p.expect(T![=]);
    trait_expr(p);
    m.complete(p, TRAIT_DECL);
}

pub(crate) fn trait_expr(p: &mut Parser) {
    let m = p.start();
    basic_trait_expr(p);
    while !p.at(T![;]) && !p.at(EOF) {
        trait_op(p);
    }
    m.complete(p, TRAIT_EXPR);
}

pub(crate) fn basic_trait_expr(p: &mut Parser) {
    if p.at(T!['{']) {
        trait_method_set(p)
    } else {
        if p.at(CAP_IDENT) && !p.nth_at(1, T![<]) && !p.at(T![lower_ident]) {
            trait_name(p);
        } else {
            trait_method(p);
        }
    }
}

fn trait_method_set(p: &mut Parser) {
    let m = p.start();

    p.bump(T!['{']);

    while !p.at(EOF) && !p.at(T!['}']) {
        method_decl(p);
    }

    p.expect(T!['}']);

    m.complete(p, TRAIT_METHOD_SET);
}

fn trait_method(p: &mut Parser) {
    let m = p.start();

    method_decl(p);

    m.complete(p, TRAIT_METHOD);
}

fn trait_name(p: &mut Parser) {
    let m = p.start();
    ty_name(p);
    m.complete(p, TRAIT_NAME);
}

pub(crate) fn trait_op(p: &mut Parser) {
    match p.current() {
        T![removes] => {
            if p.nth_at(1, T!['{']) {
                trait_remove_set(p);
            } else {
                trait_remove_sig(p);
            }
        }
        T![adds] => trait_add(p),
        T![modifies] => trait_modifies(p),
        _ => p.error("expected trait operation"),
    }
}

fn trait_remove_sig(p: &mut Parser) {
    let m = p.start();
    p.bump(T![removes]);
    while p.at(T!['[']) || p.at(CAP_IDENT) {
        method_sig(p);
    }
    m.complete(p, TRAIT_REMOVE_SIG);
}

fn trait_remove_set(p: &mut Parser) {
    let m = p.start();
    p.bump(T![removes]);
    p.bump(T!['{']);
    while !p.at(EOF) && !p.at(T!['}']) {
        method_sig(p);
    }
    m.complete(p, TRAIT_REMOVE_SET);
}

fn trait_add(p: &mut Parser) {
    let m = p.start();
    p.bump(T![adds]);
    basic_trait_expr(p);
    m.complete(p, TRAIT_ADD);
}

fn trait_modifies(p: &mut Parser) {
    let m = p.start();
    p.bump(T![modifies]);
    basic_trait_expr(p);
    m.complete(p, TRAIT_MODIFIES);
}

pub(crate) fn trait_use(p: &mut Parser) {
    let m = p.start();
    p.bump(T![uses]);
    trait_expr(p);
    p.expect(T![;]);
    m.complete(p, TRAIT_USE);
}
