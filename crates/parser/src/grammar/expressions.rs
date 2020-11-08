use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{self, *},
    T,
};

use super::{pattern, ty, ty_name, ty_path, var_name};

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    if p.at(T![new]) {
        new_expr(p, m)
    } else if p.at(T![await]) {
        async_call_await(p, m)
    } else if (p.at(INT_NUMBER) || p.at(T![core])) && p.nth_at(1, T![.]) {
        Some(original_call(p, m))
    } else {
        call_or_get(p, m)
    }
}

fn call_or_get(p: &mut Parser, m: Marker) -> Option<CompletedMarker> {
    let lhs = pure_expr_continue(p, m, 1)?;
    if p.at(T![!]) {
        // Async call
        let m = lhs.precede(p);
        var_name(p);
        p.at(T!['(']);
        pure_expr_list(p);
        p.at(T![')']);
        Some(m.complete(p, ASYNC_CALL_EXPR))
    } else if p.at(T![.]) {
        let m = lhs.precede(p);
        p.bump(T![.]);
        if p.at(T![get]) {
            p.bump(T![get]);
            Some(m.complete(p, GET_EXPR))
        } else {
            var_name(p);
            p.at(T!['(']);
            pure_expr_list(p);
            p.at(T![')']);
            Some(m.complete(p, SYNC_CALL_EXPR))
        }
    } else {
        Some(lhs)
    }
}

fn async_call_await(p: &mut Parser, m: Marker) -> Option<CompletedMarker> {
    p.bump(T![await]);
    pure_expr(p);
    p.expect(T![!]);
    var_name(p);
    p.at(T!['(']);
    pure_expr_list(p);
    p.at(T![')']);
    Some(m.complete(p, ASYNC_CALL_EXPR))
}

#[inline]
pub(super) fn pure_expr(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    pure_expr_continue(p, m, 1)
}

fn current_op(p: &Parser) -> (u8, SyntaxKind) {
    const NOT_AN_OP: (u8, SyntaxKind) = (0, T![.]);
    match p.current() {
        T![%] => (1, T![%]),
        T![/] => (1, T![/]),
        T![*] => (1, T![*]),
        T![-] => (2, T![-]),
        T![+] => (2, T![+]),
        T![>] if p.at(T![>=]) => (3, T![>=]),
        T![>] => (3, T![>]),
        T![<] if p.at(T![<=]) => (3, T![<=]),
        T![<] => (3, T![<]),
        T![!] if p.at(T![!=]) => (4, T![!=]),
        T![=] if p.at(T![==]) => (4, T![==]),
        T![&] if p.at(T![&&]) => (5, T![&&]),
        T![|] if p.at(T![||]) => (6, T![||]),
        T![implements] => (7, T![implements]),
        T![as] => (7, T![as]),
        _ => NOT_AN_OP,
    }
}

pub(super) fn pure_expr_continue(p: &mut Parser, m: Marker, bp: u8) -> Option<CompletedMarker> {
    let mut lhs = match p.current() {
        T![-] | T![!] => unary(p, m)?,
        T![this] => {
            p.bump(T![this]);
            if p.eat(T![.]) {
                var_name(p);
                m.complete(p, NAME_EXPR)
            } else {
                m.complete(p, THIS_EXPR)
            }
        }
        T![null] => {
            p.bump(T![null]);
            m.complete(p, NULL_EXPR)
        }
        c @ T![when] | c @ T![if] => {
            p.bump(c);
            pure_expr(p);
            p.expect(T![then]);
            pure_expr(p);
            p.expect(T![else]);
            pure_expr(p);
            m.complete(p, WHEN_EXPR)
        }
        T![case] => {
            p.bump(T![case]);
            pure_expr(p);
            case_expr_branches(p);
            m.complete(p, CASE_EXPR)
        }
        T![let] => {
            p.bump(T![let]);
            let_defs(p);
            p.expect(T![in]);
            pure_expr(p);
            m.complete(p, LET_EXPR)
        }
        T!['('] => {
            p.bump(T!['(']);
            pure_expr(p);
            p.expect(T![')']);
            m.complete(p, PAREN_EXPR)
        }
        c @ INT_NUMBER | c @ FLOAT_NUMBER | c @ STRING => {
            p.bump(c);
            m.complete(p, LITERAL)
        }
        _ => return None,
    };
    loop {
        let (op_bp, op) = current_op(p);
        if op_bp < bp {
            break;
        }
        if p.at(T![implements]) {
            lhs = implements_expr(p, lhs);
            continue;
        }
        if p.at(T![as]) {
            lhs = as_expr(p, lhs);
            continue;
        }
        let m = lhs.precede(p);
        p.bump(op);
        let ma = p.start();
        pure_expr_continue(p, ma, op_bp + 1);
        lhs = m.complete(p, BINARY_EXPR);
    }
    Some(lhs)
}

