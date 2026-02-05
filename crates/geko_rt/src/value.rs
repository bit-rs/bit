/// Imports
use crate::refs::{EnvRef, MutRef, Ref};
use geko_ast::stmt::Block;
use std::collections::HashMap;

/// Native function value
#[derive(Clone)]
pub struct Native {
    /// Function parameters
    pub params: Vec<String>,
    /// Native function
    pub function: Box<fn(Vec<&Value>) -> Value>,
}

/// Function value
#[derive(Clone)]
pub struct Function {
    /// Function parameters
    pub params: Vec<String>,
    /// Function block
    pub block: Block,
}

/// Closure function
#[derive(Clone)]
pub struct Closure {
    /// Function
    function: Ref<Function>,
    /// Environment
    environment: EnvRef,
}

/// User data type
#[derive(Clone)]
pub struct Type {
    /// Data type methods
    methods: HashMap<String, Ref<Function>>,
}

/// User data type instance
#[derive(Clone)]
pub struct Instance {
    /// Type of
    type_of: Type,
    /// Instance fields
    fields: HashMap<String, Value>,
}

/// Module
#[derive(Clone)]
pub struct Module {
    /// Module fields
    fields: HashMap<String, Value>,
}

/// Runtime value representation
#[derive(Clone)]
pub enum Value {
    /// Boolean value
    Bool(bool),
    /// Integer number value
    Int(i64),
    /// Float number value
    Float(f64),
    /// Function value
    Function(Ref<Closure>),
    /// Native function
    Native(Ref<Native>),
    /// Meta type
    Type(MutRef<Type>),
    /// Module
    Module(MutRef<Module>),
    /// Type instance
    Instance(MutRef<Instance>),
    /// Null reference
    Null,
}
