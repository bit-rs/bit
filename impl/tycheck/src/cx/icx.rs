/// Imports
use crate::{
    cx::{generics::GenericsCx, tcx::TyCx},
    errors::ty::TypeError,
};
use id_arena::{Arena, Id};
use tir::ty::{GenericArgs, Ty, TyVar};

/// Inference context: manages type variables, substitutions, and generic
/// instantiation during Hindley-Milner–style type inference.
///
/// `InferCx` is the primary work-horse of the type-checker. It wraps a
/// mutable reference to `TyCx` and adds:
///
/// - **Fresh type variables** - call [`fresh`] to create a new `TyVar::Unbound`
///   and obtain its `Id<TyVar>`. Unbound variables act as inference holes.
///
/// - **Substitutions** - [`substitute`] binds an unbound variable to a
///   concrete type. [`apply`] walks a `Ty` tree and replaces every
///   `Ty::Var(id)` whose variable is bound with its bound type.
///
/// - **Unification** - [`unify`] attempts to make two types equal by
///   recording substitutions, failing with [`TypeError`] if the types are
///   structurally incompatible. An occurs check prevents infinite types.
///
/// - **Generic instantiation** - [`instantiate`] replaces `Ty::Generic(i)`
///   placeholders with concrete arguments.
///
/// - **Generic scoping** - the embedded [`GenericsCx`] tracks which generic
///   parameters are in scope so that rigid variables can be identified during
///   unification.
///
/// [`fresh`]: InferCx::fresh
/// [`substitute`]: InferCx::subst
/// [`apply`]: InferCx::apply
/// [`unify`]: InferCx::unify
/// [`instantiate`]: InferCx::instantiate
///
pub struct InferCx<'tcx> {
    /// Shared type-definition context for looking up ADTs and functions.
    pub(crate) tcx: &'tcx mut TyCx,

    /// Arena that owns all type variables created during inference.
    type_variables: Arena<TyVar>,

    /// Stack of in-scope generic parameter lists.
    pub(crate) generics: GenericsCx,
}

/// Implementation
impl<'tcx> InferCx<'tcx> {
    /// Creates a new inference context wrapping `tcx`.
    pub fn new(tcx: &'tcx mut TyCx) -> Self {
        Self {
            tcx,
            type_variables: Arena::new(),
            generics: GenericsCx::default(),
        }
    }

    /// Binds type variable `id` to `ty` if it is still unbound.
    ///
    /// If `id` is already bound, this call is a no-op — existing substitutions
    /// are never overwritten.
    pub fn subst(&mut self, id: Id<TyVar>, ty: Ty) {
        let var = self.type_variables.get_mut(id).expect("invalid TyVar id");
        if let TyVar::Unbound = var {
            *var = TyVar::Bound(ty);
        }
    }

    /// Allocates a new unbound type variable and returns its ID.
    pub fn fresh(&mut self) -> Id<TyVar> {
        self.type_variables.alloc(TyVar::Unbound)
    }

    /// Allocates a new bound type variable with
    /// given binding and returns its ID.
    pub fn fresh_bound(&mut self, to: Ty) -> Id<TyVar> {
        self.type_variables.alloc(TyVar::Bound(to))
    }

    /// Allocates one fresh unbound type variable per entry in `generics` and
    /// returns them as a [`GenericArgs`] vector. Used to instantiate a
    /// polymorphic definition at a call site.
    ///
    pub fn fresh_generics(&mut self, amount: usize) -> GenericArgs {
        (0..amount).map(|_| Ty::Var(self.fresh())).collect()
    }

    /// Returns an immutable reference to the type variable with `id`.
    ///
    /// # Panics
    /// Panics if `id` is not a valid arena index.
    ///
    pub fn var(&self, id: Id<TyVar>) -> &TyVar {
        self.type_variables.get(id).expect("invalid TyVar id")
    }

    /// Returns a mutable reference to the type variable with `id`.
    ///
    /// # Panics
    /// Panics if `id` is not a valid arena index.
    ///
    pub fn var_mut(&mut self, id: Id<TyVar>) -> &mut TyVar {
        self.type_variables.get_mut(id).expect("invalid TyVar id")
    }

