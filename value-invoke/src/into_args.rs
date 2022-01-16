use value::Value;

use crate::{error::Error, Arguments};

pub trait IntoArguments {
    fn into_arguments(self) -> Result<Arguments, Error>;
}

impl IntoArguments for Arguments {
    fn into_arguments(self) -> Result<Arguments, Error> {
        Ok(self)
    }
}

impl IntoArguments for Vec<Value> {
    fn into_arguments(self) -> Result<Arguments, Error> {
        Ok(self.into())
    }
}

impl IntoArguments for () {
    fn into_arguments(self) -> Result<Arguments, Error> {
        Ok(Arguments::empty())
    }
}

macro_rules! into_args {
    ($first: ident) => {
        impl<$first: serde::Serialize> IntoArguments for ($first, ) {
            #[allow(non_snake_case)]
            fn into_arguments(self) -> Result<Arguments, Error> {
                let ($first, ) = self;
                Ok(
                    vec![
                        value::to_value($first)?,
                    ].into()
                )
            }
        }
    };
    ($first: ident, $($trail: ident),*) => {
        into_args!( $($trail),*);

        impl<$first: serde::Serialize, $($trail: serde::Serialize),*> IntoArguments for ($first, $($trail),*) {
            #[allow(non_snake_case)]
            fn into_arguments(self) -> Result<Arguments, Error> {

                let ($first, $($trail),*) = self;

                Ok(
                    vec![
                        value::to_value($first)?,
                        $(
                            value::to_value($trail)?
                        ),*
                    ].into()
                )
            }
        }
    };


}

into_args!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
