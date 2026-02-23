use crate::{refs::Ref, rt::value::Type};

pub fn provide_type() -> Ref<Type> {
    RustType::new()
        .with_methods(vec![
            ||
        ])
}