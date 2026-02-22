/// Imports
use id_arena::{Arena, Id};
use macros::bug;
use std::collections::HashMap;
use tir::{
    def::{AdtDef, FnDef, GenericParam},
    ty::{GenericArgs, Ty, TyVar},
};

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
    /// ADT arena
    pub adt: Arena<AdtDef>,

    /// Functions arena
    pub functions: Arena<FnDef>,
}

/// Implementation
impl TyCx {
    /// Allocated adt
    pub fn insert_adt(&mut self, adt: AdtDef) -> Id<AdtDef> {
        self.adt.alloc(adt)
    }

    /// Allocated function
    pub fn insert_fn(&mut self, adt: FnDef) -> Id<FnDef> {
        self.functions.alloc(adt)
    }

    /// Returns ADT by id
    pub fn adt(&mut self, id: Id<AdtDef>) -> &AdtDef {
        self.adt
            .get(id)
            .unwrap_or_else(|| bug!("adt not found by id."))
    }

    /// Returns function by id
    pub fn _fn(&mut self, id: Id<FnDef>) -> &FnDef {
        self.functions
            .get(id)
            .unwrap_or_else(|| bug!("fn not found by id."))
    }
}

/// Represents a stack-based context for managing generic type parameters
/// during type inference and hydration.
///
/// The `GenericsCx` structure maintains a stack of generic scopes.
/// Each scope (a `HashMap<usize>`) contains the names of generic type
/// parameters that are currently in scope and their ID's.
///
/// This allows the type checker and hydrator to correctly resolve
/// generic type names to their bound variables in nested or shadowed contexts.
///
/// # Fields
///
/// - `stack: Vec<HashMap<EcoString, usize>>` — A stack of scopes,
///   where each scope holds the names of generics currently active within that scope.
///
/// - `last_generic_id: usize` — Last generic id, used to
///   generate fresh UID's for generics.
///
/// # Notes
/// - contains method checks generic name
///   only in the last scope.
///
#[derive(Default, Debug)]
pub struct GenericsCx {
    stack: Vec<HashMap<String, usize>>,
    last_generic_id: usize,
}

/// Implementation
impl GenericsCx {
    /// Pushes the scope onto the stack
    /// and inserts given generic arguments
    /// in it.
    ///
    /// # Parameters
    /// - `generics: Vec<String>`
    ///   Generic parameter names.
    ///
    /// # Returns
    /// - `Vec<GenericParam>`
    ///   Created generic parameters info
    ///
    pub fn push_scope(&mut self, generics: Vec<String>) -> Vec<GenericParam> {
        let generics: HashMap<String, usize> =
            generics.into_iter().map(|g| (g, self.fresh())).collect();
        self.stack.push(generics.clone());
        generics
            .into_iter()
            .map(|g| GenericParam { name: g.0, id: g.1 })
            .collect()
    }

    /// Pushes a new scope consisting of **already constructed**
    /// generic parameters (usually reconstructed from a type).
    ///
    /// # Parameters
    /// - `generics: Vec<GenericParameter>`
    ///   Generic parameters.
    ///
    pub fn re_push_scope(&mut self, generics: Vec<GenericParam>) {
        self.stack
            .push(generics.into_iter().map(|g| (g.name, g.id)).collect());
    }

