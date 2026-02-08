/// Imports
use crate::{refs::MutRef, value::Module};
use camino::Utf8PathBuf;
use std::collections::HashMap;

/// Modules registry
#[derive(Default)]
pub struct Modules {
    /// Imported / loaded modules
    modules: HashMap<Utf8PathBuf, MutRef<Module>>,
}

/// Implementation
impl Modules {
    /// Gets module by path
    pub fn get(&self, path: &Utf8PathBuf) -> Option<MutRef<Module>> {
        self.modules.get(path).cloned()
    }

    /// Sets module by path
    pub fn set(&mut self, path: Utf8PathBuf, module: MutRef<Module>) {
        self.modules.insert(path, module);
    }
}
