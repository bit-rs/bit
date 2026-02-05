/// Imports
use crate::{env::Environment, refs::EnvRef};
use std::cell::RefCell;

/// Interpreter
pub struct Interpreter {
    /// Current environment
    env: EnvRef,
}

/// Implementation
impl Interpreter {
    /// Creates new interpreter
    pub fn new() -> Self {
        Interpreter {
            env: EnvRef::new(RefCell::new(Environment::default())),
        }
    }
}
