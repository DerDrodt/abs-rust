use crate::{grammar::ty_path, grammar::Parser, parser::Marker, SyntaxKind::*, T};

use super::data_constr_arg_list;

pub(crate) fn exception(p: &mut Parser, m: Marker) {
    p.bump(T![exception]);
    ty_path(p);
    if p.eat(T!['(']) {
        data_constr_arg_list(p);
        p.expect(T![')']);
    }
    p.expect(T![;]);
    m.complete(p, EXCEPTION_DECL);
}
