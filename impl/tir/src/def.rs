use std::sync::Arc;

/// Imports
use crate::ty::Ty;
use ast::atom::Publicity;
use common::token::Span;
use id_arena::Id;
use macros::bug;
use miette::NamedSource;

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
    pub fields: Vec<Ty>,
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
    pub variants: Vec<VariantDef>,
}

/// Represents adt definition
pub enum AdtDef {
    Struct(StructDef),
    Enum(EnumDef),
}

/// Implementation
impl AdtDef {
    // Returns ADT name
    pub fn name(&self) -> String {
        match self {
            AdtDef::Struct(struct_def) => struct_def.name.clone(),
            AdtDef::Enum(enum_def) => enum_def.name.clone(),
        }
    }

    // Returns ADT as Enum
    pub fn as_enum(&self) -> &EnumDef {
        match self {
            AdtDef::Enum(enum_def) => &enum_def,
            AdtDef::Struct(_) => bug!("converted non-enum adt to enum"),
        }
    }

    // Returns ADT as Struct
    pub fn as_struct(&self) -> &StructDef {
        match self {
            AdtDef::Struct(struct_def) => &struct_def,
            AdtDef::Enum(_) => bug!("converted non-struct adt to struct"),
        }
    }
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

/// Definition kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefKind {
    /// ADT definition
    Adt(Id<AdtDef>),

    /// Function definition
    Fn(Id<FnDef>),
}

/// Resolution definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Def {
    pub publicity: Publicity,
    pub kind: DefKind,
}

/// Represents module
pub struct Module {
    pub source: Arc<NamedSource<String>>,
    pub defs: Vec<Def>,
}
