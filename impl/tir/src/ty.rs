/// Imports
use crate::def::{AdtDef, FnDef, ModuleDef};
use id_arena::Id;
use std::fmt::Debug;

/// Defines type variable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyVar {
    /// Unbound type variable
    Unbound,

    /// Bound type variable
    Bound(Ty),
}

/// Represents generic arguments
pub type GenericArgs = Vec<Ty>;

/// Defines function signature
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnSig {
    /// Function parameters types
    pub params: Vec<Ty>,

    /// Function return type
    pub ret: Ty,
}

/// Defines meta type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetaTy {
    /// Module type
    Module(Id<ModuleDef>),

    /// Adt meta type
    Adt(Id<AdtDef>),

    /// Variant meta type
    Variant(Id<AdtDef>, String),
}

/// Defines the type used by type system
/// and typed intermediate representation (TIR)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    /// A primitive signed integer type.
    Int,

    /// A primitive floating-point type.
    Float,

    /// A primitive string slice type.
    String,

    /// A primitive boolean type
    Bool,

    /// `()` — unit type
    Unit,

    /// An adt type
    Adt(Id<AdtDef>, GenericArgs),

    /// Function definition type
    FnDef(Id<FnDef>, GenericArgs),

    /// Function pointer type
    FnRef(Box<FnSig>),

    /// Generic parameter type `T`, `K`
    Generic(usize),

    /// An inference type variable
    Var(Id<TyVar>),

    /// Meta type
    Meta(MetaTy),

    /// A placeholder for a type which could not be computed
    Error,
}
