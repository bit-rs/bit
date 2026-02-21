/// Imports
use crate::atom::{Param, TypeHint};
use common::token::Span;

/// Represents struct field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    pub span: Span,
    pub name: String,
    pub hint: TypeHint,
}

/// Represents struct top-level item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct {
    pub name: String,
    pub generics: Vec<String>,
    pub fields: Vec<Field>,
}

/// Represents enum variant
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub span: Span,
    pub name: String,
    pub fields: Vec<Field>,
}

/// Represents enum top-level item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enum {
    pub name: String,
    pub generics: Vec<String>,
    pub variants: Vec<Variant>,
}

/// Function top-level item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub name: String,
    pub generics: Vec<String>,
    pub params: Vec<Param>,
}

/// Top-level item kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Struct(Struct),
    Enum(Enum),
    Function(Function),
}

/// Top-level item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}
