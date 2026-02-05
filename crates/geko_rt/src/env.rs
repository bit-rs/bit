/// Imports
use crate::{error::RuntimeError, refs::EnvRef, value::Value};
use geko_common::bail;
use geko_lex::token::Span;
use std::collections::HashMap;

/// Variables environment
#[derive(Default)]
pub struct Environment {
    /// Variables map
    variables: HashMap<String, Value>,
    /// Enclosing
    enclosing: Option<EnvRef>,
}

/// Implementation
impl Environment {
    /// Looks up a variable
    pub fn lookup(&self, span: &Span, name: &str) -> Value {
        match self.variables.get(name) {
            Some(it) => it.clone(),
            None => match &self.enclosing {
                Some(env) => env.borrow().lookup(span, name),
                None => bail!(RuntimeError::UndefinedVariable {
                    name: name.to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
        }
    }

    /// Sets a variable value
    pub fn set(&mut self, span: &Span, name: &str, value: Value) {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
        } else {
            match &self.enclosing {
                Some(env) => env.borrow_mut().set(span, name, value),
                None => bail!(RuntimeError::UndefinedVariable {
                    name: name.to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            }
        }
    }

    /// Defines a variable
    pub fn define(&mut self, span: &Span, name: &str, value: Value) {
        if self.variables.contains_key(name) {
            bail!(RuntimeError::AlreadyDefinedVariable {
                name: name.to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            })
        } else {
            self.variables.insert(name.to_string(), value);
        }
    }

    /// Forcely defines a variable
    pub fn force_define(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
}
