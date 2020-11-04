use crate::{
    grammar::gen_arg_list, grammar::param_list, grammar::pure_expr, grammar::ty, grammar::var_name,
    grammar::Parser, parser::Marker, SyntaxKind::*, T,
};

pub(crate) fn maybe_par_fn(p: &mut Parser, m: Marker) {
    p.bump(T![def]);
    ty(p);
    if p.at(T![<]) {
        gen_arg_list(p);
    }
    let par = if p.nth_at(1, T![lower_ident]) || p.nth_at(2, T![')']) {
        p.expect(T!['(']);
        fn_name_list(p);
        p.expect(T![')']);
        true
    } else {
        false
    };
    param_list(p);
    p.expect(T![=]);
    if par && p.at(T![builtin]) {
        p.bump(T![builtin]);
    } else {
        pure_expr(p);
    }
    p.expect(T![;]);
    m.complete(
        p,
        if par {
            PAR_FUNCTION_DECL
        } else {
            FUNCTION_DECL
        },
    );
}

fn fn_name_list(p: &mut Parser) {
    let m = p.start();
    if p.at(T![lower_ident]) {
        var_name(p);
        while p.eat(T![,]) {
            var_name(p);
        }
    }
    m.complete(p, FUNCTION_NAME_LIST);
}
