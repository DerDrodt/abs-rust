use crate::{
    parser::CompletedMarker,
    parser::{Marker, Parser},
    SyntaxKind::{self, *},
    T,
};

use super::{
    annotations_opt, case_stmt_branch, expr, name_expr, pure_expr, pure_expr_list, ty, var_name,
};

pub(super) fn stmt(p: &mut Parser) {
    let m = p.start();
    annotations_opt(p);
    match p.current() {
        // VarDecl
        // Assign
        T![skip] => skip(p, m),
        T![return] => return_(p, m),
        T![assert] => assert(p, m),
        T!['{'] => block(p, m),
        T![if] => if_(p, m),
        T![switch] => switch(p, m),
        T![case] => case(p, m),
        T![while] => while_(p, m),
        T![foreach] => foreach(p, m),
        T![try] => try_catch_finally(p, m),
        T![await] => await_(p, m),
        T![suspend] => suspend(p, m),
        T![duration] => duration(p, m),
        T![throw] => throw(p, m),
        T![die] => die(p, m),
        T![movecogto] => move_cog_to(p, m),
        _ => expr_stmt(p, m),
    }
}

fn var_decl(p: &mut Parser, m: Marker) {
    ty(p);
    var_name(p);
    if p.eat(T![=]) {
        expr(p);
    }
    p.expect(T![;]);
    m.complete(p, VAR_DECL_STMT);
}

fn skip(p: &mut Parser, m: Marker) {
    p.bump(T![skip]);
    p.expect(T![;]);
    m.complete(p, SKIP_STMT);
}

fn return_(p: &mut Parser, m: Marker) {
    p.bump(T![return]);
    expr(p);
    p.expect(T![;]);
    m.complete(p, RETURN_STMT);
}

fn assert(p: &mut Parser, m: Marker) {
    p.bump(T![assert]);
    expr(p);
    p.expect(T![;]);
    m.complete(p, ASSERT_STMT);
}

fn block(p: &mut Parser, m: Marker) {
    p.bump(T!['{']);
    while !p.at(EOF) && !p.at(T!['}']) {
        stmt(p);
    }
    p.expect(T!['}']);
    m.complete(p, BLOCK);
}

fn if_(p: &mut Parser, m: Marker) {}

#[inline]
fn switch(p: &mut Parser, m: Marker) {
    p.bump(T![switch]);
    continue_case(p, m);
}

fn case(p: &mut Parser, m: Marker) {
    // TODO: handle expr
    p.bump(T![case]);
    continue_case(p, m);
}

fn continue_case(p: &mut Parser, m: Marker) {
    p.expect(T!['(']);
    pure_expr(p);
    p.expect(T![')']);
    p.expect(T!['{']);
    while !p.at(EOF) && !p.at(T!['}']) {
        case_stmt_branch(p);
    }
    p.expect(T!['}']);
    m.complete(p, SWITCH_STMT);
}

fn while_(p: &mut Parser, m: Marker) {
    p.bump(T![while]);
    p.expect(T!['(']);
    pure_expr(p);
    p.expect(T![')']);
    stmt(p);
    m.complete(p, WHILE_STMT);
}

fn foreach(p: &mut Parser, m: Marker) {
    p.bump(T![foreach]);
    p.expect(T!['(']);
    var_name(p);
    if p.eat(T![,]) {
        var_name(p);
    }
    p.expect(T![in]);
    pure_expr(p);
    p.expect(T![')']);
    stmt(p);
    m.complete(p, FOREACH_STMT);
}

fn await_(p: &mut Parser, m: Marker) {
    let expr_m = p.start();
    p.bump(T![await]);
    let is_call = guard_or_call(p, expr_m);
    p.expect(T![;]);
    if is_call {
        m.complete(p, EXPR_STMT);
    } else {
        m.complete(p, AWAIT_STMT);
    }
}

fn likely_claim_guard(p: &Parser) -> bool {
    (p.at(T![this]) && p.nth_at(1, T![.]) && p.nth_at(2, T![lower_ident]) && p.nth_at(3, T![?]))
        || p.at(T![lower_ident]) && p.nth_at(1, T![?])
}

fn guard_or_call(p: &mut Parser, m: Marker) -> bool {
    let mut lhs = if likely_claim_guard(p) {
        claim_guard(p)
    } else if p.at(T![duration]) {
        duration_guard(p)
    } else {
        match pure_expr(p) {
            Some(m) => m,
            _ => {
                p.error("expected guard or call");
                m.abandon(p);
                return false;
            }
        }
    };
    if p.eat(T![!]) {
        // await async call
        var_name(p);
        p.expect(T!['(']);
        pure_expr_list(p);
        p.expect(T![')']);
        m.complete(p, ASYNC_CALL_EXPR);
        return true;
    } else {
        loop {
            if p.at(EOF) || !p.at(T![&]) {
                break;
            }
            let m = lhs.precede(p);
            p.bump(T![&]);
            if likely_claim_guard(p) {
                claim_guard(p);
            } else if p.at(T![duration]) {
                duration_guard(p);
            } else {
                pure_expr(p);
            }
            lhs = m.complete(p, AND_GUARD);
        }
    }
    m.abandon(p);
    false
}

fn claim_guard(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    name_expr(p);
    p.expect(T![?]);
    m.complete(p, CLAIM_GUARD)
}

fn duration_guard(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![duration]);
    p.expect(T!['(']);
    pure_expr(p);
    p.expect(T![,]);
    pure_expr(p);
    p.expect(T![')']);
    m.complete(p, DURATION_GUARD)
}

fn try_catch_finally(p: &mut Parser, m: Marker) {
    p.bump(T![try]);
    stmt(p);
    p.expect(T![catch]);
    if p.eat(T!['{']) {
        while !p.at(EOF) && !p.at(T!['}']) {
            case_stmt_branch(p);
        }
        p.expect(T!['}']);
    } else {
        case_stmt_branch(p);
    }
    if p.eat(T![finally]) {
        stmt(p);
    }
    m.complete(p, TRY_CATCH_FINALLY_STMT);
}

fn suspend(p: &mut Parser, m: Marker) {
    p.bump(T![suspend]);
    p.expect(T![;]);
    m.complete(p, SUSPEND_STMT);
}

fn duration(p: &mut Parser, m: Marker) {
    p.bump(T![duration]);
    p.expect(T!['(']);
    pure_expr(p);
    p.expect(T![,]);
    pure_expr(p);
    p.expect(T![')']);
    p.expect(T![;]);
    m.complete(p, DURATION_STMT);
}

fn throw(p: &mut Parser, m: Marker) {
    p.bump(T![throw]);
    pure_expr(p);
    p.expect(T![;]);
    m.complete(p, THROW_STMT);
}

fn die(p: &mut Parser, m: Marker) {
    p.bump(T![die]);
    p.expect(T![;]);
    m.complete(p, DIE_STMT);
}

fn move_cog_to(p: &mut Parser, m: Marker) {
    p.bump(T![movecogto]);
    pure_expr(p);
    p.expect(T![;]);
    m.complete(p, MOVE_COG_TO_STMT);
}

fn expr_stmt(p: &mut Parser, m: Marker) {
    expr(p);
    p.expect(T![;]);
    m.complete(p, EXPR_STMT);
}
