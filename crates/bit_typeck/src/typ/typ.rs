/// Imports
use crate::{
    pretty::Pretty,
    typ::{cx::InferCx, def::ModuleDef},
};
use bit_ast::ast::Publicity;
use bit_common::span::Span;
use ecow::EcoString;
use id_arena::Id;
use miette::NamedSource;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

/// Represents built-in or prelude types in the language.
///
/// `PreludeType` is an enum of primitive types that are
/// available by default in the language, typically without needing
/// an explicit import. These types form the foundation for
/// type checking, expression evaluation, and variable declarations.
///
/// # Variants
///
/// - `Int`
///
///   Represents a signed integer (int64) type. Typically used for whole numbers.
///   Examples: `42`, `-7`.
///
/// - `Float`
///
///   Represents a floating-point (float64) number type. Used for decimal numbers
///   or numbers requiring fractional precision. Examples: `3.14`, `-0.001`.
///
/// - `Bool`
///
///   Represents a boolean type, which can have one of two values:
///   `true` or `false`. Used for logical expressions and control flow.
///
/// - `String`
///
///   Represents a sequence of characters. Used for textual data.
///   Examples: `"hello"`, `"Rust"`.
///
#[derive(Debug, Clone, PartialEq)]
pub enum PreludeType {
    Int,
    Float,
    Bool,
    String,
}

/// Represents a function or enum variant parameter in the language.
///
/// A `Parameter` stores the information about a single parameter
/// of a function or enum, including its type and its location from the
/// source code file.
///
/// # Fields
///
/// - `location: Address`
///   The source code location binding
///
/// - `name: EcoString`
///   The identifier of the parameter
///
/// - `typ: Typ`
///   The type of the parameter. Determines what kind of values
///   can be passed to the function for this parameter. This is
///   used during type checking to ensure correctness.
///
#[derive(Clone, PartialEq)]
pub struct Parameter {
    pub location: Span,
    pub name: EcoString,
    pub typ: Typ,
}

/// Represents a field of a struct.
///
/// A `Field` stores the name, type, and source location of a field
/// in a struct. Used for type checking.
///
/// # Fields
///
/// - `name: EcoString`
///   The identifier of the field.
///
/// - `location: Address`
///   The location in the source code where this field is declared.
///   Includes fields type annotation span too.
///
/// - `typ: Typ`
///   The type  of the field
///
#[derive(Clone, PartialEq)]
pub struct Field {
    pub name: EcoString,
    pub location: Span,
    pub typ: Typ,
}

/// Debug implementation
impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.typ)
    }
}

/// Represents a user-defined structure.
///
/// A `Struct` stores all information about a user-defined struct,
/// including its name, generic parameters, fields, and source location. Used
/// for type checking, generic instantiation and infrenece.
///
/// # Fields
///
/// - `location: Address`
///   The location in the source code where the struct is declared.
///
/// - `name: EcoString`
///   The name of the struct.
///
/// - `generics: Vec<EcoString>`
///   A list of generic parameters for the struct.
///
/// - `fields: Vec<Field>`
///   A list of fields in the struct, each with its name, type, and location.
///
#[derive(Clone)]
pub struct Struct {
    pub location: Span,
    pub name: EcoString,
    pub generics: Vec<EcoString>,
    pub fields: Vec<Field>,
}

/// Debug implementation
impl Debug for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Struct({})", self.name)
    }
}

/// Represents a variant of an enum.
///
/// An `EnumVariant` stores the name, source location, and optional
/// parameters (fields) associated with this variant. Used for
/// type checking, pattern matching, and runtime representation.
///
/// # Fields
///
/// - `location: Address`
///   The location in the source code where this variant is declared.
///
/// - `name: EcoString`
///   The identifier of the variant, e.g., `Some`, `None`.
///
/// - `fields: Vec<Field>`
///   Optional named parameters (fields) for the variant.
///
#[derive(Clone, PartialEq)]
pub struct EnumVariant {
    pub location: Span,
    pub name: EcoString,
    pub fields: Vec<Field>,
}

/// Debug implementation for `EnumVariant`
impl Debug for EnumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variant({})", self.name)
    }
}

/// Represents a custom enum type.
///
/// An `Enum` stores all information about a user-defined enum,
/// including its name, generics, variants, and source location.
/// Used for type checking, generic instantiation, pattern matching
/// exhaustiveness check.
///
/// # Fields
///
/// - `location: Address`
///   The location in the source code where the enum is declared.
///
/// - `name: EcoString`
///   The name of the enum.
///
/// - `generics: Vec<EcoString>`
///   A list of generic parameters for the enum.
///
/// - `variants: Vec<EnumVariant>`
///   A list of variants for this enum, each with its own name, location,
///   and optional parameters.
///
#[derive(Clone)]
#[allow(dead_code)]
pub struct Enum {
    pub location: Span,
    pub name: EcoString,
    pub generics: Vec<EcoString>,
    pub variants: Vec<EnumVariant>,
}

