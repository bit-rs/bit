/// Imports
use crate::ty::Ty;
use common::token::Span;

/// Function param
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}
