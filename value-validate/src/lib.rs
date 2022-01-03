#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(not(feature = "std"))]
extern crate alloc;

mod error;
mod object;
mod validation;

pub use self::{error::*, object::*, validation::*};

// use std::collections::BTreeMap;
// use validation::{TypeValidator, Validation};
use value::{NumberType, Value, ValueType};

pub fn string() -> TypedValidator {
    TypedValidator::new(ValueType::String)
}

pub fn number() -> TypedValidator {
    TypedValidator::new(ValueType::Number(NumberType::Any))
}

pub fn bool() -> TypedValidator {
    TypedValidator::new(ValueType::Bool)
}

pub fn object() -> ObjectValidator {
    ObjectValidator::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    use validation::*;
    use value::value;

    #[test]
    fn test() {
        let o = object()
            .field(
                "name",
                string().and(oneof(vec![
                    //
                    "Rasmus".into(),
                    "Willburg".into(),
                ])),
            )
            // .field(
            //     "person",
            //     object().field("name", string().and(min(5)).and(max(100))),
            // )
            .field("age", number());

        o.validate(&value!({
            "name": "Rasmus",
            "age": 2
        }))
        .expect("Hello");

        // println!("serde {}", serde_json::to_string_pretty(&o).unwrap());
    }
}
