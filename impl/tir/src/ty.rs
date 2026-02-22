/// Imports
use crate::adt::{Enum, Struct};
use id_arena::Id;

/// Defines primitive signed integer type
pub enum IntTy {
    I8,
    I16,
    I32,
    I64,
}

/// Defines primitive unsigned integer type
pub enum UIntTy {
    U8,
    U16,
    U32,
    U64,
}

/// Defines primitive floating-point type
pub enum FloatTy {
    F32,
    F64,
}

/// Defines type variable
pub enum TyVar {
    /// Unbound type variable
    Unbound,

    /// Int type variable
    Int,

    /// Float type variable
    Float,

    /// Bound type variable
    Bound(Ty),
}

/// Represents generic arguments
pub type GenericArgs = Vec<Ty>;

/// Defines the type used by type system
/// and typed intermediate representation (TIR)
pub enum Ty {
    /// A primitive signed integer type. For example, `i32`.
    Int(IntTy),

    /// A primitive unsigned integer type. For example, `u32`.
    UInt(UIntTy),

    /// A primitive floating-point type. For example, `f64`.
    Float(FloatTy),

    /// A primitive string slic type.
    String,

    /// A primitive char type.
    Char,

    /// A primitive boolean type
    Bool,

    /// An enum type
    Enum(Id<Enum>, GenericArgs),

    /// An struct type
    Struct(Id<Struct>, GenericArgs),

    /// An generic type that pointee to index
    /// in generic args vector
    Generic(usize),

    // An inference type variable
    Infer(Id<TyVar>),
}
