use rowan::{GreenNode, TextRange};
use text_edit::Indel;

use crate::{SyntaxError, SyntaxNode};

pub(crate) fn incremental_reparse(
    node: &SyntaxNode,
    edit: &Indel,
    errors: Vec<SyntaxError>,
) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
    todo!()
}
