/// Imports
use crate::ty::Ty;
use common::token::Span;

/// Represents generic parameter
pub struct GenericParam {
    /// Generic name
    pub name: String,

    /// Generic id
    pub id: usize,
}

/// Represents structure field
pub struct FieldDef {
    /// Span of the field definition
    pub span: Span,

    /// Field name
    pub name: String,

    /// Non-instantiated field type
    pub ty: Ty,
}

/// Represents structure type
pub struct StructDef {
    /// Span of the structure definition
    pub span: Span,

    /// Structure name
    pub name: String,

    /// Structure generics
    pub generics: Vec<GenericParam>,

    /// Structure fields
    pub fields: Vec<FieldDef>,
}

/// Defines enum variant
pub struct VariantDef {
    /// Span of the variant definition
    pub span: Span,

    /// Variant name
    pub name: String,

    /// Non-instantiated variant params
    pub params: Vec<Ty>,
}

/// Represents enum definition in types context
pub struct EnumDef {
    /// Span of the enum definition
    pub span: Span,

    /// Enum name
    pub name: String,

    /// Enum generics
    pub generics: Vec<GenericParam>,

    /// Enum variants
    pub fields: Vec<VariantDef>,
}

/// Represents adt definition
pub enum AdtDef {
    Struct(StructDef),
    Enum(EnumDef),
}

/// Represents function definition in types context
pub struct FnDef {
    /// Span of the enum definition
    pub span: Span,

    /// Function name
    pub name: String,

    /// Function generics
    pub generics: Vec<GenericParam>,

    /// Function non-instantiated params
    pub params: Vec<Ty>,

    /// Function non-instantiated return type
    pub ret: Ty,
}