    /// Applies the current substitution map to `ty`, replacing every bound
    /// `Ty::Var(id)` with its bound type. Unbound variables and all other
    /// type constructors are returned unchanged. The operation is shallow on
    /// unbound variables (it does not chase chains of bound variables
    /// recursively beyond one level for `Var`).
    ///
    pub fn apply(&self, ty: Ty) -> Ty {
        match ty {
            Ty::Var(id) => match self.var(id) {
                TyVar::Unbound => ty,
                TyVar::Bound(typ) => typ.clone(),
            },
            Ty::Adt(def, args) => Ty::Adt(def, args.into_iter().map(|it| self.apply(it)).collect()),
            Ty::FnDef(def, args) => {
                Ty::FnDef(def, args.into_iter().map(|it| self.apply(it)).collect())
            }
            other => other,
        }
    }

    /// Substitutes every `Ty::Generic(i)` in `ty` with `args[i]`.
    ///
    /// If `i` is out of bounds for `args` (which should not happen in
    /// well-formed code), the `Generic` is left unchanged.
    ///
    pub fn instantiate(&self, ty: Ty, args: &GenericArgs) -> Ty {
        match ty {
            Ty::Generic(i) => args.get(i).cloned().unwrap_or(Ty::Generic(i)),
            Ty::Adt(id, inner_args) => Ty::Adt(
                id,
                inner_args
                    .into_iter()
                    .map(|a| self.instantiate(a, args))
                    .collect(),
            ),
            Ty::FnDef(id, inner_args) => Ty::FnDef(
                id,
                inner_args
                    .into_iter()
                    .map(|a| self.instantiate(a, args))
                    .collect(),
            ),
            other => other,
        }
    }

