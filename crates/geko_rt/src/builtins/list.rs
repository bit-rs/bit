/// Imports
use crate::{
    builtins::utils,
    refs::{MutRef, Ref},
    rt::value::{Method, Native, Type, Value},
};
use std::{cell::RefCell, collections::HashMap};

/// Provides list type
pub fn provide_type() -> Ref<Type> {
    Ref::new(Type {
        name: "List".to_string(),
        methods: HashMap::from([
            // Init method
            (
                "init".to_string(),
                Method::Native(Ref::new(Native {
                    arity: 1,
                    function: Box::new(|_, _, values| {
                        let list = values.get(0).cloned().unwrap();
                        match list {
                            Value::Instance(instance) => {
                                let vec =
                                    Value::Any(MutRef::new(RefCell::new(Vec::<Value>::new())));

                                instance
                                    .borrow_mut()
                                    .fields
                                    .insert("$internal".to_string(), vec);

                                Value::Null
                            }
                            _ => unreachable!(),
                        }
                    }),
                })),
            ),
            // To string method
            (
                "to_string".to_string(),
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
                                    Value::Any(list) => {
                                        match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                                            Some(vec) => Value::String(format!("{vec:?}")),
                                            _ => utils::error(span, "corrupted list."),
                                        }
                                    }
                                    _ => {
                                        utils::error(span, "corrupted list.");
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }),
                })),
            ),
            // Push method
            (
                "push".to_string(),
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
                                    Value::Any(list) => {
                                        match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                                            Some(vec) => {
                                                vec.push(values.get(1).cloned().unwrap());
                                                Value::Null
                                            }
                                            _ => utils::error(span, "corrupted list."),
                                        }
                                    }
                                    _ => {
                                        utils::error(span, "corrupted list.");
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }),
                })),
            ),
        ]),
    })
}
