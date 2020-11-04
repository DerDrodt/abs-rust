use crate::{grammar::ty_path, grammar::Parser, parser::Marker, SyntaxKind::*, T};

use super::method_sig;

pub(crate) fn interface(p: &mut Parser, m: Marker) {
    p.bump(T![interface]);
    ty_path(p);
    if p.at(T![extends]) {
        extends_list(p);
    }
    p.expect(T!['{']);
    while !p.at(EOF) && !p.at(T!['}']) {
        interface_item(p);
    }
    p.expect(T!['}']);
    m.complete(p, INTERFACE_DECL);
}

fn extends_list(p: &mut Parser) {
    let m = p.start();
    p.bump(T![extends]);
    ty_path(p);
    while p.eat(T![,]) {
        ty_path(p);
    }
    m.complete(p, EXTENDS_LIST);
}

fn interface_item(p: &mut Parser) {
    let m = p.start();
    method_sig(p);
    m.complete(p, INTERFACE_ITEM);
}
