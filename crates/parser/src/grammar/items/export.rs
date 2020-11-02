use crate::{
    grammar::{any_name, path},
    parser::{Marker, Parser},
    SyntaxKind, T,
};

pub(crate) fn export(p: &mut Parser) {
    let m = p.start();
    p.bump(T![export]);
    if p.at(T![*]) {
        star(p, m)
    } else {
        partial(p, m)
    }
}

fn star(p: &mut Parser, m: Marker) {
    p.bump(T![*]);
    if p.eat(T![from]) {
        path(p);
    }
    p.bump(T![;]);
    m.complete(p, SyntaxKind::STAR_EXPORT);
}

fn partial(p: &mut Parser, m: Marker) {
    any_name(p);
    while p.eat(T![,]) {
        any_name(p);
    }
    if p.eat(T![from]) {
        path(p);
    }
    p.bump(T![;]);
    m.complete(p, SyntaxKind::PARTIAL_EXPORT);
}
