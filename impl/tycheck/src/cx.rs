/// Imports
use id_arena::{Arena, Id};
use macros::bug;
use std::collections::HashMap;
use tir::{
    def::{AdtDef, FnDef, GenericParam},
    ty::{GenericArgs, Ty, TyVar},
};

use crate::errors::TypeError;

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
    pub fn adt(&self, id: Id<AdtDef>) -> &AdtDef {
        self.adt
            .get(id)
            .unwrap_or_else(|| bug!("adt not found by id."))
    }

    /// Returns function by id
    pub fn _fn(&self, id: Id<FnDef>) -> &FnDef {
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

    /// Generates fresh int type variable.
    ///
    pub fn fresh_int(&mut self) -> Id<TyVar> {
        self.type_variables.alloc(TyVar::Int)
    }

    /// Generates fresh float type variable.
    ///
    pub fn fresh_float(&mut self) -> Id<TyVar> {
        self.type_variables.alloc(TyVar::Float)
    }

    /// Creates fresh vector of generic arguments
    /// with fresh type variables: `Ty::Var(TyVar::Unbound(...))`
    ///
    pub fn mk_fresh(&mut self, generics: &[GenericParam]) -> GenericArgs {
        (0..generics.len()).map(|_| Ty::Var(self.fresh())).collect()
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
    pub fn apply(&self, ty: Ty) -> Ty {
        match ty {
            Ty::Var(id) => match self.get(id) {
                TyVar::Unbound | TyVar::Float | TyVar::Int => ty,
                TyVar::Bound(typ) => typ.clone(),
            },
            Ty::Adt(def, args) => Ty::Adt(def, args.into_iter().map(|it| self.apply(it)).collect()),
            Ty::Fn(def, args) => Ty::Fn(def, args.into_iter().map(|it| self.apply(it)).collect()),
            other => other,
        }
    }

    /// Checks that generic is rigid by its ID
    ///
    pub fn is_rigid(&self, id: usize) -> bool {
        self.generics.is_rigid(id)
    }

    /// Returns substituted field if type is struct
    pub fn field(&self, ty: Ty, name: String) -> Option<Ty> {
        match ty {
            Ty::Adt(id, generics) => match self.tcx.adt(id) {
                AdtDef::Struct(s) => match s.fields.iter().find(|f| f.name == name) {
                    Some(f) => Some(self.subst(f.ty.clone(), &generics)),
                    None => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    /// Replaces `Generic(i)` with `args[i]` type
    pub fn subst(&self, ty: Ty, args: &GenericArgs) -> Ty {
        match ty {
            Ty::Generic(i) => args.get(i).cloned().unwrap_or(Ty::Generic(i)),
            Ty::Adt(id, inner_args) => Ty::Adt(
                id,
                inner_args
                    .into_iter()
                    .map(|a| self.subst(a, args))
                    .collect(),
            ),
            Ty::Fn(id, inner_args) => Ty::Fn(
                id,
                inner_args
                    .into_iter()
                    .map(|a| self.subst(a, args))
                    .collect(),
            ),
            Ty::Ref(inner) => Ty::Ref(Box::new(self.subst(*inner, args))),
            Ty::MutRef(inner) => Ty::MutRef(Box::new(self.subst(*inner, args))),
            other => other,
        }
    }

    /// Unifies two types
    pub fn unify(&mut self, t1: Ty, t2: Ty) -> Result<(), TypeError> {
        // Applying substitutions
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        // Matching types
        match (t1, t2) {
            // Same primitive types
            (Ty::Int(a), Ty::Int(b)) if a == b => Ok(()),
            (Ty::UInt(a), Ty::UInt(b)) if a == b => Ok(()),
            (Ty::Float(a), Ty::Float(b)) if a == b => Ok(()),
            (Ty::Bool, Ty::Bool) => Ok(()),
            (Ty::Char, Ty::Char) => Ok(()),
            (Ty::String, Ty::String) => Ok(()),
            (Ty::Unit, Ty::Unit) => Ok(()),

            // Rigid generics
            (Ty::Generic(a), Ty::Generic(b)) if a == b => Ok(()),
            (Ty::Generic(_), other) | (other, Ty::Generic(_)) => {
                Err(TypeError::RigidMismatch(other))
            }

            // ADT, unifying args
            (Ty::Adt(a_id, a_args), Ty::Adt(b_id, b_args)) if a_id == b_id => {
                for (a, b) in a_args.into_iter().zip(b_args) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // References
            (Ty::Ref(a), Ty::Ref(b)) => self.unify(*a, *b),
            (Ty::MutRef(a), Ty::MutRef(b)) => self.unify(*a, *b),

            // Functions, unifying args
            (Ty::Fn(a_id, a_args), Ty::Fn(b_id, b_args)) if a_id == b_id => {
                for (a, b) in a_args.into_iter().zip(b_args) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // Type variables
            (Ty::Var(id), ty) => self.unify_var(id, ty),
            (ty, Ty::Var(id)) => self.unify_var(id, ty),

            // Anything else
            (t1, t2) => Err(TypeError::Mismatch(t1, t2)),
        }
    }

    /// Unifies type variable and type
    fn unify_var(&mut self, id: Id<TyVar>, ty: Ty) -> Result<(), TypeError> {
        match self.get(id).clone() {
            // Variable already bound, unifying
            TyVar::Bound(bound) => self.unify(bound, ty),

            // Int literal, ty should be int variant
            TyVar::Int => match ty {
                Ty::Int(_) | Ty::UInt(_) => {
                    self.substitute(id, ty);
                    Ok(())
                }
                Ty::Var(other) => {
                    self.substitute(other, Ty::Var(id));
                    Ok(())
                }
                other => Err(TypeError::Mismatch(Ty::Var(id), other)),
            },

            // Float literal, ty should be float variant
            TyVar::Float => match ty {
                Ty::Float(_) => {
                    self.substitute(id, ty);
                    Ok(())
                }
                Ty::Var(other) => {
                    self.substitute(other, Ty::Var(id));
                    Ok(())
                }
                other => Err(TypeError::Mismatch(Ty::Var(id), other)),
            },

            // Unbound variable
            TyVar::Unbound => {
                // Performing occurs check: restricts infinite types like `T = Vec<T>`
                if self.occurs(id, &ty) {
                    return Err(TypeError::InfiniteType);
                }
                self.substitute(id, ty);
                Ok(())
            }
        }
    }

    /// Occurs check: is it an `id` variable inside ty?
    fn occurs(&self, id: Id<TyVar>, ty: &Ty) -> bool {
        match ty {
            Ty::Var(other) => {
                if *other == id {
                    return true;
                }
                match self.get(*other) {
                    TyVar::Bound(inner) => self.occurs(id, inner),
                    _ => false,
                }
            }
            Ty::Adt(_, args) | Ty::Fn(_, args) => args.iter().any(|a| self.occurs(id, a)),
            Ty::Ref(inner) | Ty::MutRef(inner) => self.occurs(id, inner),
            _ => false,
        }
    }
}
