/// Imports
use crate::{
    builtins::utils,
    refs::{MutRef, Ref},
    rt::value::{Method, Native, Type, Value},
};
use std::{cell::RefCell, collections::HashMap};

/// Init method
fn init_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    let vec = Value::Any(MutRef::new(RefCell::new(Vec::<Value>::new())));

                    instance
                        .borrow_mut()
                        .fields
                        .insert("$internal".to_string(), vec);

                    Value::Null
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// To string method
fn to_string_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    let internal = instance
                        .borrow_mut()
                        .fields
                        .get("$internal")
                        .cloned()
                        .unwrap();

                    match internal {
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                            Some(vec) => Value::String(format!("{vec:?}")),
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Push method
fn push_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    let internal = instance
                        .borrow_mut()
                        .fields
                        .get("$internal")
                        .cloned()
                        .unwrap();

                    match internal {
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                            Some(vec) => {
                                vec.push(values.get(1).cloned().unwrap());
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Get method
fn get_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    let internal = instance
                        .borrow_mut()
                        .fields
                        .get("$internal")
                        .cloned()
                        .unwrap();

                    match internal {
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                            Some(vec) => {
                                match values.get(1).cloned().unwrap() {
                                    Value::Int(idx) => {
                                        if idx >= 0 {
                                            vec[idx as usize].clone()
                                        } else {
                                            utils::error(span, "index should be positive int")
                                        }
                                    }
                                    _ => utils::error(span, "index should be an int"),
                                };
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Provides list type
pub fn provide_type() -> Ref<Type> {
    Ref::new(Type {
        name: "List".to_string(),
        methods: HashMap::from([
            // Init method
            ("init".to_string(), init_method()),
            // To string method
            ("to_string".to_string(), to_string_method()),
            // Push method
            ("push".to_string(), push_method()),
            // Get method
            ("get".to_string(), get_method()),
        ]),
    })
}
