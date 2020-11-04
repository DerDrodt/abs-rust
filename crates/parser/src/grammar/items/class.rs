use crate::{
    grammar::block, grammar::case_stmt_branch, grammar::param_list, grammar::ty_path,
    grammar::Parser, parser::Marker, SyntaxKind::*, T,
};

use super::{method_decl, trait_use};

pub(crate) fn class(p: &mut Parser, m: Marker) {
    p.bump(T![class]);
    ty_path(p);
    if p.at(T!['(']) {
        param_list(p)
    }
    if p.at(T![implements]) {
        implements_list(p);
    }
    p.expect(T!['{']);

    // TODO: fields or methods

    if p.at(T!['{']) {
        block(p);
    }

    if p.at(T![recover]) {
        recover_block(p);
    }

    while p.at(T![uses]) {
        trait_use(p);
    }

    while !p.at(EOF) && !p.at(T!['}']) {
        method_decl(p);
    }

    p.expect(T!['}']);
    m.complete(p, CLASS_DECL);
}

fn implements_list(p: &mut Parser) {
    let m = p.start();
    p.bump(T![implements]);
    ty_path(p);
    while p.eat(T![,]) {
        ty_path(p);
    }
    m.complete(p, IMPLEMENTS_LIST);
}

fn field_or_method(p: &mut Parser, m: Marker) {}

fn recover_block(p: &mut Parser) {
    let m = p.start();
    p.bump(T![recover]);
    p.expect(T!['{']);

    while !p.at(EOF) && !p.at(T!['}']) {
        case_stmt_branch(p);
    }

    p.expect(T!['}']);
    m.complete(p, RECOVER_BLOCK);
}
