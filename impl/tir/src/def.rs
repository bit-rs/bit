/// Imports
use crate::ty::Ty;
use ast::atom::Publicity;
use common::token::Span;
use id_arena::Id;
use miette::NamedSource;
use std::{collections::HashMap, sync::Arc};

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
    pub generics: Vec<String>,

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
    pub generics: Vec<String>,

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
}

/// Represents function definition in types context
pub struct FnDef {
    /// Span of the enum definition
    pub span: Span,

    /// Function name
    pub name: String,

    /// Function generics
    pub generics: Vec<String>,

    /// Function non-instantiated params
    pub params: Vec<Ty>,

    /// Function non-instantiated return type
    pub ret: Ty,
}

/// Definition kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemDefKind {
    /// ADT definition
    Adt(Id<AdtDef>),

    /// Function definition
    Fn(Id<FnDef>),
}

/// Item definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemDef {
    pub publicity: Publicity,
    pub kind: ItemDefKind,
}

/// Represents module
pub struct ModuleDef {
    pub source: Arc<NamedSource<String>>,
    pub defs: HashMap<String, ItemDef>,
}
