use crate::{
    grammar::{any_name, path},
    parser::{Marker, Parser},
    SyntaxKind, T,
};

pub(crate) fn import(p: &mut Parser) {
    let m = p.start();
    p.bump(T![import]);
    if p.at(T![*]) {
        star(p, m)
    } else {
        from_or_unqualified(p, m)
    }
}

pub(crate) fn star(p: &mut Parser, m: Marker) {
    p.bump(T![*]);
    p.bump(T![from]);
    path(p);
    p.bump(T![;]);
    m.complete(p, SyntaxKind::STAR_IMPORT);
}

pub(crate) fn from_or_unqualified(p: &mut Parser, m: Marker) {
    any_name(p);
    while p.eat(T![,]) {
        any_name(p);
    }
    let kind = if p.eat(T![from]) {
        path(p);
        SyntaxKind::FROM_IMPORT
    } else {
        SyntaxKind::UNQUALIFIED_IMPORT
    };
    p.bump(T![;]);
    m.complete(p, kind);
}
