use crate::{
    grammar::gen_arg_list, grammar::ty, grammar::ty_name, grammar::ty_path, grammar::Parser,
    parser::Marker, SyntaxKind::*, T,
};

pub(crate) fn datatype(p: &mut Parser, m: Marker) {
    p.bump(T![data]);
    ty_path(p);
    if p.at(T![<]) {
        gen_arg_list(p);
    }
    if p.at(T![=]) {
        data_constr_list(p);
    }
    p.expect(T![;]);
    m.complete(p, DATA_TYPE_DECL);
}

fn data_constr_list(p: &mut Parser) {
    let m = p.start();
    p.bump(T![=]);
    data_constr(p);
    while p.eat(T![|]) {
        data_constr(p);
    }
    m.complete(p, DATA_CONSTRUCTOR_LIST);
}

fn data_constr(p: &mut Parser) {
    let m = p.start();
    ty_name(p);
    if p.eat(T!['(']) {
        data_constr_arg_list(p);
        p.expect(T![')']);
    }
    m.complete(p, DATA_CONSTRUCTOR);
}

pub(crate) fn data_constr_arg_list(p: &mut Parser) {
    let m = p.start();
    data_constr_arg(p);
    while p.eat(T![,]) {
        data_constr_arg(p);
    }
    m.complete(p, DATA_CONSTRUCTOR_ARG_LIST);
}

fn data_constr_arg(p: &mut Parser) {
    let m = p.start();
    ty(p);
    if p.at(T![cap_ident]) {
        ty_name(p);
    }
    m.complete(p, DATA_CONSTRUCTOR_ARG);
}
