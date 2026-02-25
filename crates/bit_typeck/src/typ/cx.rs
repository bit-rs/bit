/// Imports
use crate::{
    inference::generics::Generics,
    typ::typ::{Enum, Function, GenericArgs, Struct, TyVar, Typ},
};
use ecow::EcoString;
use id_arena::{Arena, Id};

/// Type context.
///
/// `TyCx` owns and stores all type-level definitions used by the compiler,
/// such as functions, structs, and enums.
///
/// All definitions are allocated in arenas and are referenced indirectly
/// via typed IDs (`Id<T>`). This provides:
///
/// - zero-cost copying of references
/// - stable identities for types
/// - support for recursive and cyclic type graphs
/// - clear separation between type *references* and type *definitions*
///
/// `TyCx` is expected to live for the entire duration of type checking
/// and later compilation phases.
///
#[derive(Default)]
pub struct TyCx {
    /// Arena storing all function type definitions.
    pub funcs: Arena<Function>,

    /// Arena storing all struct definitions.
    pub structs: Arena<Struct>,

    /// Arena storing all enum definitions.
    pub enums: Arena<Enum>,
}

impl TyCx {
    /// Allocates a new function definition in the type context
    /// and returns its unique ID.
    #[inline]
    pub fn insert_function(&mut self, function: Function) -> Id<Function> {
        self.funcs.alloc(function)
    }

    /// Allocates a new struct definition in the type context
    /// and returns its unique ID.
    #[inline]
    pub fn insert_struct(&mut self, struct_: Struct) -> Id<Struct> {
        self.structs.alloc(struct_)
    }

    /// Allocates a new enum definition in the type context
    /// and returns its unique ID.
    #[inline]
    pub fn insert_enum(&mut self, enum_: Enum) -> Id<Enum> {
        self.enums.alloc(enum_)
    }

    /// Returns an immutable reference to a function definition.
    ///
    /// # Panics
    ///
    /// Panics if the given `id` does not belong to this `TyCx`.
    #[inline]
    pub fn function(&self, id: Id<Function>) -> &Function {
        self.funcs.get(id).expect("invalid Function id")
    }

    /// Returns an immutable reference to a struct definition.
    ///
    /// # Panics
    ///
    /// Panics if the given `id` does not belong to this `TyCx`.
    #[inline]
    pub fn struct_(&self, id: Id<Struct>) -> &Struct {
        self.structs.get(id).expect("invalid Struct id")
    }

    /// Returns an immutable reference to an enum definition.
    ///
    /// # Panics
    ///
    /// Panics if the given `id` does not belong to this `TyCx`.
    #[inline]
    pub fn enum_(&self, id: Id<Enum>) -> &Enum {
        self.enums.get(id).expect("invalid Enum id")
    }

    /// Returns a mutable reference to a function definition.
    ///
    /// # Panics
    ///
    /// Panics if the given `id` does not belong to this `TyCx`.
    #[inline]
    pub fn function_mut(&mut self, id: Id<Function>) -> &mut Function {
        self.funcs.get_mut(id).expect("invalid Function id")
    }

    /// Returns a mutable reference to a struct definition.
    ///
    /// # Panics
    ///
    /// Panics if the given `id` does not belong to this `TyCx`.
    #[inline]
    pub fn struct_mut(&mut self, id: Id<Struct>) -> &mut Struct {
        self.structs.get_mut(id).expect("invalid Struct id")
    }

    /// Returns a mutable reference to an enum definition.
    ///
    /// # Panics
    ///
    /// Panics if the given `id` does not belong to this `TyCx`.
    #[inline]
    pub fn enum_mut(&mut self, id: Id<Enum>) -> &mut Enum {
        self.enums.get_mut(id).expect("invalid Enum id")
    }

    /// Executes `f` with an immutable reference to the function definition
    /// if the given `id` exists in this context.
    ///
    /// Returns `Some(result)` if the function was found, or `None` otherwise.
    #[inline]
    pub fn with_function<R>(&self, id: Id<Function>, f: impl FnOnce(&Function) -> R) -> Option<R> {
        self.funcs.get(id).map(f)
    }

    /// Executes `f` with an immutable reference to the struct definition
    /// if the given `id` exists in this context.
    #[inline]
    pub fn with_struct<R>(&self, id: Id<Struct>, f: impl FnOnce(&Struct) -> R) -> Option<R> {
        self.structs.get(id).map(f)
    }

    /// Executes `f` with an immutable reference to the enum definition
    /// if the given `id` exists in this context.
    #[inline]
    pub fn with_enum<R>(&self, id: Id<Enum>, f: impl FnOnce(&Enum) -> R) -> Option<R> {
        self.enums.get(id).map(f)
    }
}

