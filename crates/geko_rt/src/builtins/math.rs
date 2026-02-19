use geko_common::bail;

/// Imports
use crate::{
    error::RuntimeError,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use std::{cell::RefCell, rc::Rc};

/// Math sin
fn sin() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::sin(*int as f64)),
            Value::Float(float) => Value::Float(f64::sin(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math cos
fn cos() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::cos(*int as f64)),
            Value::Float(float) => Value::Float(f64::cos(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math sqrt
fn sqrt() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::sqrt(*int as f64)),
            Value::Float(float) => Value::Float(f64::sqrt(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math cbrt
fn cbrt() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::cbrt(*int as f64)),
            Value::Float(float) => Value::Float(f64::cbrt(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math pow
fn pow() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            // Int pow
            Value::Int(a) => match values.get(1).unwrap() {
                // Int exp
                Value::Int(b) => {
                    use std::convert::TryInto;

                    // Positive exponent
                    if *b >= 0 {
                        // Safe cast
                        let b_u32: u32 = (*b).try_into().unwrap_or_else(|_| {
                            bail!(RuntimeError::Bail {
                                text: format!("exponent {} is too large", b),
                                src: span.0.clone(),
                                span: span.1.clone().into(),
                            })
                        });

                        match a.checked_pow(b_u32) {
                            Some(result) => Value::Int(result),
                            None => bail!(RuntimeError::Bail {
                                text: "int overflow in exp".to_string(),
                                src: span.0.clone(),
                                span: span.1.clone().into()
                            }),
                        }
                    }
                    // Negative exponent
                    else {
                        // Safe cast
                        let b_i32: i32 = (*b).try_into().unwrap_or_else(|_| {
                            bail!(RuntimeError::Bail {
                                text: format!("exponent {} is too small", b),
                                src: span.0.clone(),
                                span: span.1.clone().into(),
                            })
                        });

                        Value::Float((*a as f64).powi(b_i32))
                    }
                }
                // Float exp
                Value::Float(b) => Value::Float((*a as f64).powf(*b)),
                // Otherwise, raising error
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            // Float pow
            Value::Float(a) => match values.get(1).unwrap() {
                // Int exp
                Value::Int(b) => Value::Float(a.powi(*b as i32)),
                // Float exp
                Value::Float(b) => Value::Float(a.powf(*b)),
                // Otherwise, raising error
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Provides math module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();
    env.force_define("sin", Value::Callable(Callable::Native(sin())));
    env.force_define("cos", Value::Callable(Callable::Native(cos())));
    env.force_define("sqrt", Value::Callable(Callable::Native(sqrt())));
    env.force_define("cbrt", Value::Callable(Callable::Native(cbrt())));
    env.force_define("pow", Value::Callable(Callable::Native(pow())));
    Rc::new(RefCell::new(env))
}
