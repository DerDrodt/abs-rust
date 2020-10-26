use std::fmt;

use super::{Ident, PureExpr};

pub enum Guard {
    Claim { this: bool, ident: Ident },
    Expr(PureExpr),
    And(Box<Guard>, Box<Guard>),
    Duration(PureExpr, PureExpr),
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Guard::Claim { this, ident } => {
                if *this {
                    write!(f, "this.{}?", ident)
                } else {
                    write!(f, "{}?", ident)
                }
            }
            Guard::Expr(e) => fmt::Display::fmt(e, f),
            Guard::And(l, r) => write!(f, "{} & {}", l, r),
            Guard::Duration(min, max) => write!(f, "duration({},{})", min, max),
        }
    }
}