    /// Attempts to make `t1` and `t2` equal by recording substitutions for
    /// unbound type variables.
    ///
    /// Substitutions applied before comparing using [`apply`], so partially
    /// solved variables are chased before any structural comparison.
    ///
    /// # Coercions
    /// A `Ty::FnDef` (a specific named function) unifies with a `Ty::FnRef`
    /// (a function-pointer signature) by instantiating the definition's
    /// parameter and return types and unifying them pairwise.
    ///
    /// # Errors
    /// Returns [`TypeError::Mismatch`] if the two types are structurally
    /// incompatible, [`TypeError::RigidMismatch`] if a rigid generic variable
    /// is unified with a different type, or [`TypeError::InfiniteType`] if the
    /// occurs check detects a cycle (e.g. `T = Vec<T>`).
    ///
    pub fn unify(&mut self, t1: Ty, t2: Ty) -> Result<(), TypeError> {
        // Applying substitutions
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        // Matching types
        match (t1, t2) {
            // Skipping errors
            (Ty::Error, _) | (_, Ty::Error) => Ok(()),

            // Same primitive types
            (Ty::Int, Ty::Int) => Ok(()),
            (Ty::Float, Ty::Float) => Ok(()),
            (Ty::Bool, Ty::Bool) => Ok(()),
            (Ty::String, Ty::String) => Ok(()),
            (Ty::Unit, Ty::Unit) => Ok(()),

            // Rigid generics: two identical generic indices unify; anything else is an error.
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

            // Function definitions, unifying args
            (Ty::FnDef(a_id, a_args), Ty::FnDef(b_id, b_args)) if a_id == b_id => {
                for (a, b) in a_args.into_iter().zip(b_args) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // Two function-pointer types: unify param lists and return type.
            (Ty::FnRef(a_sig), Ty::FnRef(b_sig)) => {
                if a_sig.params.len() != b_sig.params.len() {
                    return Err(TypeError::Mismatch(Ty::FnRef(a_sig), Ty::FnRef(b_sig)));
                }
                for (a, b) in a_sig.params.into_iter().zip(b_sig.params) {
                    self.unify(a, b)?;
                }
                self.unify(a_sig.ret, b_sig.ret)
            }

            // Implicit coercion: a named function definition unifies with a
            // compatible function-pointer signature.
            (Ty::FnDef(id, args), Ty::FnRef(sig)) | (Ty::FnRef(sig), Ty::FnDef(id, args)) => {
                let def = self.tcx._fn(id);
                let params: Vec<Ty> = def
                    .params
                    .iter()
                    .map(|p| self.instantiate(p.clone(), &args))
                    .collect();
                let ret = self.instantiate(def.ret.clone(), &args);
                for (a, b) in params.into_iter().zip(sig.params) {
                    self.unify(a, b)?;
                }
                self.unify(ret, sig.ret)
            }

            // Type variables: delegate to unify_var.
            (Ty::Var(id), ty) => self.unify_var(id, ty),
            (ty, Ty::Var(id)) => self.unify_var(id, ty),

            // Meta types: equal only if they carry identical metadata.
            (Ty::Meta(a), Ty::Meta(b)) if a == b => Ok(()),

            // All other combinations are hard mismatches.
            (t1, t2) => Err(TypeError::Mismatch(t1, t2)),
        }
    }

    /// Unifies type variable `id` with `ty`.
    ///
    /// - If `id` is already bound, the bound type is unified with `ty`.
    /// - If `id` is unbound, an occurs check is performed first to rule out
    ///   infinite types, then `id` is substituted with `ty`.
    ///
    fn unify_var(&mut self, id: Id<TyVar>, ty: Ty) -> Result<(), TypeError> {
        match self.var(id).clone() {
            // Variable already bound, unifying
            TyVar::Bound(bound) => self.unify(bound, ty),

            // Unbound variable
            TyVar::Unbound => {
                // Performing occurs check: restricts infinite types like `T = Vec<T>`
                if self.occurs(id, &ty) {
                    return Err(TypeError::InfiniteType);
                }
                self.subst(id, ty);
                Ok(())
            }
        }
    }

    /// Returns `true` if type variable `id` appears anywhere inside `ty`.
    ///
    /// Used by the occurs check in [`unify_var`] to prevent binding a variable
    /// to a type that contains itself, which would create an infinite type.
    ///
    fn occurs(&self, id: Id<TyVar>, ty: &Ty) -> bool {
        match ty {
            Ty::Var(other) => {
                if *other == id {
                    return true;
                }
                match self.var(*other) {
                    TyVar::Bound(inner) => self.occurs(id, inner),
                    _ => false,
                }
            }
            Ty::Adt(_, args) | Ty::FnDef(_, args) => args.iter().any(|a| self.occurs(id, a)),
            Ty::FnRef(sig) => {
                sig.params.iter().any(|a| self.occurs(id, a)) || self.occurs(id, &sig.ret)
            }
            _ => false,
        }
    }

    /// Returns a human-readable string representation of `ty`.
    ///
    /// Generic parameters are displayed using their declared names from the
    /// current [`GenericsCx`] scope when available, falling back to `T{i}`.
    /// Unresolved inference variables are shown as `_`.
    pub fn pretty(&self, ty: &Ty) -> String {
        match ty {
            Ty::Int => "Int".to_string(),
            Ty::Float => "Float".to_string(),
            Ty::Bool => "Bool".to_string(),
            Ty::String => "String".to_string(),
            Ty::Unit => "()".to_string(),
            Ty::Var(_) => "_".to_string(),
            Ty::Generic(id) => self
                .generics
                .name_of(*id)
                .unwrap_or_else(|| format!("T{id}")),
            Ty::Adt(id, args) => {
                let name = self.tcx.adt(*id).name().to_string();
                if args.is_empty() {
                    name
                } else {
                    let args = args
                        .iter()
                        .map(|a| self.pretty(a))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{name}<{args}>")
                }
            }
            Ty::FnDef(id, args) => {
                let def = self.tcx._fn(*id);
                let params = def
                    .params
                    .iter()
                    .map(|p| self.pretty(&self.instantiate(p.clone(), args)))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret = self.pretty(&def.ret);
                format!("fn({params}) -> {ret}")
            }
            Ty::FnRef(sig) => {
                let params = sig
                    .params
                    .iter()
                    .map(|p| self.pretty(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret = self.pretty(&sig.ret);
                format!("fn({params}) -> {ret}")
            }
            Ty::Meta(meta) => match meta {
                tir::ty::TyMeta::Module(_) => "Meta(Module)".to_string(),
                tir::ty::TyMeta::Adt(_) => "Meta(Adt)".to_string(),
                tir::ty::TyMeta::Variant(_, _) => "Meta(Variant)".to_string(),
            },
            Ty::Error => "Error".to_string(),
        }
    }

    /// Returns `true` if `ty` is a numeric type (`Int` or `Float`).
    pub fn is_numeric_ty(&self, ty: &Ty) -> bool {
        match ty {
            Ty::Int | Ty::Float => true,
            _ => false,
        }
    }

    /// Returns `true` if `ty` is exactly `Int`.
    pub fn is_int_ty(&self, ty: &Ty) -> bool {
        match ty {
            Ty::Int => true,
            _ => false,
        }
    }

    /// Returns `true` if `ty` is exactly `Bool`.
    pub fn is_bool_ty(&self, ty: &Ty) -> bool {
        matches!(ty, Ty::Bool)
    }
}
