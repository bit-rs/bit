/// Imports
use id_arena::Id;
use std::collections::HashMap;
use tir::{
    def::{AdtDef, ItemDef, ModDef},
    ty::Ty,
};

/// Query resolution
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Res {
    /// Top-level definition
    Item(ItemDef),

    /// Module definition
    Mod(Id<ModDef>),

    /// Local type
    Local(Ty),
}

/// Module resolver
#[derive(Default)]
pub struct Resolver {
    /// Scopes stack
    scopes: Vec<HashMap<String, Ty>>,

    /// Module level definitions
    items: HashMap<String, ItemDef>,

    /// Imported modules
    mods: HashMap<String, Id<ModDef>>,
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

    /// Defines top-level item, returns true on success,
    /// returns false if item already defined
    pub fn define_item(&mut self, name: &str, def: ItemDef) -> bool {
        if self.items.contains_key(name) {
            false
        } else {
            self.items.insert(name.to_string(), def);
            true
        }
    }

    /// Defines module, returns true on success,
    /// returns false if item already defined
    pub fn define_mod(&mut self, name: &str, def: Id<ModDef>) -> bool {
        if self.mods.contains_key(name) {
            false
        } else {
            self.mods.insert(name.to_string(), def);
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

    /// Looks up top-level definition
    pub fn lookup_item(&self, name: &str) -> Option<ItemDef> {
        self.items.get(name).cloned()
    }

    /// Looks up module definition
    pub fn lookup_mod(&self, name: &str) -> Option<Id<ModDef>> {
        self.mods.get(name).cloned()
    }

    /// Looks up top-level adt definition
    pub fn lookup_adt(&self, name: &str) -> Option<Id<AdtDef>> {
        match self.items.get(name)?.kind {
            tir::def::ItemDefKind::Adt(id) => Some(id),
            tir::def::ItemDefKind::Fn(_) => None,
        }
    }

    /// Looks up local-level definition
    pub fn lookup_local(&self, name: &str) -> Option<Ty> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    /// Looks up local-level, top-level item or module
    pub fn lookup(&self, name: &str) -> Option<Res> {
        self.lookup_local(name)
            .map(Res::Local)
            .or_else(|| self.lookup_item(name).map(Res::Item))
            .or_else(|| self.lookup_mod(name).map(Res::Mod))
    }
}
