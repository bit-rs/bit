/// Imports
use crate::{
    env::Environment,
    refs::{EnvRef, Ref},
    value::{Callable, Native, Value},
};
use std::cell::RefCell;

/// Println definition
pub fn println() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|values| {
            println!("{}", values.get(0).unwrap());
            Value::Null
        }),
    });
}

/// Provides builtins
pub fn provide_builtins() -> EnvRef {
    let mut env = Environment::default();
    env.force_define("println", Value::Callable(Callable::Native(println())));
    EnvRef::new(RefCell::new(env))
}
