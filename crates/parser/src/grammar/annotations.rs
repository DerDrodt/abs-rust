use super::{pure_expr, ty, Parser, SyntaxKind::*, T};

pub(crate) fn annotations_opt(p: &mut Parser) {
    while p.at(T!['[']) {
        annotations(p);
    }
}

fn annotations(p: &mut Parser) {
    let m = p.start();
    p.bump(T!['[']);
    annotation(p);
    while p.eat(T![,]) {
        annotation(p);
    }
    p.expect(T![']']);
    m.complete(p, ANNOTATIONS);
}

fn annotation(p: &mut Parser) {
    let m = p.start();

    // Both types and exprs can start with annotations
    if p.at(T!['[']) {
        annotations(p);
    }

    // Could be type or expression
    if p.at(CAP_IDENT) && (p.nth_at(1, DOT) || p.nth_at(1, COLON)) {
        ty(p);
        p.expect(COLON);
    }
    pure_expr(p);
    m.complete(p, ANNOTATION);
}
