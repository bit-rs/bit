/// Imports
use id_arena::{Arena, Id};
use macros::bug;
use tir::ty::{AdtDef, FnDef, ModDef};

/// Context for all type-level definitions used across compilation.
///
/// `TyCx` owns three arenas that store ADTs (structs/enums) definitions, function
/// definitions, and module definitions. Every definition is heap-allocated
/// inside its arena and identified by a typed `Id<T>`.
///
/// The context is expected to be created once and kept alive for the full
/// duration of type-checking and all subsequent compilation phases.
///
#[derive(Default)]
pub struct TyCx {
    /// Storage for all algebraic data type definitions (structs and enums).
    pub adt: Arena<AdtDef>,

    /// Storage for all function definitions.
    pub functions: Arena<FnDef>,

    /// Storage for all module definitions.
    pub modules: Arena<ModDef>,
}

/// Implementation
impl TyCx {
    /// Inserts an ADT definition into the arena and returns its fresh ID.
    pub fn insert_adt(&mut self, adt: AdtDef) -> Id<AdtDef> {
        self.adt.alloc(adt)
    }

    /// Inserts a function definition into the arena and returns its fresh ID.
    pub fn insert_fn(&mut self, adt: FnDef) -> Id<FnDef> {
        self.functions.alloc(adt)
    }

    /// Inserts a module definition into the arena and returns its fresh ID.
    pub fn insert_mod(&mut self, m: ModDef) -> Id<ModDef> {
        self.modules.alloc(m)
    }

    /// Returns a reference to the ADT definition with the given ID.
    ///
    /// # Panics
    /// Panics (via the `bug!` macro) if `id` does not correspond to any
    /// allocated ADT definition. This indicates a compiler bug.
    ///
    pub fn adt(&self, id: Id<AdtDef>) -> &AdtDef {
        self.adt
            .get(id)
            .unwrap_or_else(|| bug!("adt not found by id."))
    }

    /// Returns a reference to the function definition with the given ID.
    ///
    /// # Panics
    /// Panics (via the `bug!` macro) if `id` does not correspond to any
    /// allocated function definition. This indicates a compiler bug.
    ///
    pub fn _fn(&self, id: Id<FnDef>) -> &FnDef {
        self.functions
            .get(id)
            .unwrap_or_else(|| bug!("fn not found by id."))
    }

    /// Returns a reference to the module definition with the given ID.
    ///
    /// # Panics
    /// Panics (via the `bug!` macro) if `id` does not correspond to any
    /// allocated module definition. This indicates a compiler bug.
    ///
    pub fn _mod(&self, id: Id<ModDef>) -> &ModDef {
        self.modules
            .get(id)
            .unwrap_or_else(|| bug!("module not found by id."))
    }

    /// Returns a mutable reference to the ADT definition with the given ID.
    ///
    /// # Panics
    /// Panics (via the `bug!` macro) if `id` does not correspond to any
    /// allocated ADT definition. This indicates a compiler bug.
    ///
    pub fn adt_mut(&mut self, id: Id<AdtDef>) -> &mut AdtDef {
        self.adt
            .get_mut(id)
            .unwrap_or_else(|| bug!("adt not found by id."))
    }

    /// Returns a mutable reference to the function definition with the given ID.
    ///
    /// # Panics
    /// Panics (via the `bug!` macro) if `id` does not correspond to any
    /// allocated function definition. This indicates a compiler bug.
    ///
    pub fn fn_mut(&mut self, id: Id<FnDef>) -> &mut FnDef {
        self.functions
            .get_mut(id)
            .unwrap_or_else(|| bug!("fn not found by id."))
    }

    /// Returns a mutable reference to the module definition with the given ID.
    ///
    /// # Panics
    /// Panics (via the `bug!` macro) if `id` does not correspond to any
    /// allocated module definition. This indicates a compiler bug.
    ///
    pub fn mod_mut(&mut self, id: Id<ModDef>) -> &mut ModDef {
        self.modules
            .get_mut(id)
            .unwrap_or_else(|| bug!("module not found by id."))
    }
}
