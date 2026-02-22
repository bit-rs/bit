/// Imports
use crate::adt::Struct;
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
    Enum(Id<EnumTy>, GenericArgs),

    /// An struct type
    Struct(Id<StructTy>, GenericArgs),

    /// An generic type that pointee to index
    /// in generic args vector
    Generic(usize),

    // An inference type variable
    Infer(Id<TyVar>),
}