/// Debug implementation for `Enum`
///
/// Displays the enum name in the format `Enum(name)`.
impl Debug for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enum({})", self.name)
    }
}

/// Represents a user-defined function in the language.
///
/// A `Function` stores all necessary information about a function,
/// including its name, parameters, return type, generics, and source location.
///
/// # Fields
///
/// - `location: Address`
///   The location in the source code where the function is declared. Useful
///   for error reporting and debugging.
///
/// - `name: EcoString`
///   The name of the function.
///
/// - `generics: Vec<EcoString>`
///   A list of generic parameters for the function, if any.
///
/// - `params: Vec<Parameter>`
///   A list of function parameters, each with a type and source location.
///
/// - `ret: Typ`
///   The inferred return type of the function.
///
#[derive(Clone)]
pub struct Function {
    pub location: Span,
    pub name: EcoString,
    pub generics: Vec<EcoString>,
    pub params: Vec<Parameter>,
    pub ret: Typ,
}

/// Debug implementation for `Function`
impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function({})", self.name)
    }
}

/// PartialEq implementation for `Function`
///
/// The `PartialEq` implementation ignores `name` and `location`, and considers
/// two functions equal if they have the same parameters and return type. This
/// is useful for type checking and generic instantiation.
///
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.ret == other.ret
    }
}

/// Represents a module in the language.
///
/// A `Module` contains named definitions (`ModDef`) such as functions,
/// structs, enums, or submodules. Modules serve as namespaces and
/// are used to organize code.
///
/// # Fields
///
/// - `source: Arc<NamedSource<String>>`
///   The source code file where the module is defined.
///
/// - `name: EcoString`
///   The name of the module.
///
/// - `fields: HashMap<EcoString, ModDuleef>`
///   The definitions contained in the module, keyed by their names.
///
#[derive(Clone)]
#[allow(dead_code)]
pub struct Module {
    pub source: Arc<NamedSource<String>>,
    pub name: EcoString,
    pub fields: HashMap<EcoString, ModuleDef>,
}

/// Debug implementation for `Module`
impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Module({})", self.name)
    }
}

/// Represents a generic arguments
pub type GenericArgs = Vec<Typ>;

/// Represents an inference type variable used during type checking
/// and type inference.
///
/// An `TyVar` can either be unbound (not yet determined) or bound
/// (linked to a concrete type `Typ`).
///
pub enum TyVar {
    /// A type variable that has not yet been resolved.
    /// During inference, this may later be linked to a concrete type.
    Unbound,

    /// A type variable that has already been resolved (substituted)
    /// with a concrete type `Typ`.
    Bound(Typ),
}

/// Represents a type in the language.
///
/// `Typ` covers all possible types, including:
/// - prelude primitive types (`Int`, `Float`, etc.)
/// - user-defined structs and enums
/// - function types
/// - unit type `()`
/// - unbound types for type inference
/// - generic type variables
///
#[derive(Clone, PartialEq, Debug)]
pub enum Typ {
    /// Prelude primitive types
    Prelude(PreludeType),
    /// User-defined struct type with substitutions
    Struct(Id<Struct>, GenericArgs),
    /// User-defined enum type with substitutions
    Enum(Id<Enum>, GenericArgs),
    /// Function definiton
    FnDef(Id<Function>, GenericArgs),
    /// Function reference
    FnRef(Vec<Typ>, Box<Typ>),
    /// Inference type with unique id used during type inference.
    /// (id is used to link unbound `Typ` with substitution)
    Var(Id<TyVar>),
    /// Generic type variable index
    Generic(usize),
    /// Unit type, representing `()`
    Unit,
}

/// `Typ` methods implementation
impl Typ {
    /// Retrieves fields and applies
    /// substitution by `InferCx`.
    ///
    /// # Notes
    /// If `Typ` isn't `Typ::Struct(_, _)`, will
    /// return empty vector.
    pub fn fields(&self, icx: &mut InferCx) -> Vec<Field> {
        match self {
            Typ::Struct(id, generics) => icx
                .tcx
                .struct_(*id)
                .fields
                .clone()
                .into_iter()
                .map(|field| Field {
                    location: field.location,
                    name: field.name,
                    typ: icx.subst(field.typ, generics),
                })
                .collect(),

            _ => vec![],
        }
    }

