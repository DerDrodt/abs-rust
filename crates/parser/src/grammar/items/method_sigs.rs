use crate::{
    grammar::{annotations_opt, param_list, ty, var_name, Parser},
    SyntaxKind,
};

pub(crate) fn method_sig(p: &mut Parser) {
    let m = p.start();
    annotations_opt(p);
    ty(p);
    var_name(p);
    param_list(p);
    m.complete(p, SyntaxKind::METHOD_SIG);
}
