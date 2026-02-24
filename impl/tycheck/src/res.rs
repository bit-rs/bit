/// Imports
use id_arena::Id;
use std::collections::HashMap;
use tir::{
    def::{AdtDef, FnDef},
    ty::Ty,
};

/// Resolution of item
#[derive(Clone)]
pub enum Res {
    /// Local resolution
    Local(Ty),

    /// Module definition
    Def(Def),
}

/// Module definitions
#[derive(Clone)]
pub enum Def {
    /// Function definition
    Function(Id<FnDef>),

    /// Data type definition
    Adt(Id<AdtDef>),
}

/// Module resolver
#[derive(Default)]
pub struct Resolver {
    /// Scopes stack
    scopes: Vec<HashMap<String, Ty>>,

    /// Module level definitions
    defs: HashMap<String, Def>,
}

/// Implementation
impl Resolver {
    /// Pushes a new scope onto the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pops the current scope from the stack
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Resolves local variable
    pub fn resolve_local(&self, name: &str) -> Option<Ty> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    /// Resolves module definition
    pub fn resolve_def(&self, name: &str) -> Option<Def> {
        self.defs.get(name).cloned()
    }

    /// Defines module-level definition, returns true on success,
    /// returns false if item already defined
    pub fn define_def(&mut self, name: &str, def: Def) -> bool {
        if self.defs.contains_key(name) {
            false
        } else {
            self.defs.insert(name.to_string(), def);
            true
        }
    }

    /// Defines scope-level definition, returns true on success,
    /// returns false if item already defined
    pub fn define_local(&mut self, name: &str, ty: Ty) -> bool {
        match self.scopes.last_mut() {
            Some(scope) => {
                if scope.contains_key(name) {
                    false
                } else {
                    scope.insert(name.to_string(), ty);
                    true
                }
            }
            None => false,
        }
    }

    /// Resolves item
    pub fn resolve(&self, name: &str) -> Option<Res> {
        self.resolve_local(name)
            .map(Res::Local)
            .or_else(|| self.resolve_def(name).map(Res::Def))
    }
}
