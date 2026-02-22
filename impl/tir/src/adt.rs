/// Imports
use crate::ty::Ty;
use common::token::Span;

/// Defines generic parameter
pub struct Generic {
    /// Generic name
    pub name: String,

    /// Generic id
    pub id: usize,
}

/// Defines structure field
pub struct Field {
    /// Span of the field definition
    pub span: Span,

    /// Field name
    pub name: String,

    /// Non-instantiated field type
    pub ty: Ty,
}

/// Defines structure type
pub struct Struct {
    /// Span of the structure definition
    pub span: Span,

    /// Structure name
    pub name: String,

    /// Structure generics
    pub generics: Vec<Generic>,

    /// Structure fields
    pub fields: Vec<Field>,
}

/// Defines enum variant
pub struct Variant {
    /// Span of the variant definition
    pub span: Span,

    /// Variant name
    pub name: String,

    /// Non-instantiated variant params
    pub params: Vec<Ty>,
}

/// Defines enum type
pub struct Enum {
    /// Span of the enum definition
    pub span: Span,

    /// Enum name
    pub name: String,

    /// Enum generics
    pub generics: Vec<Generic>,

    /// Enum fields
    pub fields: Vec<Variant>,
}
