mod class;
mod datatypes;
mod exceptions;
mod export;
mod functions;
mod import;
mod interfaces;
mod method_decls;
mod method_sigs;
mod traits;
mod type_syn;

use crate::{parser::Parser, T};

pub(crate) use class::*;
pub(crate) use datatypes::*;
pub(crate) use exceptions::*;
pub(crate) use export::*;
pub(crate) use functions::*;
pub(crate) use import::*;
pub(crate) use interfaces::*;
pub(crate) use method_decls::*;
pub(crate) use method_sigs::*;
pub(crate) use traits::*;
pub(crate) use type_syn::*;

use super::annotations_opt;

pub(crate) fn decl(p: &mut Parser) {
    let m = p.start();
    annotations_opt(p);
    match p.current() {
        T![data] => datatype(p, m),
        T![def] => maybe_par_fn(p, m),
        T![type] => type_syn(p, m),
        T![exception] => exception(p, m),
        T![interface] => interface(p, m),
        T![class] => class(p, m),
        T![trait] => trait_decl(p, m),
        _ => return p.error("expected a declaration"),
    }
}