    /// Retrieves variants and applies
    /// substitution by `InferCx`.
    ///
    /// # Notes
    /// If `Typ` isn't `Typ::Enum(_, _)`, will
    /// return empty vector.
    pub fn variants(&self, icx: &mut InferCx) -> Vec<EnumVariant> {
        // Matching self
        match self {
            Typ::Enum(id, generics) => icx
                .tcx
                .enum_(*id)
                .variants
                .clone()
                .into_iter()
                .map(|variant| EnumVariant {
                    location: variant.location,
                    name: variant.name,
                    fields: variant
                        .fields
                        .into_iter()
                        .map(|field| Field {
                            location: field.location,
                            name: field.name,
                            typ: icx.subst(field.typ, generics),
                        })
                        .collect(),
                })
                .collect(),
            _ => vec![],
        }
    }

    /// Retrieves params and applies
    /// substitution by `InferCx`.
    ///
    /// # Notes
    /// If `Typ` isn't `Typ::FnDef(_, _)` or `Typ::FnPtr(_, _)`, will
    /// return empty vector.
    ///
    pub fn params(&self, icx: &mut InferCx) -> Vec<Typ> {
        // Matching self
        match self {
            Typ::FnDef(id, generics) => icx
                .tcx
                .function(*id)
                .params
                .clone()
                .into_iter()
                .map(|param| icx.subst(param.typ, generics))
                .collect(),
            Typ::FnRef(params, _) => params.clone(),
            _ => vec![],
        }
    }

    /// Retrieves return type and applies
    /// substitution by `InferCx`.
    ///
    /// # Notes
    /// If `Typ` isn't `Typ::FnDef(_, _)` or `Typ::FnPtr(_, _)`,
    /// will return Typ::Unit.
    ///
    pub fn ret(&self, icx: &mut InferCx) -> Typ {
        // Matching self
        match self {
            Typ::FnDef(id, generics) => icx.subst(icx.tcx.function(*id).ret.clone(), generics),
            Typ::FnRef(_, ret) => *ret.clone(),
            _ => Typ::Unit,
        }
    }
}

/// Pretty printing implementation
impl Pretty for Typ {
    /// Pretty prints type
    ///
    /// # Parameters
    /// - `icx: &mut InferCx`
    ///   Inference context used
    ///   to get struct, enum or function info.
    ///
    fn pretty(&self, icx: &mut InferCx) -> String {
        // Matching self
        match icx.apply(self.clone()) {
            Typ::Prelude(ty) => format!("{ty:?}"),
            Typ::Struct(id, generic_args) if !generic_args.is_empty() => {
                format!(
                    "{}[{}]",
                    icx.tcx.struct_(id).name.clone(),
                    generic_args
                        .iter()
                        .map(|t| t.pretty(icx))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Typ::Struct(id, _) => {
                format!("{}", icx.tcx.struct_(id).name.clone())
            }
            Typ::Enum(id, generic_args) if !generic_args.is_empty() => {
                format!(
                    "{}[{}]",
                    icx.tcx.enum_(id).name.clone(),
                    generic_args
                        .iter()
                        .map(|t| t.pretty(icx))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Typ::Enum(id, _) => {
                format!("{}", icx.tcx.enum_(id).name.clone())
            }
            it @ Typ::FnDef(_, _) => {
                format!(
                    "fn({}) -> {}",
                    it.params(icx)
                        .iter()
                        .map(|p| p.pretty(icx))
                        .collect::<Vec<String>>()
                        .join(", "),
                    it.ret(icx).pretty(icx)
                )
            }
            Typ::FnRef(params, ret) => {
                format!(
                    "fn({}) -> {}",
                    params
                        .iter()
                        .map(|p| p.pretty(icx))
                        .collect::<Vec<String>>()
                        .join(", "),
                    (*ret).pretty(icx)
                )
            }
            Typ::Var(id) => format!("?{}", id.index()),
            Typ::Generic(name) => format!("{name}"),
            Typ::Unit => "Unit".to_string(),
        }
    }
}

/// Wraps a value with its publicity information.
///
/// `WithPublicity` is a generic struct used to attach access
/// modifiers (like `public` or `private`) to any value.
///
/// # Fields
///
/// - `publicity: Publicity`
///   The visibility modifier of the value.
///
/// - `value: T`
///   The value being wrapped.
///
#[derive(Clone, PartialEq)]
pub struct WithPublicity<T: Clone + PartialEq> {
    pub publicity: Publicity,
    pub value: T,
}

/// Debug implementation for `WithPublicity<T>`
///
/// Displays the value along with its publicity modifier for debugging.
impl<T: Debug + Clone + PartialEq> Debug for WithPublicity<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WithPublicity({:?}, {:?})", self.publicity, self.value)
    }
}
