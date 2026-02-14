/// Imports
use crate::{builtins::math, refs::MutRef, rt::value::Module};
use std::{cell::RefCell, collections::HashMap};

/// Provides modules
pub fn provide_modules() -> HashMap<String, MutRef<Module>> {
    let mut modules = HashMap::new();
    modules.insert(
        "math".to_string(),
        MutRef::new(RefCell::new(Module {
            env: math::provide_env(),
        })),
    );
    modules
}
