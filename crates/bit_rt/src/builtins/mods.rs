/// Imports
use crate::{
    builtins::{is, math, env},
    refs::MutRef,
    rt::value::Module,
};
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
    modules.insert(
        "is".to_string(),
        MutRef::new(RefCell::new(Module {
            env: is::provide_env(),
        })),
    );
    modules.insert(
        "env".to_string(),
        MutRef::new(RefCell::new(Module {
            env: env::provide_env(),
        })),
    );
    modules
}
