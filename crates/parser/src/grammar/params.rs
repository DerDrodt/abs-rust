use super::{annotations_opt, ty, var_name, Parser, SyntaxKind::*, T};

pub(crate) fn param_list(p: &mut Parser) {
    let m = p.start();
    p.bump(T!['(']);

    while !p.at(EOF) && !p.at(T![')']) {
        param(p);
        if !p.at(T![')']) {
            p.expect(T![,]);
        }
    }

    p.expect(T!['(']);
    m.complete(p, PARAM_LIST);
}

fn param(p: &mut Parser) {
    annotations_opt(p);
    ty(p);
    var_name(p);
}
