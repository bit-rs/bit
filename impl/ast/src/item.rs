/// Imports
use crate::atom::{Param, TypeHint};
use common::token::Span;

/// Represents struct field
#[derive(Debug, Clone)]
pub struct Field {
    pub span: Span,
    pub name: String,
    pub hint: TypeHint,
}

/// Represents struct top-level item
#[derive(Debug, Clone)]
pub struct Struct {
    pub span: Span,
    pub name: String,
    pub generics: Vec<String>,
    pub fields: Vec<Field>,
}

/// Represents enum variant
#[derive(Debug, Clone)]
pub struct Variant {
    pub span: Span,
    pub name: String,
    pub fields: Vec<Field>,
}

/// Represents enum top-level item
#[derive(Debug, Clone)]
pub struct Enum {
    pub span: Span,
    pub name: String,
    pub generics: Vec<String>,
    pub variants: Vec<Variant>,
}

/// Function top-level item
#[derive(Debug, Clone)]
pub struct Function {
    pub span: Span,
    pub name: String,
    pub generics: Vec<String>,
    pub params: Vec<Param>,
}

/// Top-level item
#[derive(Debug, Clone)]
pub enum Item {
    Struct(Struct),
    Enum(Enum),
    Function(Function),
}
