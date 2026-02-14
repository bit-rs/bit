/// Imports
use crate::{
    refs::{EnvRef, Ref},
    rt::env::Environment,
    rt::value::{Callable, Native, Value},
};
use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

/// Print definition
pub fn print() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, values| {
            print!("{}", values.get(0).unwrap());
            io::stdout().flush().unwrap();
            Value::Null
        }),
    });
}

/// Println definition
pub fn println() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, values| {
            println!("{}", values.get(0).unwrap());
            Value::Null
        }),
    });
}

/// Readln definition
pub fn readln() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|_, _| {
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line);
            Value::String(line)
        }),
    });
}

/// Provides env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();
    env.force_define("print", Value::Callable(Callable::Native(print())));
    env.force_define("println", Value::Callable(Callable::Native(println())));
    env.force_define("readln", Value::Callable(Callable::Native(readln())));
    Rc::new(RefCell::new(env))
}
