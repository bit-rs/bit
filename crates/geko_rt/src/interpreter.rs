/// Imports
use crate::{builtins, env::Environment, error::RuntimeError, refs::EnvRef, value::Value};
use geko_common::bail;
use geko_lex::token::Span;
use std::cell::RefCell;

/// Interpreter
pub struct Interpreter {
    /// Builtins environment
    pub(crate) builtins: EnvRef,
    /// Current environment
    pub(crate) env: EnvRef,
}

/// Implementation
impl Interpreter {
    /// Creates new interpreter
    pub fn new() -> Self {
        Interpreter {
            builtins: builtins::provide_builtins(),
            env: EnvRef::new(RefCell::new(Environment::default())),
        }
    }

    /// Is truthy helper
    pub(crate) fn is_truthy(&self, span: &Span, value: &Value) -> bool {
        if let Value::Bool(bool) = value {
            bool.clone()
        } else {
            bail!(RuntimeError::ExpectedBool {
                value: value.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            })
        }
    }
}
