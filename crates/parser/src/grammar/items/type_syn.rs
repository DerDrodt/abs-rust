use crate::{grammar::ty, grammar::ty_path, grammar::Parser, parser::Marker, SyntaxKind::*, T};

pub(crate) fn type_syn(p: &mut Parser, m: Marker) {
    p.bump(T![type]);
    ty_path(p);
    p.expect(T![=]);
    ty(p);
    p.expect(T![;]);
    m.complete(p, TYPE_SYN);
}
