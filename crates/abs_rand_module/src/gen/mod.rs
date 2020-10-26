use abs_syntax::ast;

mod expr;
mod lit;
mod stmt;
pub mod ty;

pub use expr::*;
pub use lit::*;
pub use stmt::*;

fn ident<S: Into<String>>(str: S) -> ast::Ident {
    ast::Ident { str: str.into() }
}

pub struct ModuleBuilder {
    name: ast::Ident,
    children: Vec<ast::ModuleItem>,
}

impl ModuleBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name: ident(name),
            children: Vec::new(),
        }
    }

    pub fn add_child<N: Into<ast::ModuleItem>>(&mut self, child: N) {
        self.children.push(child.into());
    }

    pub fn with_child<N: Into<ast::ModuleItem>>(mut self, child: N) -> Self {
        self.add_child(child);
        self
    }

    pub fn complete(self) -> ast::Module {
        ast::Module {
            name: self.name,
            children: self.children,
        }
    }
}

pub fn start_module<S: Into<String>>(name: S) -> ModuleBuilder {
    ModuleBuilder::new(name.into())
}

pub struct InterfaceBuilder {
    ident: ast::Ident,
    extends: Vec<ast::Ident>,
    sigs: Vec<ast::MethodSig>,
}

impl InterfaceBuilder {
    pub fn new(name: String) -> Self {
        Self {
            ident: ident(name),
            extends: vec![],
            sigs: vec![],
        }
    }

    pub fn add_extends<S: Into<String>>(&mut self, e: S) {
        self.extends.push(ident(e))
    }

    pub fn with_extends(mut self, e: String) -> Self {
        self.add_extends(e);
        self
    }

    pub fn add_sig(&mut self, sig: ast::MethodSig) {
        self.sigs.push(sig)
    }

    pub fn with_sig(mut self, sig: ast::MethodSig) -> Self {
        self.add_sig(sig);
        self
    }

    pub fn complete(self) -> ast::InterfaceDecl {
        ast::InterfaceDecl {
            ident: self.ident,
            sigs: self.sigs,
            extends: self.extends,
        }
    }
}

pub fn start_interface_decl<S: Into<String>>(name: S) -> InterfaceBuilder {
    InterfaceBuilder::new(name.into())
}

pub struct ClassDeclBuilder {
    ident: ast::Ident,
    params: Vec<ast::Param>,
    implements: Vec<ast::Ident>,
    fields: Vec<ast::FieldDecl>,
    init: Option<ast::Block>,
    recover: Vec<ast::CaseBranch<ast::Stmt>>,
    methods: Vec<ast::MethodDecl>,
}

impl ClassDeclBuilder {
    pub fn new(name: String) -> Self {
        Self {
            ident: ident(name),
            params: vec![],
            implements: vec![],
            fields: vec![],
            init: None,
            recover: vec![],
            methods: vec![],
        }
    }

    pub fn add_param(&mut self, p: ast::Param) {
        self.params.push(p);
    }

    pub fn add_implements<S: Into<String>>(&mut self, s: S) {
        self.implements.push(ident(s));
    }

    pub fn add_field(&mut self, p: ast::FieldDecl) {
        self.fields.push(p);
    }

    pub fn add_init(&mut self, i: ast::Block) {
        self.init = Some(i);
    }

    pub fn add_recover(&mut self, p: ast::CaseBranch<ast::Stmt>) {
        self.recover.push(p);
    }

    pub fn add_method(&mut self, p: ast::MethodDecl) {
        self.methods.push(p);
    }

    pub fn with_param(mut self, p: ast::Param) -> Self {
        self.add_param(p);
        self
    }

    pub fn with_implements<S: Into<String>>(mut self, p: S) -> Self {
        self.add_implements(p);
        self
    }

    pub fn with_field(mut self, p: ast::FieldDecl) -> Self {
        self.add_field(p);
        self
    }

    pub fn with_init(mut self, p: ast::Block) -> Self {
        self.add_init(p);
        self
    }

    pub fn with_recover(mut self, p: ast::CaseBranch<ast::Stmt>) -> Self {
        self.add_recover(p);
        self
    }

    pub fn with_method(mut self, p: ast::MethodDecl) -> Self {
        self.add_method(p);
        self
    }

    pub fn complete(self) -> ast::ClassDecl {
        ast::ClassDecl {
            ident: self.ident,
            params: self.params,
            implements: self.implements,
            fields: self.fields,
            init: self.init,
            recover: self.recover,
            methods: self.methods,
        }
    }
}

pub fn start_class_decl<S: Into<String>>(name: S) -> ClassDeclBuilder {
    ClassDeclBuilder::new(name.into())
}

pub struct MethodSigBuilder {
    ret: Option<ast::Type>,
    ident: ast::Ident,
    params: Vec<ast::Param>,
}

impl MethodSigBuilder {
    pub fn new(name: String) -> Self {
        Self {
            ret: None,
            ident: ident(name),
            params: vec![],
        }
    }

    pub fn add_ret(&mut self, ret: ast::Type) {
        self.ret = Some(ret)
    }

    pub fn add_param(&mut self, param: ast::Param) {
        self.params.push(param)
    }

    pub fn with_ret(mut self, ret: ast::Type) -> Self {
        self.add_ret(ret);
        self
    }

    pub fn with_param(mut self, param: ast::Param) -> Self {
        self.add_param(param);
        self
    }

    pub fn complete(self) -> ast::MethodSig {
        ast::MethodSig {
            ret: self.ret.unwrap(),
            ident: self.ident,
            params: self.params,
        }
    }
}

pub fn start_method_sig<S: Into<String>>(name: S) -> MethodSigBuilder {
    MethodSigBuilder::new(name.into())
}

pub fn create_param<S: Into<String>>(ty: ast::Type, name: S) -> ast::Param {
    ast::Param {
        ty,
        ident: ident(name.into()),
    }
}

pub fn create_field<S: Into<String>>(ty: ast::Type, name: S) -> ast::FieldDecl {
    ast::FieldDecl {
        ty,
        ident: ident(name.into()),
        init: None,
    }
}

pub fn create_field_init<S: Into<String>>(
    ty: ast::Type,
    name: S,
    init: ast::PureExpr,
) -> ast::FieldDecl {
    ast::FieldDecl {
        ty,
        ident: ident(name.into()),
        init: Some(init),
    }
}

pub fn create_method_decl(sig: ast::MethodSig, body: ast::Block) -> ast::MethodDecl {
    ast::MethodDecl { sig, body }
}
