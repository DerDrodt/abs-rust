use super::{pattern, pure_expr, stmts::stmt, Parser};
use crate::{SyntaxKind::*, T};

pub(crate) fn case_stmt_branch(p: &mut Parser) {
    let m = p.start();
    pattern(p);
    p.expect(T![=>]);
    stmt(p);
    m.complete(p, CASE_STMT_BRANCH);
}

pub(crate) fn case_expr_branch(p: &mut Parser) {
    let m = p.start();
    pattern(p);
    p.expect(T![=>]);
    pure_expr(p);
    m.complete(p, CASE_EXPR_BRANCH);
}
