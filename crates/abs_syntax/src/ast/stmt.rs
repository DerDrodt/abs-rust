use std::fmt;

use super::{CaseBranch, Expr, Guard, Ident, PureExpr, Type};

pub enum Stmt {
    Skip,
    VarDecl(VarDeclStmt),
    Assign(AssignStmt),
    Expr(ExprStmt),
    Assert(AssertStmt),
    Await(AwaitStmt),
    Suspend,
    Throw(ThrowStmt),
    Return(ReturnStmt),
    Block(Block),
    If(IfStmt),
    Switch(SwitchStmt),
    While(WhileStmt),
    Foreach(ForeachStmt),
    TryCatchFinally(TryCatchFinallyStmt),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Skip => fmt::Display::fmt("skip;", f),
            Stmt::VarDecl(s) => fmt::Display::fmt(s, f),
            Stmt::Assign(s) => fmt::Display::fmt(s, f),
            Stmt::Expr(s) => fmt::Display::fmt(s, f),
            Stmt::Assert(s) => fmt::Display::fmt(s, f),
            Stmt::Await(s) => fmt::Display::fmt(s, f),
            Stmt::Suspend => fmt::Display::fmt("suspend;", f),
            Stmt::Throw(s) => fmt::Display::fmt(s, f),
            Stmt::Return(s) => fmt::Display::fmt(s, f),
            Stmt::Block(s) => fmt::Display::fmt(s, f),
            Stmt::If(s) => fmt::Display::fmt(s, f),
            Stmt::Switch(s) => fmt::Display::fmt(s, f),
            Stmt::While(s) => fmt::Display::fmt(s, f),
            Stmt::Foreach(s) => fmt::Display::fmt(s, f),
            Stmt::TryCatchFinally(s) => fmt::Display::fmt(s, f),
        }
    }
}

impl From<VarDeclStmt> for Stmt {
    fn from(s: VarDeclStmt) -> Self {
        Stmt::VarDecl(s)
    }
}

impl From<AssignStmt> for Stmt {
    fn from(s: AssignStmt) -> Self {
        Stmt::Assign(s)
    }
}

impl From<ExprStmt> for Stmt {
    fn from(s: ExprStmt) -> Self {
        Stmt::Expr(s)
    }
}

impl From<IfStmt> for Stmt {
    fn from(s: IfStmt) -> Self {
        Stmt::If(s)
    }
}

impl From<WhileStmt> for Stmt {
    fn from(s: WhileStmt) -> Self {
        Stmt::While(s)
    }
}

impl From<Block> for Stmt {
    fn from(s: Block) -> Self {
        Stmt::Block(s)
    }
}

impl From<ReturnStmt> for Stmt {
    fn from(s: ReturnStmt) -> Self {
        Stmt::Return(s)
    }
}

pub struct VarDeclStmt {
    pub ty: Type,
    pub ident: Ident,
    pub init: Option<Expr>,
}

impl fmt::Display for VarDeclStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let init = match &self.init {
            Some(e) => format!(" = {}", e),
            None => String::new(),
        };
        write!(f, "{} {}{};", self.ty, self.ident, init)
    }
}

pub struct AssignStmt {
    pub this: bool,
    pub ident: Ident,
    pub expr: Expr,
}

impl fmt::Display for AssignStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.this {
            write!(f, "this.{} = {};", self.ident, self.expr)
        } else {
            write!(f, "{} = {};", self.ident, self.expr)
        }
    }
}

pub struct ExprStmt {
    pub expr: Expr,
}

impl fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};", self.expr)
    }
}

pub struct AssertStmt {
    pub condition: PureExpr,
}

impl fmt::Display for AssertStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "assert {};", self.condition)
    }
}

pub struct AwaitStmt {
    pub guard: Guard,
}

impl fmt::Display for AwaitStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "await {};", self.guard)
    }
}

pub struct ReturnStmt {
    pub expr: Expr,
}

impl fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.expr)
    }
}

pub struct ThrowStmt {
    pub expr: PureExpr,
}

impl fmt::Display for ThrowStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "throw {};", self.expr)
    }
}

pub struct IfStmt {
    pub condition: PureExpr,
    pub then: Box<Stmt>,
    pub r#else: Option<Box<Stmt>>,
}

impl fmt::Display for IfStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r#else = match &self.r#else {
            Some(s) => format!(" else {}", s),
            _ => String::new(),
        };
        write!(f, "if ({}) {}{}", self.condition, self.then, r#else)
    }
}

pub struct SwitchStmt {
    pub expr: PureExpr,
    pub branches: Vec<CaseBranch<Stmt>>,
}

impl fmt::Display for SwitchStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut branches = String::new();

        for (i, b) in self.branches.iter().enumerate() {
            if i > 0 {
                branches.push_str("\n")
            }
            branches.push('\t');
            branches.push_str(&b.to_string());
        }

        write!(f, "switch {} {{{}}}", self.expr, branches)
    }
}

pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stmts = String::new();

        for (i, s) in self.stmts.iter().enumerate() {
            if i > 0 {
                stmts.push_str("\n")
            }
            stmts.push('\t');
            stmts.push_str(&s.to_string());
        }

        write!(f, "{{{}}}", stmts)
    }
}

pub struct WhileStmt {
    pub condition: PureExpr,
    pub body: Box<Stmt>,
}

impl fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while ({}) {}", self.condition, self.body)
    }
}

pub struct ForeachStmt {
    pub loop_var: Ident,
    pub iter: PureExpr,
    pub body: Box<Stmt>,
}

impl fmt::Display for ForeachStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "foreach ({} in {}) {}",
            self.loop_var, self.iter, self.body
        )
    }
}

pub struct TryCatchFinallyStmt {
    pub r#try: Box<Stmt>,
    pub catch_branches: Vec<CaseBranch<Stmt>>,
    pub finally: Option<Box<Stmt>>,
}

impl fmt::Display for TryCatchFinallyStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut catch_branches = String::new();

        for (i, b) in self.catch_branches.iter().enumerate() {
            if i > 0 {
                catch_branches.push_str("\n")
            }
            catch_branches.push('\t');
            catch_branches.push_str(&b.to_string());
        }

        let finally = match &self.finally {
            Some(s) => format!(" finally {}", s),
            _ => String::new(),
        };

        write!(
            f,
            "try {} catch {{{}}}{}",
            self.r#try, catch_branches, finally
        )
    }
}