fn implements_expr(p: &mut Parser, lhs: CompletedMarker) -> CompletedMarker {
    let m = lhs.precede(p);
    p.bump(T![implements]);
    ty_name(p);
    m.complete(p, IMPLEMENTS_EXPR)
}

fn as_expr(p: &mut Parser, lhs: CompletedMarker) -> CompletedMarker {
    let m = lhs.precede(p);
    p.bump(T![as]);
    ty_name(p);
    m.complete(p, AS_EXPR)
}

fn unary(p: &mut Parser, m: Marker) -> Option<CompletedMarker> {
    if p.at(T![-]) {
        p.bump(T![-])
    } else if p.at(T![!]) {
        p.bump(T![!]);
    }
    pure_expr(p);
    Some(m.complete(p, UNARY_EXPR))
}

pub(super) fn eff_expr(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    match p.current() {
        T![new] => new_expr(p, m),
        T![await] => async_call_await(p, m),
        T![core] | T![original] => Some(original_call(p, m)),
        INT_NUMBER if p.nth_at(1, T![.]) => Some(original_call(p, m)),
        _ => call_or_get(p, m),
    }
}

fn new_expr(p: &mut Parser, m: Marker) -> Option<CompletedMarker> {
    p.bump(T![new]);
    p.eat(T![local]);
    ty_path(p);
    p.expect(T!['(']);
    pure_expr_list(p);
    p.expect(T![')']);
    Some(m.complete(p, NEW_EXPR))
}

pub(crate) fn name_expr(p: &mut Parser) {
    let m = p.start();
    if p.eat(T![this]) {
        p.expect(T![.]);
    }
    var_name(p);
    m.complete(p, NAME_EXPR);
}

pub(crate) fn pure_expr_list(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) && !p.at(T!['(']) {
        pure_expr(p);
    }
    p.expect(T![')']);
    m.complete(p, PURE_EXPR_LIST);
}

fn original_call(p: &mut Parser, m: Marker) -> CompletedMarker {
    if p.at(T![core]) || p.at(INT_NUMBER) {
        p.expect(T![.]);
    }
    p.expect(T![original]);
    p.at(T!['(']);
    pure_expr_list(p);
    p.at(T![')']);
    m.complete(p, ORIGINAL_CALL_EXPR)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum CaseExprBranchMode {
    Pipe,
    Semi,
}

fn case_expr_branches(p: &mut Parser) {
    // First branch
    let m = p.start();
    pattern(p);
    p.expect(T![=>]);
    pure_expr(p);
    let mode = match p.current() {
        T![|] => CaseExprBranchMode::Pipe,
        T![;] => CaseExprBranchMode::Semi,
        T!['}'] => {
            m.complete(p, CASE_EXPR_BRANCH);
            return;
        }
        _ => {
            p.error("expected |, ;, or {");
            return;
        }
    };
    if let CaseExprBranchMode::Semi = mode {
        p.bump(T![;]);
    }
    m.complete(p, CASE_EXPR_BRANCH);
    while !p.at(EOF) && !p.at(T!['}']) {
        case_expr_branch(p, mode)
    }
}

fn case_expr_branch(p: &mut Parser, mode: CaseExprBranchMode) {
    let m = p.start();
    if let CaseExprBranchMode::Pipe = mode {
        p.expect(T![|]);
    }
    pattern(p);
    p.expect(T![=>]);
    pure_expr(p);
    if let CaseExprBranchMode::Semi = mode {
        p.expect(T![;]);
    }
    m.complete(p, CASE_EXPR_BRANCH);
}

fn let_defs(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    let_def(p);
    while !p.at(EOF) && !p.at(T![in]) {
        let_def(p);
    }
    m.complete(p, LET_DEFS)
}

fn let_def(p: &mut Parser) {
    let m = p.start();
    let paren = p.eat(T!['(']);
    ty(p);
    var_name(p);
    p.expect(T![=]);
    pure_expr(p);
    if paren {
        p.expect(T![')']);
    }
    m.complete(p, LET_DEF);
}
