/// Imports
use crate::ty::Ty;
use ast::atom::Publicity;
use common::token::Span;
use id_arena::Id;
use macros::bug;
use miette::NamedSource;
use std::{collections::HashMap, sync::Arc};

/// Represents structure field
#[derive(Clone)]
pub struct FieldDef {
    /// Span of the field definition
    pub span: Span,

    /// Field name
    pub name: String,

    /// Non-instantiated field type
    pub ty: Ty,
}

/// Represents structure type
#[derive(Clone)]
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
#[derive(Clone)]
pub struct VariantDef {
    /// Span of the variant definition
    pub span: Span,

    /// Variant name
    pub name: String,

    /// Non-instantiated variant params
    pub fields: Vec<Ty>,
}

/// Represents enum definition in types context
#[derive(Clone)]
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
#[derive(Clone)]
pub enum AdtDef {
    Struct(StructDef),
    Enum(EnumDef),
}

/// Implementation
impl AdtDef {
    // Returns ADT name
    pub fn name(&self) -> String {
        match self {
            AdtDef::Struct(s) => s.name.clone(),
            AdtDef::Enum(e) => e.name.clone(),
        }
    }

    // Returns ADT as StructDef if it is, else emits bug
    pub fn as_struct(&self) -> &StructDef {
        match self {
            AdtDef::Enum(_) => bug!("expected struct, got enum by id"),
            AdtDef::Struct(s) => s,
        }
    }

    // Returns ADT as EnumDef if it is, else emits bug
    pub fn as_enum(&self) -> &EnumDef {
        match self {
            AdtDef::Struct(_) => bug!("expected struct, got enum by id"),
            AdtDef::Enum(e) => e,
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
pub struct ModDef {
    pub source: Arc<NamedSource<String>>,
    pub defs: HashMap<String, ItemDef>,
}
