use std::fmt;

use super::{CaseBranch, Ident, Literal, Type};
pub enum Expr {
    Pure(PureExpr),
    Eff(EffExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Pure(e) => fmt::Display::fmt(e, f),
            Expr::Eff(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl<E: Into<PureExpr>> From<E> for Expr {
    fn from(e: E) -> Self {
        let e: PureExpr = e.into();
        Expr::Pure(e)
    }
}

impl From<EffExpr> for Expr {
    fn from(e: EffExpr) -> Self {
        Expr::Eff(e)
    }
}

pub enum PureExpr {
    Ident(IdentExpr),
    ThisIdent(IdentExpr),
    This,
    Null,
    Literal(Literal),
    TemplateString,
    Let(LetExpr),
    DataConstr(DataConstrExpr),
    FnApp(FnAppExpr),
    ParFnApp(ParFnAppExpr),
    When(WhenExpr),
    Case(CaseExpr),
    Operator(OperatorExpr),
    TypeCheck(TypeCheckExpr),
    TypeCast(TypeCastExpr),
}

impl fmt::Display for PureExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PureExpr::Ident(i) => fmt::Display::fmt(i, f),
            PureExpr::ThisIdent(i) => write!(f, "this.{}", i),
            PureExpr::This => write!(f, "this"),
            PureExpr::Literal(i) => fmt::Display::fmt(i, f),
            PureExpr::TemplateString => todo!(),
            PureExpr::Let(i) => fmt::Display::fmt(i, f),
            PureExpr::DataConstr(i) => fmt::Display::fmt(i, f),
            PureExpr::FnApp(i) => fmt::Display::fmt(i, f),
            PureExpr::ParFnApp(i) => fmt::Display::fmt(i, f),
            PureExpr::When(i) => fmt::Display::fmt(i, f),
            PureExpr::Case(i) => fmt::Display::fmt(i, f),
            PureExpr::Operator(i) => fmt::Display::fmt(i, f),
            PureExpr::TypeCheck(i) => fmt::Display::fmt(i, f),
            PureExpr::TypeCast(i) => fmt::Display::fmt(i, f),
            PureExpr::Null => write!(f, "null"),
        }
    }
}

impl From<Literal> for PureExpr {
    fn from(e: Literal) -> Self {
        PureExpr::Literal(e)
    }
}

impl From<IdentExpr> for PureExpr {
    fn from(e: IdentExpr) -> Self {
        PureExpr::Ident(e)
    }
}

pub struct IdentExpr {
    pub ident: Ident,
}

impl fmt::Display for IdentExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.ident, f)
    }
}

pub struct LetExpr {
    pub ty: Type,
    pub ident: Ident,
    pub value: Box<PureExpr>,
    pub inner: Box<PureExpr>,
}

impl fmt::Display for LetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "let {} {} = {}\n\tin {}",
            self.ty, self.ident, self.value, self.inner
        )
    }
}

pub struct DataConstrExpr {
    pub ident: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for DataConstrExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = String::new();

        for (i, a) in self.args.iter().enumerate() {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&a.to_string());
        }

        write!(f, "{}({})", self.ident, args)
    }
}

pub struct FnAppExpr {
    pub ident: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for FnAppExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = String::new();

        for (i, a) in self.args.iter().enumerate() {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&a.to_string());
        }

        write!(f, "{}({})", self.ident, args)
    }
}

pub struct ParFnAppExpr {}

impl fmt::Display for ParFnAppExpr {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub struct WhenExpr {
    pub condition: Box<PureExpr>,
    pub then: Box<PureExpr>,
    pub r#else: Box<PureExpr>,
}

impl fmt::Display for WhenExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "when {} then {} else {}",
            self.condition, self.then, self.r#else
        )
    }
}

pub struct CaseExpr {
    pub expr: Box<PureExpr>,
    pub branches: Vec<CaseBranch<PureExpr>>,
}

impl fmt::Display for CaseExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut branches = String::new();

        for (i, b) in self.branches.iter().enumerate() {
            if i > 0 {
                branches.push_str("| ");
            }
            branches.push_str(&b.to_string());
        }

        write!(f, "case {} {{{}}}", self.expr, branches)
    }
}

pub struct TypeCheckExpr {
    pub expr: Box<PureExpr>,
    pub ty: Ident,
}

impl fmt::Display for TypeCheckExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} implements {}", self.expr, self.ty)
    }
}

pub struct TypeCastExpr {
    pub expr: Box<PureExpr>,
    pub ty: Ident,
}

impl fmt::Display for TypeCastExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} as {}", self.expr, self.ty)
    }
}

pub enum OperatorExpr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl fmt::Display for OperatorExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatorExpr::Unary(u) => fmt::Display::fmt(u, f),
            OperatorExpr::Binary(b) => fmt::Display::fmt(b, f),
        }
    }
}

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<PureExpr>,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.op, self.expr)
    }
}

pub enum UnaryOp {
    Not,
    Minus,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::Minus => write!(f, "-"),
        }
    }
}

pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Box<PureExpr>,
    pub right: Box<PureExpr>,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

pub enum BinaryOp {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Or => "||",
            BinaryOp::And => "&&",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Mult => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
        };
        fmt::Display::fmt(s, f)
    }
}

pub enum EffExpr {
    New(NewExpr),
    SyncCall(SyncCallExpr),
    AsyncCall(AsyncCallExpr),
    Get(GetExpr),
    Await(AwaitExpr),
}

impl fmt::Display for EffExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffExpr::New(e) => fmt::Display::fmt(e, f),
            EffExpr::SyncCall(e) => fmt::Display::fmt(e, f),
            EffExpr::AsyncCall(e) => fmt::Display::fmt(e, f),
            EffExpr::Get(e) => fmt::Display::fmt(e, f),
            EffExpr::Await(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<NewExpr> for EffExpr {
    fn from(e: NewExpr) -> Self {
        EffExpr::New(e)
    }
}

pub struct NewExpr {
    pub local: bool,
    pub ty: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for NewExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let local = if self.local { "local " } else { "" };
        let mut args = String::new();

        for (i, a) in self.args.iter().enumerate() {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&a.to_string());
        }

        write!(f, "new {} {}({})", local, self.ty, args)
    }
}

pub struct SyncCallExpr {
    pub callee: PureExpr,
    pub method: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for SyncCallExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = String::new();

        for (i, a) in self.args.iter().enumerate() {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&a.to_string());
        }
        write!(f, "{}.{}({})", self.callee, self.method, args)
    }
}

pub struct AsyncCallExpr {
    pub callee: PureExpr,
    pub method: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for AsyncCallExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = String::new();

        for (i, a) in self.args.iter().enumerate() {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&a.to_string());
        }
        write!(f, "{}!{}({})", self.callee, self.method, args)
    }
}

pub struct GetExpr {
    pub expr: PureExpr,
}

impl fmt::Display for GetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.get", self.expr)
    }
}

pub struct AwaitExpr {
    pub call: AsyncCallExpr,
}

impl fmt::Display for AwaitExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "await {}", self.call)
    }
}
