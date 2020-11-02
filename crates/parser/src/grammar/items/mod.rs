mod class;
mod export;
mod import;

use crate::parser::Parser;

pub(crate) use class::*;
pub(crate) use export::*;
pub(crate) use import::*;

pub(crate) fn decl(p: &mut Parser) {}
