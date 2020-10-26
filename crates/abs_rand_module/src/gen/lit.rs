use abs_syntax::ast;

pub fn create_lit<S: Into<String>>(s: S) -> ast::Literal {
    ast::Literal { s: s.into() }
}
