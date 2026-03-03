/// Imports
use common::token::Span;

use crate::ty::Ty;

/// Function param
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}
