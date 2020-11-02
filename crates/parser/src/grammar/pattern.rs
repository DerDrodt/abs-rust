use crate::parser::Marker;

use super::{fragments::TokenSet, ty_path, var_name, Parser, SyntaxKind::*, T};

pub(crate) fn pattern(p: &mut Parser) {
    pattern_r(p, PAT_RECOVERY_SET)
}

const PAT_RECOVERY_SET: TokenSet = TokenSet::new(&[CASE_KW, SWITCH_KW, R_PAREN, COMMA]);

fn pattern_r(p: &mut Parser, recovery: TokenSet) {
    let m = p.start();
    let kind = match p.current() {
        T![_] => {
            p.bump_any();
            WILD_CARD_PATTERN
        }
        INT_NUMBER => {
            p.bump_any();
            INT_PATTERN
        }
        STRING => {
            p.bump_any();
            STRING_PATTERN
        }
        LOW_IDENT => {
            var_name(p);
            VAR_PATTERN
        }
        CAP_IDENT => {
            return constr_pattern(p, m);
        }
        _ => {
            p.err_recover("expected pattern", recovery);
            return;
        }
    };
    m.complete(p, kind);
}

fn constr_pattern(p: &mut Parser, m: Marker) {
    ty_path(p);
    if p.eat(T!['(']) && !p.eat(T![')']) {
        pattern(p);
        while p.eat(T![,]) {
            pattern(p);
        }
        p.expect(T![')']);
    }
    m.complete(p, CONSTRUCTOR_PATTERN);
}
