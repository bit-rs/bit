use crate::{env::Environment, refs::EnvRef, value::Value};

/// Println definition
pub fn println(args: Vec<&Value>) -> Value {
    Value::Null
}

/// Provides builtins
pub fn provide_builtins() -> EnvRef {
    let env = Environment::default();
    env.force_define("println", );
    EnvRef::new(env)
}