    /// Pops scope from the stack
    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }

    /// Returns generic ID by the name
    /// from the last scope, if generic exists.
    ///
    /// # Parameters
    /// - `name: &str`
    ///   Name of the generic
    ///
    pub fn get(&self, name: &str) -> Option<usize> {
        self.stack.last().and_then(|s| s.get(name).copied())
    }

    /// Generates fresh unique id
    /// for the generic type variable.
    ///
    #[inline]
    pub fn fresh(&mut self) -> usize {
        self.last_generic_id += 1;
        self.last_generic_id
    }

    /// Checks that generic is rigid
    #[must_use]
    #[inline]
    pub fn is_rigid(&self, id: usize) -> bool {
        self.stack
            .last()
            .is_some_and(|s| s.values().any(|g| g == &id))
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
/// - `tcx: &'mut TyCx`
///   An types context reference used to access ADT and functions
///
/// - `type_variables: Arena<TyVar>`
///   An arena that handles type variables used during the inference
///
/// - `generics: GenericsCx`
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

    /// Type variables arena
    type_variables: Arena<TyVar>,

    /// The currently active generic scopes.
    pub(crate) generics: GenericsCx,
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
            generics: GenericsCx::default(),
        }
    }

    /// Creates a substitution
    ///
    /// # Parameters
    /// - `id: Id<TyVar>`
    ///   Type variable id, with what we need to creates substitution
    /// - `ty: Ty`
    ///   The type that we using to create substitution
    ///
    /// # Notes
    /// If substitution is already exists, this function
    /// isn't updating the already created substitution.
    ///
    pub fn substitute(&mut self, id: Id<TyVar>, ty: Ty) {
        let var = self.type_variables.get_mut(id).expect("invalid TyVar id");
        if let TyVar::Unbound = var {
            *var = TyVar::Bound(ty);
        }
    }

    /// Generates fresh unbound type variable.
    ///
    pub fn fresh(&mut self) -> Id<TyVar> {
        self.type_variables.alloc(TyVar::Unbound)
    }
    /// Creates fresh vector of generic arguments
    /// with fresh type variables: `Ty::Var(TyVar::Unbound(...))`
    ///
    pub fn mk_fresh(&mut self, generics: &[GenericParam]) -> GenericArgs {
        FresheningCx::new(self).mk_generics(generics, Vec::new())
    }

    /// Creates fresh vector of generic arguments
    /// with fresh type variables:
    ///  `Ty::Var(TyVar::Unbound(...))`
    ///  *unless an explicit substitution is already provided*
    ///
    pub fn mk_fresh_m(&mut self, generics: &[GenericParam], m: &[Ty]) -> GenericArgs {
        FresheningCx::new(self).mk_generics(generics, m)
    }

    /// Generates fresh type variable bound to given type.
    ///
    pub fn bind(&mut self, to: Ty) -> Id<TyVar> {
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

    /// Applies a substitutions for the given type
    ///
    /// # Parameters
    /// - `ty: Ty`
    ///   The type that we using to apply substitution
    ///
    pub fn apply(&self, typ: Ty) -> Ty {
        match typ {
            Ty::Var(id) => match self.get(id) {
                TyVar::Unbound => typ,
                TyVar::Bound(typ) => typ.clone(),
            },
            Ty::Enum(id, args) => Typ::Enum(
                id,
                GenericArgs {
                    subtitutions: args
                        .subtitutions
                        .iter()
                        .map(|it| (*it.0, self.apply(it.1.clone())))
                        .collect(),
                },
            ),
            Typ::Struct(id, args) => Typ::Struct(
                id,
                GenericArgs {
                    subtitutions: args
                        .subtitutions
                        .iter()
                        .map(|it| (*it.0, self.apply(it.1.clone())))
                        .collect(),
                },
            ),
            Typ::Function(id, args) => Typ::Function(
                id,
                GenericArgs {
                    subtitutions: args
                        .subtitutions
                        .iter()
                        .map(|it| (*it.0, self.apply(it.1.clone())))
                        .collect(),
                },
            ),
            other => other,
        }
    }

    /// Checks that generic is rigid by its ID
    ///
    pub fn is_rigid(&self, id: usize) -> bool {
        self.generics.is_rigid(id)
    }
}

