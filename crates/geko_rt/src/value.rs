/// Imports
use crate::refs::{EnvRef, MutRef, Ref};
use geko_ast::stmt::Block;
use std::{collections::HashMap, fmt::Display, rc::Rc};

/// Native function value
#[derive(Clone, Debug)]
pub struct Native {
    /// Function parameters arity
    pub arity: usize,
    /// Native function
    pub function: Box<fn(Vec<Value>) -> Value>,
}

/// Function value
#[derive(Clone, Debug)]
pub struct Function {
    /// Function parameters
    pub params: Vec<String>,
    /// Function block
    pub block: Block,
}

/// Closure function
#[derive(Clone, Debug)]
pub struct Closure {
    /// Function
    pub function: Ref<Function>,
    /// Environment
    pub environment: EnvRef,
}

/// User data type
#[derive(Clone, Debug)]
pub struct Type {
    /// Data type name
    pub name: String,
    /// Data type methods
    pub methods: HashMap<String, Ref<Function>>,
}

/// User data type instance
#[derive(Clone, Debug)]
pub struct Instance {
    /// Type of
    pub type_of: Type,
    /// Instance fields
    pub fields: HashMap<String, Value>,
}

/// Module
#[derive(Clone, Debug)]
pub struct Module {
    /// Module fields
    pub fields: HashMap<String, Value>,
}

/// Runtime value representation
#[derive(Clone, Debug)]
pub enum Value {
    /// Boolean value
    Bool(bool),
    /// Integer number value
    Int(i64),
    /// Float number value
    Float(f64),
    /// String value
    String(String),
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

/// Display implementation
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Matchin value
        match self {
            Value::Bool(val) => write!(f, "{val}"),
            Value::Int(int) => write!(f, "{int}"),
            Value::Float(float) => write!(f, "{float}"),
            Value::String(string) => write!(f, "{string}"),
            Value::Function(_) => write!(f, "Closure"),
            Value::Native(_) => write!(f, "Native"),
            Value::Type(typ) => write!(f, "Type({})", typ.borrow().name),
            Value::Module(_) => write!(f, "Module"),
            Value::Instance(instance) => write!(f, "Instance({})", instance.borrow().type_of.name),
            Value::Null => todo!(),
        }
    }
}

/// PartialEq implementation
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Function(a), Self::Function(b)) => Rc::ptr_eq(a, b),
            (Self::Native(a), Self::Native(b)) => Rc::ptr_eq(a, b),
            (Self::Type(a), Self::Type(b)) => Rc::ptr_eq(a, b),
            (Self::Module(a), Self::Module(b)) => Rc::ptr_eq(a, b),
            (Self::Instance(a), Self::Instance(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
