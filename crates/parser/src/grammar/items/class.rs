use crate::{
    grammar::block, grammar::case_stmt_branch, grammar::param_list, grammar::ty_path,
    grammar::Parser, parser::Marker, SyntaxKind::*, T,
};

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

    if p.at(T![uses]) {}

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

fn trait_use(p: &mut Parser) {
    let m = p.start();
    p.bump(T![uses]);

    trait_expr(p);

    p.expect(T![;]);
    m.complete(p, TRAIT_USE);
}

fn trait_expr(p: &mut Parser) {
    let m = p.start();
    basic_trait_expr(p);
    while !p.at(T![;]) && !p.at(EOF) {
        trait_op(p);
    }
    m.complete(p, TRAIT_EXPR);
}

fn basic_trait_expr(p: &mut Parser) {}

fn trait_op(p: &mut Parser) {}
