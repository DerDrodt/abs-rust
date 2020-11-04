use crate::{grammar::Parser, SyntaxKind::*, T};

use super::ty_name;

pub(crate) fn gen_arg_list(p: &mut Parser) {
    let m = p.start();
    p.bump(T![<]);
    ty_name(p);
    while p.eat(T![,]) {
        ty_name(p);
    }
    p.expect(T![>]);
    m.complete(p, GENERIC_ARG_LIST);
}
