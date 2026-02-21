/// Imports
use crate::{
    atom::{Param, Publicity, TypeHint},
    stmt::Block,
};
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
    pub params: Vec<TypeHint>,
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
    pub ret: TypeHint,
    pub block: Block,
}

/// Top-level use kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UseKind {
    /// `as $name`
    As(String),

    /// `for ...`
    For(Vec<String>),

    /// Just import
    Just,
}

/// Top-level use
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Use {
    pub path: String,
    pub kind: UseKind,
}

/// Top-level item kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemKind {
    /// Struct item
    Struct(Struct),

    /// Enum item
    Enum(Enum),

    /// Function item
    Function(Function),

    /// Use item
    Use(Use),
}

/// Top-level item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub publicity: Publicity,
    pub kind: ItemKind,
    pub span: Span,
}

/// Module
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub items: Vec<Item>,
}
