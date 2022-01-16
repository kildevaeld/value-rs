use crate::{Validation, ValidationBox, Validator};
use alloc::{boxed::Box, vec, vec::Vec};

pub trait Validatable {
    fn validator() -> Validator;
}

pub trait ValidationList {
    fn into_list(self) -> Vec<ValidationBox>;
}

impl ValidationList for Vec<ValidationBox> {
    fn into_list(self) -> Vec<ValidationBox> {
        self
    }
}

macro_rules! validation_list {
    ($type:ident) => {
        impl<S> ValidationList for (S,)
        where
            S: Validation + 'static,
        {
            fn into_list(self) -> Vec<ValidationBox> {
                vec![Box::new(self.0)]
            }
        }
    };
    ($type1:ident, $( $type:ident ),*) => {
        validation_list!($( $type ),*);

        impl<$type1: Validation + 'static, $( $type: Validation + 'static ),*> ValidationList for ($type1, $($type),*) {
            fn into_list(self) -> Vec<ValidationBox> {
                #[allow(non_snake_case)]
                let ($type1, $( $type ),*) = self;
                vec![
                    Box::new($type1), $( Box::new($type) ),*
                ]
            }
        }

    };
}

validation_list! {
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16
}
