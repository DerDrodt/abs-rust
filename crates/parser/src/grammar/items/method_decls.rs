use crate::{
    grammar::{block, Parser},
    SyntaxKind::*,
};

use super::method_sig;

pub(crate) fn method_decl(p: &mut Parser) {
    let m = p.start();
    method_sig(p);
    block(p);
    m.complete(p, METHOD_DECL);
}