/// Performs type variable substitution and instantiation during type inference.
///
/// The `InferCx` is responsible for **resolving unbound type variables**,
/// applying substitutions, and **instantiating generic types** into concrete
/// representations. It operates during the type inference process (unification),
/// ensuring that all types in the type system are fully resolved (i.e., “inferred”).
///
/// # Fields
///
/// - `substitutions: HashMap<usize, Typ>`
///   A mapping of **unbound type variable IDs** to their corresponding resolved types.
///   During unification, whenever a variable is assigned a type, that binding is stored here.
///   The hydrator uses this map to recursively replace variable references with their final types
///   during the type instantiation.
///
/// - `last_unbound_id: usize`
///   The **last generated unbound type ID**, used to generate fresh uid
///   for new unbound type variables during inference.
///   This acts like a counter for generating unique type variable identifiers.
///
/// - `generics: Generics`
///   Tracks **generic parameters** visible in the current scope.
///   This allows the hydrator to distinguish between *generic* and *inference* variables,
///   and to correctly instantiate generics when entering or leaving scopes.
///
/// # Typical Responsibilities
///
/// 1. **Apply substitutions**
///    Recursively replaces all unbound type variables (`Typ::Unbound(id)`) with their
///    corresponding resolved types from the `substitutions` map.
///
/// 2. **Instantiate generics**
///    When a generic type is used, the hydrator creates a fresh unbound type variable
///    for each generic parameter (α-renaming).
///
/// 3. **Create and track unbound variables**
///    Generates fresh type variables during inference when type information
///    is not yet available.
///
pub struct InferCx<'tcx> {
    /// Represents types context
    pub(crate) tcx: &'tcx mut TyCx,

    /// Mapping of type variable IDs to resolved types.
    type_variables: Arena<TyVar>,

    /// The currently active generic scopes.
    pub(crate) generics: Generics,
}

/// Implementation
impl<'tcx> InferCx<'tcx> {
    /// Creates new inference context
    ///
    /// # Parameters
    /// - `tcx: &'tcx mut TyCx`
    ///   Types context
    ///
    pub fn new(tcx: &'tcx mut TyCx) -> Self {
        Self {
            tcx,
            type_variables: Arena::new(),
            generics: Generics::default(),
        }
    }

    /// Creates a substitution pair in substitutions map
    ///
    /// # Parameters
    /// - `id: Id<TyVar>`
    ///   Type variable id, with what we need to creates substitution
    /// - `typ: Typ`
    ///   The type that we using to create substitution
    ///
    /// # Notes
    /// If substitution is already exists, this function
    /// isn't updating the already created substitution.
    ///
    pub fn substitute(&mut self, id: Id<TyVar>, typ: Typ) {
        let var = self.type_variables.get_mut(id).expect("invalid TyVar id");
        if let TyVar::Unbound = var {
            *var = TyVar::Bound(typ);
        }
    }

    /// Generates fresh unbound type variable.
    ///
    pub fn fresh(&mut self) -> Id<TyVar> {
        self.type_variables.alloc(TyVar::Unbound)
    }

    /// Generates fresh generic args vector
    pub fn mk_fresh_generics(&mut self, generics: &[EcoString]) -> GenericArgs {
        std::iter::repeat_with(|| Typ::Var(self.fresh()))
            .take(generics.len())
            .collect()
    }

    /// Generates fresh type variable bound to given type.
    ///
    pub fn bind(&mut self, to: Typ) -> Id<TyVar> {
        self.type_variables.alloc(TyVar::Bound(to))
    }

    /// Return immutable reference to the type variable by id
    ///
    pub fn get(&self, id: Id<TyVar>) -> &TyVar {
        self.type_variables.get(id).expect("invalid TyVar id")
    }

    /// Return mutable reference to the type variable by id
    ///
    pub fn get_mut(&mut self, id: Id<TyVar>) -> &mut TyVar {
        self.type_variables.get_mut(id).expect("invalid TyVar id")
    }

    /// Applies a substitutions for the given typ
    ///
    /// # Parameters
    /// - `typ: Typ`
    ///   The type that we using to apply substitution
    ///
    pub fn apply(&self, typ: Typ) -> Typ {
        match typ {
            Typ::Var(id) => match self.get(id) {
                TyVar::Unbound => typ,
                TyVar::Bound(typ) => typ.clone(),
            },
            Typ::Enum(id, args) => {
                Typ::Enum(id, args.into_iter().map(|it| self.apply(it)).collect())
            }
            Typ::Struct(id, args) => {
                Typ::Struct(id, args.into_iter().map(|it| self.apply(it)).collect())
            }
            Typ::FnDef(def, args) => {
                Typ::FnDef(def, args.into_iter().map(|it| self.apply(it)).collect())
            }
            other => other,
        }
    }

    /// Replaces `Generic(i)` with `args[i]` type
    pub fn subst(&self, typ: Typ, args: &GenericArgs) -> Typ {
        match typ {
            Typ::Generic(i) => args.get(i).cloned().unwrap_or(Typ::Generic(i)),
            Typ::Enum(id, inner_args) => Typ::Enum(
                id,
                inner_args
                    .into_iter()
                    .map(|a| self.subst(a, args))
                    .collect(),
            ),
            Typ::Struct(id, inner_args) => Typ::Struct(
                id,
                inner_args
                    .into_iter()
                    .map(|a| self.subst(a, args))
                    .collect(),
            ),
            Typ::FnDef(id, inner_args) => Typ::FnDef(
                id,
                inner_args
                    .into_iter()
                    .map(|a| self.subst(a, args))
                    .collect(),
            ),
            Typ::FnRef(params, ret) => Typ::FnRef(
                params.into_iter().map(|p| self.subst(p, args)).collect(),
                Box::new(self.subst(*ret.clone(), args)),
            ),
            other => other,
        }
    }
}
