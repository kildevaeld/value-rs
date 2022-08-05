use std::convert::Infallible;

use super::Arguments;
use crate::errors::IntoArgumentsError;
use value_types::IntoValue;

pub trait ToArguments {
    type Error: Into<IntoArgumentsError>;
    fn to_arguments(self) -> Result<Arguments, Self::Error>;
}

impl ToArguments for () {
    type Error = Infallible;
    fn to_arguments(self) -> Result<Arguments, Self::Error> {
        Ok(Arguments::default())
    }
}

impl ToArguments for Arguments {
    type Error = Infallible;
    fn to_arguments(self) -> Result<Arguments, Self::Error> {
        Ok(self)
    }
}

macro_rules! toargs {
    ($first: ident) => {
        impl<$first: IntoValue> ToArguments for ($first,)
        where
            $first::Error: std::error::Error + Send + Sync + 'static,
        {
            type Error = IntoArgumentsError;
            fn to_arguments(self) -> Result<Arguments, Self::Error> {
                Ok(Arguments::build().with(self.0).map_err(IntoArgumentsError::convert)?.build())
            }
        }
    };
    ($first: ident $($rest: ident)*) => {
        toargs!($($rest)*);

        impl<$first: IntoValue, $($rest: IntoValue),*> ToArguments for ($first, $($rest),*)
        where
            $first::Error: std::error::Error + Send + Sync + 'static,
            $(

                $rest::Error: std::error::Error + Send + Sync + 'static
            ),*
        {
            type Error = IntoArgumentsError;
            #[allow(non_snake_case)]
            fn to_arguments(self) -> Result<Arguments, Self::Error> {
                let mut args = Arguments::build();

                let ($first, $($rest),*) = self;

                args.add($first).map_err(IntoArgumentsError::convert)?;

                $(
                    args.add($rest).map_err(IntoArgumentsError::convert)?;
                )*

                Ok(args.build())

            }
        }
    }
}

toargs!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
