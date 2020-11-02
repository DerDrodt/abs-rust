//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarName {
    pub(crate) syntax: SyntaxNode,
}
impl VarName {
    pub fn lower_ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lower_ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeName {
    pub(crate) syntax: SyntaxNode,
}
impl TypeName {
    pub fn cap_ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![cap_ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}
impl Path {
    pub fn qualifier(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
    pub fn segment(&self) -> Option<VarName> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePath {
    pub(crate) syntax: SyntaxNode,
}
impl TypePath {
    pub fn qualifier(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
    pub fn segment(&self) -> Option<TypeName> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: SyntaxNode,
}
impl ParamList {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn params(&self) -> AstChildren<Param> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for Param {}
impl Param {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotations {
    pub(crate) syntax: SyntaxNode,
}
impl Annotations {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.syntax)
    }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type {
    pub(crate) syntax: SyntaxNode,
}
impl Type {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![<])
    }
    pub fn types(&self) -> AstChildren<Type> {
        support::children(&self.syntax)
    }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![>])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotation {
    pub(crate) syntax: SyntaxNode,
}
impl Annotation {
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::PureExprListOwner for FnExpr {}
impl FnExpr {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartialFnExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::PureExprListOwner for PartialFnExpr {}
impl PartialFnExpr {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn fn_list(&self) -> Option<FnList> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariadicFnExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::PureExprListOwner for VariadicFnExpr {}
impl VariadicFnExpr {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::PureExprListOwner for ConstructorExpr {}
impl ConstructorExpr {
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl UnaryExpr {
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl BinaryExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for NameExpr {}
impl NameExpr {
    pub fn this_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![this])
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThisExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ThisExpr {
    pub fn this_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![this])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullExpr {
    pub(crate) syntax: SyntaxNode,
}
impl NullExpr {
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![null])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementsExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::TypeNameOwner for ImplementsExpr {}
impl ImplementsExpr {
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn implements_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![implements])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::TypeNameOwner for AsExpr {}
impl AsExpr {
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![as])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl WhenExpr {
    pub fn when_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![when])
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![if])
    }
    pub fn condition(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn then_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![then])
    }
    pub fn else_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![else])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseExpr {
    pub(crate) syntax: SyntaxNode,
}
impl CaseExpr {
    pub fn case_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![case])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn case_expr_branch(&self) -> Option<CaseExprBranch> {
        support::child(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetExpr {
    pub(crate) syntax: SyntaxNode,
}
impl LetExpr {
    pub fn let_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![let])
    }
    pub fn let_defs(&self) -> Option<LetDefs> {
        support::child(&self.syntax)
    }
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![in])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ParenExpr {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PureExprList {
    pub(crate) syntax: SyntaxNode,
}
impl PureExprList {
    pub fn pure_exprs(&self) -> AstChildren<PureExpr> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnList {
    pub(crate) syntax: SyntaxNode,
}
impl FnList {
    pub fn fn_list_param(&self) -> Option<FnListParam> {
        support::child(&self.syntax)
    }
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![,])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseExprBranch {
    pub(crate) syntax: SyntaxNode,
}
impl CaseExprBranch {
    pub fn pipe_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![|])
    }
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }
    pub fn fat_arrow_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=>])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetDefs {
    pub(crate) syntax: SyntaxNode,
}
impl LetDefs {
    pub fn let_def(&self) -> Option<LetDef> {
        support::child(&self.syntax)
    }
    pub fn let_defs(&self) -> AstChildren<LetDef> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnonFn {
    pub(crate) syntax: SyntaxNode,
}
impl AnonFn {
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    pub fn fat_arrow_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=>])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetDef {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for LetDef {}
impl LetDef {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetExpr {
    pub(crate) syntax: SyntaxNode,
}
impl GetExpr {
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
    pub fn get_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![get])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::PureExprListOwner for NewExpr {}
impl NewExpr {
    pub fn new_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![new])
    }
    pub fn local_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![local])
    }
    pub fn class(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for AsyncCallExpr {}
impl ast::PureExprListOwner for AsyncCallExpr {}
impl AsyncCallExpr {
    pub fn await_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![await])
    }
    pub fn callee(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn excl_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![!])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyncCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for SyncCallExpr {}
impl ast::PureExprListOwner for SyncCallExpr {}
impl SyncCallExpr {
    pub fn callee(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OriginalCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::PureExprListOwner for OriginalCallExpr {}
impl OriginalCallExpr {
    pub fn core_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![core])
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
    pub fn original_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![original])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarDeclStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for VarDeclStmt {}
impl VarDeclStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AssignStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn name_expr(&self) -> Option<NameExpr> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkipStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SkipStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn skip_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![skip])
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ReturnStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn return_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![return])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssertStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AssertStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn assert_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![assert])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub(crate) syntax: SyntaxNode,
}
impl Block {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn stmts(&self) -> AstChildren<Stmt> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStmt {
    pub(crate) syntax: SyntaxNode,
}
impl IfStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![if])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn condition(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn else_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![else])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SwitchStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn switch_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![switch])
    }
    pub fn case_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![case])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn case_stmt_branchs(&self) -> AstChildren<CaseStmtBranch> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileStmt {
    pub(crate) syntax: SyntaxNode,
}
impl WhileStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn while_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![while])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn condition(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn body(&self) -> Option<Stmt> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeachStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ForeachStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn foreach_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![foreach])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![,])
    }
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![in])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn stmt(&self) -> Option<Stmt> {
        support::child(&self.syntax)
    }
    pub fn any_name(&self) -> Option<AnyName> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryCatchFinallyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl TryCatchFinallyStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn try_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![try])
    }
    pub fn stmt(&self) -> Option<Stmt> {
        support::child(&self.syntax)
    }
    pub fn catch_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![catch])
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn case_stmt_branchs(&self) -> AstChildren<CaseStmtBranch> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
    pub fn case_stmt_branch(&self) -> Option<CaseStmtBranch> {
        support::child(&self.syntax)
    }
    pub fn finally_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![finally])
    }
    pub fn finally(&self) -> Option<Stmt> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AwaitStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn await_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![await])
    }
    pub fn guard(&self) -> Option<Guard> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuspendStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SuspendStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn suspend_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![suspend])
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DurationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DurationStmt {
    pub fn duration_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![duration])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![,])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ThrowStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn throw_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![throw])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DieStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DieStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn die_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![die])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveCogToStmt {
    pub(crate) syntax: SyntaxNode,
}
impl MoveCogToStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn movecogto_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![movecogto])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ExprStmt {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseStmtBranch {
    pub(crate) syntax: SyntaxNode,
}
impl CaseStmtBranch {
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }
    pub fn fat_arrow_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=>])
    }
    pub fn stmt(&self) -> Option<Stmt> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClaimGuard {
    pub(crate) syntax: SyntaxNode,
}
impl ClaimGuard {
    pub fn name_expr(&self) -> Option<NameExpr> {
        support::child(&self.syntax)
    }
    pub fn question_mark_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![?])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DurationGuard {
    pub(crate) syntax: SyntaxNode,
}
impl DurationGuard {
    pub fn duration_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![duration])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![,])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprGuard {
    pub(crate) syntax: SyntaxNode,
}
impl ExprGuard {
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AndGuard {
    pub(crate) syntax: SyntaxNode,
}
impl AndGuard {
    pub fn left(&self) -> Option<Guard> {
        support::child(&self.syntax)
    }
    pub fn amp_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![&])
    }
    pub fn right(&self) -> Option<Guard> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WildCardPattern {
    pub(crate) syntax: SyntaxNode,
}
impl WildCardPattern {
    pub fn underscore_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![_])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntPattern {
    pub(crate) syntax: SyntaxNode,
}
impl IntPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringPattern {
    pub(crate) syntax: SyntaxNode,
}
impl StringPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarPattern {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for VarPattern {}
impl VarPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl ConstructorPattern {
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn patterns(&self) -> AstChildren<Pattern> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataTypeDecl {
    pub(crate) syntax: SyntaxNode,
}
impl DataTypeDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn data_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![data])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn generic_arg_list(&self) -> Option<GenericArgList> {
        support::child(&self.syntax)
    }
    pub fn data_constructor_list(&self) -> Option<DataConstructorList> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericArgList {
    pub(crate) syntax: SyntaxNode,
}
impl GenericArgList {
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![<])
    }
    pub fn type_names(&self) -> AstChildren<TypeName> {
        support::children(&self.syntax)
    }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![>])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataConstructorList {
    pub(crate) syntax: SyntaxNode,
}
impl DataConstructorList {
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn data_constructors(&self) -> AstChildren<DataConstructor> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataConstructor {
    pub(crate) syntax: SyntaxNode,
}
impl ast::TypeNameOwner for DataConstructor {}
impl DataConstructor {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn data_constructor_arg_list(&self) -> Option<DataConstructorArgList> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataConstructorArgList {
    pub(crate) syntax: SyntaxNode,
}
impl DataConstructorArgList {
    pub fn data_constructor_args(&self) -> AstChildren<DataConstructorArg> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataConstructorArg {
    pub(crate) syntax: SyntaxNode,
}
impl ast::TypeNameOwner for DataConstructorArg {}
impl DataConstructorArg {
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeSyn {
    pub(crate) syntax: SyntaxNode,
}
impl TypeSyn {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![type])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExceptionDecl {
    pub(crate) syntax: SyntaxNode,
}
impl ExceptionDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn exception_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![exception])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn data_constructor_arg_list(&self) -> Option<DataConstructorArgList> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDecl {
    pub(crate) syntax: SyntaxNode,
}
impl FunctionDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn def_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![def])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn generic_arg_list(&self) -> Option<GenericArgList> {
        support::child(&self.syntax)
    }
    pub fn params(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn builtin_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![builtin])
    }
    pub fn body(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParFunctionDecl {
    pub(crate) syntax: SyntaxNode,
}
impl ParFunctionDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn def_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![def])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn generic_arg_list(&self) -> Option<GenericArgList> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn functions(&self) -> Option<FunctionNameList> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn params(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn body(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionNameList {
    pub(crate) syntax: SyntaxNode,
}
impl FunctionNameList {
    pub fn var_names(&self) -> AstChildren<VarName> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceDecl {
    pub(crate) syntax: SyntaxNode,
}
impl InterfaceDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn interface_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![interface])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn extends(&self) -> Option<ExtendsList> {
        support::child(&self.syntax)
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn interface_items(&self) -> AstChildren<InterfaceItem> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtendsList {
    pub(crate) syntax: SyntaxNode,
}
impl ExtendsList {
    pub fn type_paths(&self) -> AstChildren<TypePath> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceItem {
    pub(crate) syntax: SyntaxNode,
}
impl InterfaceItem {
    pub fn method_sig(&self) -> Option<MethodSig> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodSig {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for MethodSig {}
impl MethodSig {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodDecl {
    pub(crate) syntax: SyntaxNode,
}
impl MethodDecl {
    pub fn method_sig(&self) -> Option<MethodSig> {
        support::child(&self.syntax)
    }
    pub fn block(&self) -> Option<Block> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassDecl {
    pub(crate) syntax: SyntaxNode,
}
impl ClassDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn class_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![class])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    pub fn implements_list(&self) -> Option<ImplementsList> {
        support::child(&self.syntax)
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn field_decls(&self) -> AstChildren<FieldDecl> {
        support::children(&self.syntax)
    }
    pub fn init(&self) -> Option<Block> {
        support::child(&self.syntax)
    }
    pub fn recover_block(&self) -> Option<RecoverBlock> {
        support::child(&self.syntax)
    }
    pub fn trait_uses(&self) -> AstChildren<TraitUse> {
        support::children(&self.syntax)
    }
    pub fn method_decls(&self) -> AstChildren<MethodDecl> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementsList {
    pub(crate) syntax: SyntaxNode,
}
impl ImplementsList {
    pub fn implements_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![implements])
    }
    pub fn type_paths(&self) -> AstChildren<TypePath> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldDecl {
    pub(crate) syntax: SyntaxNode,
}
impl ast::VarNameOwner for FieldDecl {}
impl FieldDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn pure_expr(&self) -> Option<PureExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecoverBlock {
    pub(crate) syntax: SyntaxNode,
}
impl RecoverBlock {
    pub fn recover_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![recover])
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn case_stmt_branchs(&self) -> AstChildren<CaseStmtBranch> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitUse {
    pub(crate) syntax: SyntaxNode,
}
impl TraitUse {
    pub fn uses_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![uses])
    }
    pub fn trait_expr(&self) -> Option<TraitExpr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub(crate) syntax: SyntaxNode,
}
impl Module {
    pub fn module_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![module])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
    pub fn imports(&self) -> AstChildren<Import> {
        support::children(&self.syntax)
    }
    pub fn exports(&self) -> AstChildren<Export> {
        support::children(&self.syntax)
    }
    pub fn decls(&self) -> AstChildren<Decl> {
        support::children(&self.syntax)
    }
    pub fn main(&self) -> Option<Block> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarImport {
    pub(crate) syntax: SyntaxNode,
}
impl StarImport {
    pub fn import_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![import])
    }
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![*])
    }
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![from])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromImport {
    pub(crate) syntax: SyntaxNode,
}
impl FromImport {
    pub fn import_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![import])
    }
    pub fn any_names(&self) -> AstChildren<AnyName> {
        support::children(&self.syntax)
    }
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![from])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnqualifiedImport {
    pub(crate) syntax: SyntaxNode,
}
impl UnqualifiedImport {
    pub fn import_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![import])
    }
    pub fn any_names(&self) -> AstChildren<AnyName> {
        support::children(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarExport {
    pub(crate) syntax: SyntaxNode,
}
impl StarExport {
    pub fn export_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![export])
    }
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![*])
    }
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![from])
    }
    pub fn from(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartialExport {
    pub(crate) syntax: SyntaxNode,
}
impl PartialExport {
    pub fn export_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![export])
    }
    pub fn any_names(&self) -> AstChildren<AnyName> {
        support::children(&self.syntax)
    }
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![from])
    }
    pub fn from(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitDecl {
    pub(crate) syntax: SyntaxNode,
}
impl TraitDecl {
    pub fn annotationss(&self) -> AstChildren<Annotations> {
        support::children(&self.syntax)
    }
    pub fn trait_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![trait])
    }
    pub fn type_path(&self) -> Option<TypePath> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn trait_expr(&self) -> Option<TraitExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitExpr {
    pub(crate) syntax: SyntaxNode,
}
impl TraitExpr {
    pub fn basic_trait_expr(&self) -> Option<BasicTraitExpr> {
        support::child(&self.syntax)
    }
    pub fn trait_ops(&self) -> AstChildren<TraitOp> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitMethodSet {
    pub(crate) syntax: SyntaxNode,
}
impl TraitMethodSet {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn method_decls(&self) -> AstChildren<MethodDecl> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitMethod {
    pub(crate) syntax: SyntaxNode,
}
impl TraitMethod {
    pub fn method_decl(&self) -> Option<MethodDecl> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitName {
    pub(crate) syntax: SyntaxNode,
}
impl ast::TypeNameOwner for TraitName {}
impl TraitName {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitRemoveSig {
    pub(crate) syntax: SyntaxNode,
}
impl TraitRemoveSig {
    pub fn removes_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![removes])
    }
    pub fn method_sigs(&self) -> AstChildren<MethodSig> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitRemoveSet {
    pub(crate) syntax: SyntaxNode,
}
impl TraitRemoveSet {
    pub fn removes_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![removes])
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn method_sigs(&self) -> AstChildren<MethodSig> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitAdd {
    pub(crate) syntax: SyntaxNode,
}
impl TraitAdd {
    pub fn adds_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![adds])
    }
    pub fn basic_trait_expr(&self) -> Option<BasicTraitExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitModifies {
    pub(crate) syntax: SyntaxNode,
}
impl TraitModifies {
    pub fn modifies_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![modifies])
    }
    pub fn basic_trait_expr(&self) -> Option<BasicTraitExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnyName {
    VarName(VarName),
    TypeName(TypeName),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PureExpr {
    FnExpr(FnExpr),
    PartialFnExpr(PartialFnExpr),
    VariadicFnExpr(VariadicFnExpr),
    ConstructorExpr(ConstructorExpr),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    NameExpr(NameExpr),
    Literal(Literal),
    ThisExpr(ThisExpr),
    NullExpr(NullExpr),
    ImplementsExpr(ImplementsExpr),
    AsExpr(AsExpr),
    WhenExpr(WhenExpr),
    CaseExpr(CaseExpr),
    LetExpr(LetExpr),
    ParenExpr(ParenExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    PureExpr(PureExpr),
    EffExpr(EffExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EffExpr {
    GetExpr(GetExpr),
    NewExpr(NewExpr),
    AsyncCallExpr(AsyncCallExpr),
    SyncCallExpr(SyncCallExpr),
    OriginalCallExpr(OriginalCallExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FnListParam {
    VarName(VarName),
    AnonFn(AnonFn),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    WildCardPattern(WildCardPattern),
    IntPattern(IntPattern),
    StringPattern(StringPattern),
    VarPattern(VarPattern),
    ConstructorPattern(ConstructorPattern),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    VarDeclStmt(VarDeclStmt),
    AssignStmt(AssignStmt),
    SkipStmt(SkipStmt),
    ReturnStmt(ReturnStmt),
    AssertStmt(AssertStmt),
    Block(Block),
    IfStmt(IfStmt),
    SwitchStmt(SwitchStmt),
    WhileStmt(WhileStmt),
    ForeachStmt(ForeachStmt),
    TryCatchFinallyStmt(TryCatchFinallyStmt),
    AwaitStmt(AwaitStmt),
    SuspendStmt(SuspendStmt),
    DurationStmt(DurationStmt),
    ThrowStmt(ThrowStmt),
    DieStmt(DieStmt),
    MoveCogToStmt(MoveCogToStmt),
    ExprStmt(ExprStmt),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Guard {
    ClaimGuard(ClaimGuard),
    DurationGuard(DurationGuard),
    ExprGuard(ExprGuard),
    AndGuard(AndGuard),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Import {
    StarImport(StarImport),
    FromImport(FromImport),
    UnqualifiedImport(UnqualifiedImport),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Export {
    StarExport(StarExport),
    PartialExport(PartialExport),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Decl {
    DataTypeDecl(DataTypeDecl),
    FunctionDecl(FunctionDecl),
    ParFunctionDecl(ParFunctionDecl),
    TypeSyn(TypeSyn),
    ExceptionDecl(ExceptionDecl),
    InterfaceDecl(InterfaceDecl),
    ClassDecl(ClassDecl),
    TraitDecl(TraitDecl),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BasicTraitExpr {
    TraitMethodSet(TraitMethodSet),
    TraitMethod(TraitMethod),
    TraitName(TraitName),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitOp {
    TraitRemoveSig(TraitRemoveSig),
    TraitRemoveSet(TraitRemoveSet),
    TraitAdd(TraitAdd),
    TraitModifies(TraitModifies),
}
impl AstNode for VarName {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VAR_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TypeName {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Path {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TypePath {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_PATH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Param {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Annotations {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ANNOTATIONS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Type {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Annotation {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ANNOTATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FnExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FN_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PartialFnExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARTIAL_FN_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for VariadicFnExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VARIADIC_FN_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstructorExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CONSTRUCTOR_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UnaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == UNARY_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BinaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BINARY_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NameExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAME_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ThisExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == THIS_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NullExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NULL_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ImplementsExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IMPLEMENTS_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AsExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == AS_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WhenExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == WHEN_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CaseExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CASE_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LetExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LET_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParenExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PAREN_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PureExprList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PURE_EXPR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FnList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FN_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CaseExprBranch {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CASE_EXPR_BRANCH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LetDefs {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LET_DEFS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AnonFn {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ANON_FN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LetDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LET_DEF
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for GetExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GET_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NewExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NEW_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AsyncCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASYNC_CALL_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SyncCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SYNC_CALL_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OriginalCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ORIGINAL_CALL_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for VarDeclStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VAR_DECL_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AssignStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASSIGN_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SkipStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SKIP_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReturnStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RETURN_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AssertStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASSERT_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Block {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IfStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IF_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SwitchStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SWITCH_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WhileStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == WHILE_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ForeachStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FOREACH_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TryCatchFinallyStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRY_CATCH_FINALLY_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AwaitStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == AWAIT_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SuspendStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SUSPEND_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DurationStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DURATION_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ThrowStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == THROW_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DieStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DIE_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MoveCogToStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MOVE_COG_TO_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExprStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXPR_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CaseStmtBranch {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CASE_STMT_BRANCH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ClaimGuard {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CLAIM_GUARD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DurationGuard {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DURATION_GUARD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExprGuard {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXPR_GUARD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AndGuard {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == AND_GUARD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WildCardPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == WILD_CARD_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IntPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INT_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for StringPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRING_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for VarPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VAR_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstructorPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CONSTRUCTOR_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DataTypeDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DATA_TYPE_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for GenericArgList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GENERIC_ARG_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DataConstructorList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DATA_CONSTRUCTOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DataConstructor {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DATA_CONSTRUCTOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DataConstructorArgList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DATA_CONSTRUCTOR_ARG_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DataConstructorArg {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DATA_CONSTRUCTOR_ARG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TypeSyn {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_SYN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExceptionDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXCEPTION_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FunctionDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FUNCTION_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParFunctionDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PAR_FUNCTION_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FunctionNameList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FUNCTION_NAME_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for InterfaceDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INTERFACE_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExtendsList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXTENDS_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for InterfaceItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INTERFACE_ITEM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MethodSig {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == METHOD_SIG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MethodDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == METHOD_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ClassDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CLASS_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ImplementsList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IMPLEMENTS_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FieldDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FIELD_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RecoverBlock {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RECOVER_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitUse {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_USE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Module {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MODULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for StarImport {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STAR_IMPORT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FromImport {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FROM_IMPORT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UnqualifiedImport {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == UNQUALIFIED_IMPORT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for StarExport {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STAR_EXPORT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PartialExport {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARTIAL_EXPORT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitDecl {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_DECL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitMethodSet {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_METHOD_SET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitMethod {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_METHOD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitName {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitRemoveSig {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_REMOVE_SIG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitRemoveSet {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_REMOVE_SET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitAdd {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_ADD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TraitModifies {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_MODIFIES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl From<VarName> for AnyName {
    fn from(node: VarName) -> AnyName {
        AnyName::VarName(node)
    }
}
impl From<TypeName> for AnyName {
    fn from(node: TypeName) -> AnyName {
        AnyName::TypeName(node)
    }
}
impl AstNode for AnyName {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            VAR_NAME | TYPE_NAME => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            VAR_NAME => AnyName::VarName(VarName { syntax }),
            TYPE_NAME => AnyName::TypeName(TypeName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyName::VarName(it) => &it.syntax,
            AnyName::TypeName(it) => &it.syntax,
        }
    }
}
impl From<FnExpr> for PureExpr {
    fn from(node: FnExpr) -> PureExpr {
        PureExpr::FnExpr(node)
    }
}
impl From<PartialFnExpr> for PureExpr {
    fn from(node: PartialFnExpr) -> PureExpr {
        PureExpr::PartialFnExpr(node)
    }
}
impl From<VariadicFnExpr> for PureExpr {
    fn from(node: VariadicFnExpr) -> PureExpr {
        PureExpr::VariadicFnExpr(node)
    }
}
impl From<ConstructorExpr> for PureExpr {
    fn from(node: ConstructorExpr) -> PureExpr {
        PureExpr::ConstructorExpr(node)
    }
}
impl From<UnaryExpr> for PureExpr {
    fn from(node: UnaryExpr) -> PureExpr {
        PureExpr::UnaryExpr(node)
    }
}
impl From<BinaryExpr> for PureExpr {
    fn from(node: BinaryExpr) -> PureExpr {
        PureExpr::BinaryExpr(node)
    }
}
impl From<NameExpr> for PureExpr {
    fn from(node: NameExpr) -> PureExpr {
        PureExpr::NameExpr(node)
    }
}
impl From<Literal> for PureExpr {
    fn from(node: Literal) -> PureExpr {
        PureExpr::Literal(node)
    }
}
impl From<ThisExpr> for PureExpr {
    fn from(node: ThisExpr) -> PureExpr {
        PureExpr::ThisExpr(node)
    }
}
impl From<NullExpr> for PureExpr {
    fn from(node: NullExpr) -> PureExpr {
        PureExpr::NullExpr(node)
    }
}
impl From<ImplementsExpr> for PureExpr {
    fn from(node: ImplementsExpr) -> PureExpr {
        PureExpr::ImplementsExpr(node)
    }
}
impl From<AsExpr> for PureExpr {
    fn from(node: AsExpr) -> PureExpr {
        PureExpr::AsExpr(node)
    }
}
impl From<WhenExpr> for PureExpr {
    fn from(node: WhenExpr) -> PureExpr {
        PureExpr::WhenExpr(node)
    }
}
impl From<CaseExpr> for PureExpr {
    fn from(node: CaseExpr) -> PureExpr {
        PureExpr::CaseExpr(node)
    }
}
impl From<LetExpr> for PureExpr {
    fn from(node: LetExpr) -> PureExpr {
        PureExpr::LetExpr(node)
    }
}
impl From<ParenExpr> for PureExpr {
    fn from(node: ParenExpr) -> PureExpr {
        PureExpr::ParenExpr(node)
    }
}
impl AstNode for PureExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FN_EXPR | PARTIAL_FN_EXPR | VARIADIC_FN_EXPR | CONSTRUCTOR_EXPR | UNARY_EXPR
            | BINARY_EXPR | NAME_EXPR | LITERAL | THIS_EXPR | NULL_EXPR | IMPLEMENTS_EXPR
            | AS_EXPR | WHEN_EXPR | CASE_EXPR | LET_EXPR | PAREN_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            FN_EXPR => PureExpr::FnExpr(FnExpr { syntax }),
            PARTIAL_FN_EXPR => PureExpr::PartialFnExpr(PartialFnExpr { syntax }),
            VARIADIC_FN_EXPR => PureExpr::VariadicFnExpr(VariadicFnExpr { syntax }),
            CONSTRUCTOR_EXPR => PureExpr::ConstructorExpr(ConstructorExpr { syntax }),
            UNARY_EXPR => PureExpr::UnaryExpr(UnaryExpr { syntax }),
            BINARY_EXPR => PureExpr::BinaryExpr(BinaryExpr { syntax }),
            NAME_EXPR => PureExpr::NameExpr(NameExpr { syntax }),
            LITERAL => PureExpr::Literal(Literal { syntax }),
            THIS_EXPR => PureExpr::ThisExpr(ThisExpr { syntax }),
            NULL_EXPR => PureExpr::NullExpr(NullExpr { syntax }),
            IMPLEMENTS_EXPR => PureExpr::ImplementsExpr(ImplementsExpr { syntax }),
            AS_EXPR => PureExpr::AsExpr(AsExpr { syntax }),
            WHEN_EXPR => PureExpr::WhenExpr(WhenExpr { syntax }),
            CASE_EXPR => PureExpr::CaseExpr(CaseExpr { syntax }),
            LET_EXPR => PureExpr::LetExpr(LetExpr { syntax }),
            PAREN_EXPR => PureExpr::ParenExpr(ParenExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            PureExpr::FnExpr(it) => &it.syntax,
            PureExpr::PartialFnExpr(it) => &it.syntax,
            PureExpr::VariadicFnExpr(it) => &it.syntax,
            PureExpr::ConstructorExpr(it) => &it.syntax,
            PureExpr::UnaryExpr(it) => &it.syntax,
            PureExpr::BinaryExpr(it) => &it.syntax,
            PureExpr::NameExpr(it) => &it.syntax,
            PureExpr::Literal(it) => &it.syntax,
            PureExpr::ThisExpr(it) => &it.syntax,
            PureExpr::NullExpr(it) => &it.syntax,
            PureExpr::ImplementsExpr(it) => &it.syntax,
            PureExpr::AsExpr(it) => &it.syntax,
            PureExpr::WhenExpr(it) => &it.syntax,
            PureExpr::CaseExpr(it) => &it.syntax,
            PureExpr::LetExpr(it) => &it.syntax,
            PureExpr::ParenExpr(it) => &it.syntax,
        }
    }
}
impl From<PureExpr> for Expr {
    fn from(node: PureExpr) -> Expr {
        Expr::PureExpr(node)
    }
}
impl From<EffExpr> for Expr {
    fn from(node: EffExpr) -> Expr {
        Expr::EffExpr(node)
    }
}
impl From<GetExpr> for EffExpr {
    fn from(node: GetExpr) -> EffExpr {
        EffExpr::GetExpr(node)
    }
}
impl From<NewExpr> for EffExpr {
    fn from(node: NewExpr) -> EffExpr {
        EffExpr::NewExpr(node)
    }
}
impl From<AsyncCallExpr> for EffExpr {
    fn from(node: AsyncCallExpr) -> EffExpr {
        EffExpr::AsyncCallExpr(node)
    }
}
impl From<SyncCallExpr> for EffExpr {
    fn from(node: SyncCallExpr) -> EffExpr {
        EffExpr::SyncCallExpr(node)
    }
}
impl From<OriginalCallExpr> for EffExpr {
    fn from(node: OriginalCallExpr) -> EffExpr {
        EffExpr::OriginalCallExpr(node)
    }
}
impl AstNode for EffExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GET_EXPR | NEW_EXPR | ASYNC_CALL_EXPR | SYNC_CALL_EXPR | ORIGINAL_CALL_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GET_EXPR => EffExpr::GetExpr(GetExpr { syntax }),
            NEW_EXPR => EffExpr::NewExpr(NewExpr { syntax }),
            ASYNC_CALL_EXPR => EffExpr::AsyncCallExpr(AsyncCallExpr { syntax }),
            SYNC_CALL_EXPR => EffExpr::SyncCallExpr(SyncCallExpr { syntax }),
            ORIGINAL_CALL_EXPR => EffExpr::OriginalCallExpr(OriginalCallExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            EffExpr::GetExpr(it) => &it.syntax,
            EffExpr::NewExpr(it) => &it.syntax,
            EffExpr::AsyncCallExpr(it) => &it.syntax,
            EffExpr::SyncCallExpr(it) => &it.syntax,
            EffExpr::OriginalCallExpr(it) => &it.syntax,
        }
    }
}
impl From<VarName> for FnListParam {
    fn from(node: VarName) -> FnListParam {
        FnListParam::VarName(node)
    }
}
impl From<AnonFn> for FnListParam {
    fn from(node: AnonFn) -> FnListParam {
        FnListParam::AnonFn(node)
    }
}
impl AstNode for FnListParam {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            VAR_NAME | ANON_FN => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            VAR_NAME => FnListParam::VarName(VarName { syntax }),
            ANON_FN => FnListParam::AnonFn(AnonFn { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FnListParam::VarName(it) => &it.syntax,
            FnListParam::AnonFn(it) => &it.syntax,
        }
    }
}
impl From<WildCardPattern> for Pattern {
    fn from(node: WildCardPattern) -> Pattern {
        Pattern::WildCardPattern(node)
    }
}
impl From<IntPattern> for Pattern {
    fn from(node: IntPattern) -> Pattern {
        Pattern::IntPattern(node)
    }
}
impl From<StringPattern> for Pattern {
    fn from(node: StringPattern) -> Pattern {
        Pattern::StringPattern(node)
    }
}
impl From<VarPattern> for Pattern {
    fn from(node: VarPattern) -> Pattern {
        Pattern::VarPattern(node)
    }
}
impl From<ConstructorPattern> for Pattern {
    fn from(node: ConstructorPattern) -> Pattern {
        Pattern::ConstructorPattern(node)
    }
}
impl AstNode for Pattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WILD_CARD_PATTERN | INT_PATTERN | STRING_PATTERN | VAR_PATTERN
            | CONSTRUCTOR_PATTERN => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            WILD_CARD_PATTERN => Pattern::WildCardPattern(WildCardPattern { syntax }),
            INT_PATTERN => Pattern::IntPattern(IntPattern { syntax }),
            STRING_PATTERN => Pattern::StringPattern(StringPattern { syntax }),
            VAR_PATTERN => Pattern::VarPattern(VarPattern { syntax }),
            CONSTRUCTOR_PATTERN => Pattern::ConstructorPattern(ConstructorPattern { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Pattern::WildCardPattern(it) => &it.syntax,
            Pattern::IntPattern(it) => &it.syntax,
            Pattern::StringPattern(it) => &it.syntax,
            Pattern::VarPattern(it) => &it.syntax,
            Pattern::ConstructorPattern(it) => &it.syntax,
        }
    }
}
impl From<VarDeclStmt> for Stmt {
    fn from(node: VarDeclStmt) -> Stmt {
        Stmt::VarDeclStmt(node)
    }
}
impl From<AssignStmt> for Stmt {
    fn from(node: AssignStmt) -> Stmt {
        Stmt::AssignStmt(node)
    }
}
impl From<SkipStmt> for Stmt {
    fn from(node: SkipStmt) -> Stmt {
        Stmt::SkipStmt(node)
    }
}
impl From<ReturnStmt> for Stmt {
    fn from(node: ReturnStmt) -> Stmt {
        Stmt::ReturnStmt(node)
    }
}
impl From<AssertStmt> for Stmt {
    fn from(node: AssertStmt) -> Stmt {
        Stmt::AssertStmt(node)
    }
}
impl From<Block> for Stmt {
    fn from(node: Block) -> Stmt {
        Stmt::Block(node)
    }
}
impl From<IfStmt> for Stmt {
    fn from(node: IfStmt) -> Stmt {
        Stmt::IfStmt(node)
    }
}
impl From<SwitchStmt> for Stmt {
    fn from(node: SwitchStmt) -> Stmt {
        Stmt::SwitchStmt(node)
    }
}
impl From<WhileStmt> for Stmt {
    fn from(node: WhileStmt) -> Stmt {
        Stmt::WhileStmt(node)
    }
}
impl From<ForeachStmt> for Stmt {
    fn from(node: ForeachStmt) -> Stmt {
        Stmt::ForeachStmt(node)
    }
}
impl From<TryCatchFinallyStmt> for Stmt {
    fn from(node: TryCatchFinallyStmt) -> Stmt {
        Stmt::TryCatchFinallyStmt(node)
    }
}
impl From<AwaitStmt> for Stmt {
    fn from(node: AwaitStmt) -> Stmt {
        Stmt::AwaitStmt(node)
    }
}
impl From<SuspendStmt> for Stmt {
    fn from(node: SuspendStmt) -> Stmt {
        Stmt::SuspendStmt(node)
    }
}
impl From<DurationStmt> for Stmt {
    fn from(node: DurationStmt) -> Stmt {
        Stmt::DurationStmt(node)
    }
}
impl From<ThrowStmt> for Stmt {
    fn from(node: ThrowStmt) -> Stmt {
        Stmt::ThrowStmt(node)
    }
}
impl From<DieStmt> for Stmt {
    fn from(node: DieStmt) -> Stmt {
        Stmt::DieStmt(node)
    }
}
impl From<MoveCogToStmt> for Stmt {
    fn from(node: MoveCogToStmt) -> Stmt {
        Stmt::MoveCogToStmt(node)
    }
}
impl From<ExprStmt> for Stmt {
    fn from(node: ExprStmt) -> Stmt {
        Stmt::ExprStmt(node)
    }
}
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            VAR_DECL_STMT
            | ASSIGN_STMT
            | SKIP_STMT
            | RETURN_STMT
            | ASSERT_STMT
            | BLOCK
            | IF_STMT
            | SWITCH_STMT
            | WHILE_STMT
            | FOREACH_STMT
            | TRY_CATCH_FINALLY_STMT
            | AWAIT_STMT
            | SUSPEND_STMT
            | DURATION_STMT
            | THROW_STMT
            | DIE_STMT
            | MOVE_COG_TO_STMT
            | EXPR_STMT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            VAR_DECL_STMT => Stmt::VarDeclStmt(VarDeclStmt { syntax }),
            ASSIGN_STMT => Stmt::AssignStmt(AssignStmt { syntax }),
            SKIP_STMT => Stmt::SkipStmt(SkipStmt { syntax }),
            RETURN_STMT => Stmt::ReturnStmt(ReturnStmt { syntax }),
            ASSERT_STMT => Stmt::AssertStmt(AssertStmt { syntax }),
            BLOCK => Stmt::Block(Block { syntax }),
            IF_STMT => Stmt::IfStmt(IfStmt { syntax }),
            SWITCH_STMT => Stmt::SwitchStmt(SwitchStmt { syntax }),
            WHILE_STMT => Stmt::WhileStmt(WhileStmt { syntax }),
            FOREACH_STMT => Stmt::ForeachStmt(ForeachStmt { syntax }),
            TRY_CATCH_FINALLY_STMT => Stmt::TryCatchFinallyStmt(TryCatchFinallyStmt { syntax }),
            AWAIT_STMT => Stmt::AwaitStmt(AwaitStmt { syntax }),
            SUSPEND_STMT => Stmt::SuspendStmt(SuspendStmt { syntax }),
            DURATION_STMT => Stmt::DurationStmt(DurationStmt { syntax }),
            THROW_STMT => Stmt::ThrowStmt(ThrowStmt { syntax }),
            DIE_STMT => Stmt::DieStmt(DieStmt { syntax }),
            MOVE_COG_TO_STMT => Stmt::MoveCogToStmt(MoveCogToStmt { syntax }),
            EXPR_STMT => Stmt::ExprStmt(ExprStmt { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::VarDeclStmt(it) => &it.syntax,
            Stmt::AssignStmt(it) => &it.syntax,
            Stmt::SkipStmt(it) => &it.syntax,
            Stmt::ReturnStmt(it) => &it.syntax,
            Stmt::AssertStmt(it) => &it.syntax,
            Stmt::Block(it) => &it.syntax,
            Stmt::IfStmt(it) => &it.syntax,
            Stmt::SwitchStmt(it) => &it.syntax,
            Stmt::WhileStmt(it) => &it.syntax,
            Stmt::ForeachStmt(it) => &it.syntax,
            Stmt::TryCatchFinallyStmt(it) => &it.syntax,
            Stmt::AwaitStmt(it) => &it.syntax,
            Stmt::SuspendStmt(it) => &it.syntax,
            Stmt::DurationStmt(it) => &it.syntax,
            Stmt::ThrowStmt(it) => &it.syntax,
            Stmt::DieStmt(it) => &it.syntax,
            Stmt::MoveCogToStmt(it) => &it.syntax,
            Stmt::ExprStmt(it) => &it.syntax,
        }
    }
}
impl From<ClaimGuard> for Guard {
    fn from(node: ClaimGuard) -> Guard {
        Guard::ClaimGuard(node)
    }
}
impl From<DurationGuard> for Guard {
    fn from(node: DurationGuard) -> Guard {
        Guard::DurationGuard(node)
    }
}
impl From<ExprGuard> for Guard {
    fn from(node: ExprGuard) -> Guard {
        Guard::ExprGuard(node)
    }
}
impl From<AndGuard> for Guard {
    fn from(node: AndGuard) -> Guard {
        Guard::AndGuard(node)
    }
}
impl AstNode for Guard {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CLAIM_GUARD | DURATION_GUARD | EXPR_GUARD | AND_GUARD => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CLAIM_GUARD => Guard::ClaimGuard(ClaimGuard { syntax }),
            DURATION_GUARD => Guard::DurationGuard(DurationGuard { syntax }),
            EXPR_GUARD => Guard::ExprGuard(ExprGuard { syntax }),
            AND_GUARD => Guard::AndGuard(AndGuard { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Guard::ClaimGuard(it) => &it.syntax,
            Guard::DurationGuard(it) => &it.syntax,
            Guard::ExprGuard(it) => &it.syntax,
            Guard::AndGuard(it) => &it.syntax,
        }
    }
}
impl From<StarImport> for Import {
    fn from(node: StarImport) -> Import {
        Import::StarImport(node)
    }
}
impl From<FromImport> for Import {
    fn from(node: FromImport) -> Import {
        Import::FromImport(node)
    }
}
impl From<UnqualifiedImport> for Import {
    fn from(node: UnqualifiedImport) -> Import {
        Import::UnqualifiedImport(node)
    }
}
impl AstNode for Import {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STAR_IMPORT | FROM_IMPORT | UNQUALIFIED_IMPORT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            STAR_IMPORT => Import::StarImport(StarImport { syntax }),
            FROM_IMPORT => Import::FromImport(FromImport { syntax }),
            UNQUALIFIED_IMPORT => Import::UnqualifiedImport(UnqualifiedImport { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Import::StarImport(it) => &it.syntax,
            Import::FromImport(it) => &it.syntax,
            Import::UnqualifiedImport(it) => &it.syntax,
        }
    }
}
impl From<StarExport> for Export {
    fn from(node: StarExport) -> Export {
        Export::StarExport(node)
    }
}
impl From<PartialExport> for Export {
    fn from(node: PartialExport) -> Export {
        Export::PartialExport(node)
    }
}
impl AstNode for Export {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STAR_EXPORT | PARTIAL_EXPORT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            STAR_EXPORT => Export::StarExport(StarExport { syntax }),
            PARTIAL_EXPORT => Export::PartialExport(PartialExport { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Export::StarExport(it) => &it.syntax,
            Export::PartialExport(it) => &it.syntax,
        }
    }
}
impl From<DataTypeDecl> for Decl {
    fn from(node: DataTypeDecl) -> Decl {
        Decl::DataTypeDecl(node)
    }
}
impl From<FunctionDecl> for Decl {
    fn from(node: FunctionDecl) -> Decl {
        Decl::FunctionDecl(node)
    }
}
impl From<ParFunctionDecl> for Decl {
    fn from(node: ParFunctionDecl) -> Decl {
        Decl::ParFunctionDecl(node)
    }
}
impl From<TypeSyn> for Decl {
    fn from(node: TypeSyn) -> Decl {
        Decl::TypeSyn(node)
    }
}
impl From<ExceptionDecl> for Decl {
    fn from(node: ExceptionDecl) -> Decl {
        Decl::ExceptionDecl(node)
    }
}
impl From<InterfaceDecl> for Decl {
    fn from(node: InterfaceDecl) -> Decl {
        Decl::InterfaceDecl(node)
    }
}
impl From<ClassDecl> for Decl {
    fn from(node: ClassDecl) -> Decl {
        Decl::ClassDecl(node)
    }
}
impl From<TraitDecl> for Decl {
    fn from(node: TraitDecl) -> Decl {
        Decl::TraitDecl(node)
    }
}
impl AstNode for Decl {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            DATA_TYPE_DECL | FUNCTION_DECL | PAR_FUNCTION_DECL | TYPE_SYN | EXCEPTION_DECL
            | INTERFACE_DECL | CLASS_DECL | TRAIT_DECL => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            DATA_TYPE_DECL => Decl::DataTypeDecl(DataTypeDecl { syntax }),
            FUNCTION_DECL => Decl::FunctionDecl(FunctionDecl { syntax }),
            PAR_FUNCTION_DECL => Decl::ParFunctionDecl(ParFunctionDecl { syntax }),
            TYPE_SYN => Decl::TypeSyn(TypeSyn { syntax }),
            EXCEPTION_DECL => Decl::ExceptionDecl(ExceptionDecl { syntax }),
            INTERFACE_DECL => Decl::InterfaceDecl(InterfaceDecl { syntax }),
            CLASS_DECL => Decl::ClassDecl(ClassDecl { syntax }),
            TRAIT_DECL => Decl::TraitDecl(TraitDecl { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Decl::DataTypeDecl(it) => &it.syntax,
            Decl::FunctionDecl(it) => &it.syntax,
            Decl::ParFunctionDecl(it) => &it.syntax,
            Decl::TypeSyn(it) => &it.syntax,
            Decl::ExceptionDecl(it) => &it.syntax,
            Decl::InterfaceDecl(it) => &it.syntax,
            Decl::ClassDecl(it) => &it.syntax,
            Decl::TraitDecl(it) => &it.syntax,
        }
    }
}
impl From<TraitMethodSet> for BasicTraitExpr {
    fn from(node: TraitMethodSet) -> BasicTraitExpr {
        BasicTraitExpr::TraitMethodSet(node)
    }
}
impl From<TraitMethod> for BasicTraitExpr {
    fn from(node: TraitMethod) -> BasicTraitExpr {
        BasicTraitExpr::TraitMethod(node)
    }
}
impl From<TraitName> for BasicTraitExpr {
    fn from(node: TraitName) -> BasicTraitExpr {
        BasicTraitExpr::TraitName(node)
    }
}
impl AstNode for BasicTraitExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRAIT_METHOD_SET | TRAIT_METHOD | TRAIT_NAME => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TRAIT_METHOD_SET => BasicTraitExpr::TraitMethodSet(TraitMethodSet { syntax }),
            TRAIT_METHOD => BasicTraitExpr::TraitMethod(TraitMethod { syntax }),
            TRAIT_NAME => BasicTraitExpr::TraitName(TraitName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            BasicTraitExpr::TraitMethodSet(it) => &it.syntax,
            BasicTraitExpr::TraitMethod(it) => &it.syntax,
            BasicTraitExpr::TraitName(it) => &it.syntax,
        }
    }
}
impl From<TraitRemoveSig> for TraitOp {
    fn from(node: TraitRemoveSig) -> TraitOp {
        TraitOp::TraitRemoveSig(node)
    }
}
impl From<TraitRemoveSet> for TraitOp {
    fn from(node: TraitRemoveSet) -> TraitOp {
        TraitOp::TraitRemoveSet(node)
    }
}
impl From<TraitAdd> for TraitOp {
    fn from(node: TraitAdd) -> TraitOp {
        TraitOp::TraitAdd(node)
    }
}
impl From<TraitModifies> for TraitOp {
    fn from(node: TraitModifies) -> TraitOp {
        TraitOp::TraitModifies(node)
    }
}
impl AstNode for TraitOp {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TRAIT_REMOVE_SIG | TRAIT_REMOVE_SET | TRAIT_ADD | TRAIT_MODIFIES => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TRAIT_REMOVE_SIG => TraitOp::TraitRemoveSig(TraitRemoveSig { syntax }),
            TRAIT_REMOVE_SET => TraitOp::TraitRemoveSet(TraitRemoveSet { syntax }),
            TRAIT_ADD => TraitOp::TraitAdd(TraitAdd { syntax }),
            TRAIT_MODIFIES => TraitOp::TraitModifies(TraitModifies { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TraitOp::TraitRemoveSig(it) => &it.syntax,
            TraitOp::TraitRemoveSet(it) => &it.syntax,
            TraitOp::TraitAdd(it) => &it.syntax,
            TraitOp::TraitModifies(it) => &it.syntax,
        }
    }
}
impl std::fmt::Display for AnyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PureExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for EffExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FnListParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BasicTraitExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParamList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Annotations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Annotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FnExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PartialFnExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VariadicFnExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ConstructorExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NameExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ThisExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NullExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ImplementsExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AsExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WhenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CaseExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PureExprList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CaseExprBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetDefs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnonFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NewExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AsyncCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SyncCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for OriginalCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VarDeclStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AssignStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SkipStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AssertStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SwitchStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ForeachStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TryCatchFinallyStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AwaitStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SuspendStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DurationStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ThrowStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DieStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MoveCogToStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CaseStmtBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ClaimGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DurationGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExprGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AndGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WildCardPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IntPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for StringPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VarPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ConstructorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DataTypeDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GenericArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DataConstructorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DataConstructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DataConstructorArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DataConstructorArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeSyn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExceptionDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FunctionDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParFunctionDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FunctionNameList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for InterfaceDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExtendsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for InterfaceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MethodSig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MethodDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ClassDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ImplementsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FieldDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecoverBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for StarImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FromImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for UnqualifiedImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for StarExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PartialExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitMethodSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitRemoveSig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitRemoveSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TraitModifies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