/// A temporary instantiation context used to replace generic types with
/// fresh inference variables.
///
/// This context performs the *α-renaming* (freshening) of generic parameters
/// when entering an instantiation site — for example, when calling a generic
/// function or constructing a generic struct/enum.
///
/// In practice, `FresheningCx` converts:
///
/// - `Typ::Generic(id)` → a fresh `Typ::Unbound(...)`
///   *unless an explicit substitution is already provided*
///
/// - recursively transforms function types, ADTs (`Struct`, `Enum`) and their
///   generic arguments.
///
/// The context stores two important pieces of data:
///
/// - A reference to the `InferCx`, used for allocating fresh
///   inference variables.
/// - A local `mapping: HashMap<usize, Typ>` that maps **generic parameter IDs**
///   to the *fresh inference variables* that now stand for them.
///
/// `FresheningCx` is short-lived: it exists only for the duration of a single
/// instantiation (e.g. one function call).
pub struct FresheningCx<'icx, 'tcx> {
    /// Reference to the inference cx
    icx: &'icx mut InferCx<'tcx>,

    /// A mapping of **generic parameter IDs** (`Generic(id)`) to the fresh
    /// inference variables created during this instantiation.
    ///
    /// This ensures that generic parameters remain consistent:
    /// `{g(n) -> u(m)}`, reused everywhere within a single instantiation.
    mapping: IndexMap<usize, Typ>,
}

/// Implementation
impl<'icx, 'tcx> FresheningCx<'icx, 'tcx> {
    /// Creates new freshening context
    pub fn new(icx: &'icx mut InferCx<'tcx>) -> Self {
        Self {
            icx,
            mapping: IndexMap::new(),
        }
    }

    /// Performs freshening of the type
    pub fn fresh(icx: &'icx mut InferCx<'tcx>, typ: Typ) -> Typ {
        let mut fcx = Self {
            icx,
            mapping: IndexMap::new(),
        };
        fcx.mk_ty(typ)
    }

    /// Creates new hydration context with given mapping
    pub fn fresh_m(icx: &'icx mut InferCx<'tcx>, typ: Typ, mapping: IndexMap<usize, Typ>) -> Typ {
        let mut fcx = Self { icx, mapping };
        fcx.mk_ty(typ)
    }

    /// Instantiates type by replacing
    /// Generic(id) -> Unbound($id)
    pub fn mk_ty(&mut self, t: Typ) -> Typ {
        match t {
            Typ::Prelude(_) | Typ::Unit | Typ::Var(_) => t,
            Typ::Generic(id) => {
                // If typ is already specified
                if let Some(typ) = self.mapping.get(&id) {
                    typ.clone()
                } else if self.icx.is_rigid(id) {
                    Typ::Generic(id)
                } else {
                    let fresh = Typ::Var(self.icx.fresh());
                    self.mapping.insert(id, fresh.clone());
                    fresh
                }
            }
            Typ::Function(id, args) => {
                let args = args
                    .subtitutions
                    .iter()
                    .map(|(k, v)| (*k, self.mk_ty(v.clone())))
                    .collect();
                let generics = self.mk_generics(&self.icx.tcx.function(id).generics.clone(), args);

                Typ::Function(id, generics)
            }
            Typ::Struct(id, args) => {
                let args = args
                    .subtitutions
                    .iter()
                    .map(|(k, v)| (*k, self.mk_ty(v.clone())))
                    .collect();
                let generics = self.mk_generics(&self.icx.tcx.struct_(id).generics.clone(), args);

                Typ::Struct(id, generics)
            }
            Typ::Enum(id, args) => {
                let args = args
                    .subtitutions
                    .iter()
                    .map(|(k, v)| (*k, self.mk_ty(v.clone())))
                    .collect();
                let generics = self.mk_generics(&self.icx.tcx.enum_(id).generics.clone(), args);

                Typ::Enum(id, generics)
            }
        }
    }

    /// Instantiates generics with args
    /// Generic(id) -> Unbound($id) | Given substitution
    pub fn mk_generics(
        &mut self,
        params: &[GenericParameter],
        args: IndexMap<usize, Typ>,
    ) -> GenericArgs {
        GenericArgs {
            subtitutions: params
                .iter()
                .map(|p| {
                    let generic_id = p.id;
                    match args.get(&generic_id) {
                        Some(s) => (generic_id, s.clone()),
                        None => (generic_id, Typ::Var(self.icx.fresh())),
                    }
                })
                .collect(),
        }
    }
}
