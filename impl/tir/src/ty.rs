/// Imports
use crate::def::{AdtDef, FnDef};
use id_arena::Id;
use std::fmt::Debug;

/// Defines primitive signed integer type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum IntTy {
    I8,
    I16,
    I32,
    I64,
}

/// Defines primitive unsigned integer type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum UIntTy {
    U8,
    U16,
    U32,
    U64,
}

/// Defines primitive floating-point type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum FloatTy {
    F32,
    F64,
}

/// Defines type variable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyVar {
    /// Unbound type variable
    Unbound,

    /// Int type variable
    Int(Option<bool>),

    /// Float type variable
    Float,

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

/// Defines the type used by type system
/// and typed intermediate representation (TIR)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    /// A primitive signed integer type. For example, `i32`.
    Int(IntTy),

    /// A primitive unsigned integer type. For example, `u32`.
    UInt(UIntTy),

    /// A primitive floating-point type. For example, `f64`.
    Float(FloatTy),

    /// A primitive string slice type.
    String,

    /// A primitive char type.
    Char,

    /// A primitive boolean type
    Bool,

    /// `()` — unit type
    Unit,

    /// An adt type
    Adt(Id<AdtDef>, GenericArgs),

    /// Function definition type
    FnDef(Id<FnDef>, GenericArgs),

    /// Function pointer type
    FnPtr(Box<FnSig>),

    /// Generic parameter type `T`, `K`
    Generic(usize),

    /// An inference type variable
    Var(Id<TyVar>),

    /// Shared reference `&T`
    Ref(Box<Ty>),

    /// Mutable reference `&mut T`
    MutRef(Box<Ty>),

    /// A placeholder for a type which could not be computed
    Error,
}
