/// Imports
use common::token::Span;

use crate::ty::Ty;

/// Represents item publicity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Publicity {
    Pub,
    Private,
}

/// Represents declaration mutability
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mutability {
    Mut,
    Immut,
}

/// Function param
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}
