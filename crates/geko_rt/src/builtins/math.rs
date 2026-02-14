/// Imports
use crate::{
    refs::{EnvRef, Ref},
    rt::env::Environment,
    rt::value::{Callable, Native, Value},
};
use std::{cell::RefCell, rc::Rc};

/// Math sin
fn sin() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::sin(*int as f64)),
            Value::Float(float) => Value::Float(f64::sin(*float)),
            _ => panic!("not a number."),
        }),
    });
}

/// Math cos
fn cos() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::cos(*int as f64)),
            Value::Float(float) => Value::Float(f64::cos(*float)),
            _ => panic!("not a number."),
        }),
    });
}

/// Provides math module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();
    env.force_define("sin", Value::Callable(Callable::Native(sin())));
    env.force_define("cos", Value::Callable(Callable::Native(cos())));
    Rc::new(RefCell::new(env))
}
