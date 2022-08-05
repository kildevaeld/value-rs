use super::Arguments;
use crate::{errors::ArgumentError, Parameters};
use std::convert::Infallible;
use value_types::{FromValueRef, HasType};

pub trait FromArguments<'a>: Sized + Send {
    type Error: Into<ArgumentError>;
    fn from_arguments(args: &'a Arguments) -> Result<Self, Self::Error>;

    fn parameters() -> Parameters;
}

impl<'a> FromArguments<'a> for () {
    type Error = Infallible;
    fn from_arguments(_args: &'a Arguments) -> Result<Self, Self::Error> {
        Ok(())
    }

    fn parameters() -> Parameters {
        Parameters::default()
    }
}

macro_rules! count {
    (@step $idx: expr, $args:expr, $type1:ident, $( $type:ident ),*) => {

        let $type1 = $args.try_get_ref::<$type1>($idx)?;
        count!(@step $idx + 1usize, $args, $($type),*);
    };

    (@step $idx: expr, $args:expr, $type1:ident) => {
        let $type1 = $args.try_get_ref::<$type1>($idx)?;
    };

    (@step $_idx:expr, $args: expr,) => {};
}

macro_rules! arguments {
    ($first: ident) => {
        impl<'a, $first: FromValueRef<'a> + HasType + Send> FromArguments<'a> for ($first,)
        where
            $first::Error: Into<ArgumentError>
        {
            type Error = ArgumentError;
            fn from_arguments(args: &'a Arguments) -> Result<Self, Self::Error> {
                Ok((args.try_get_ref::<$first>(0).map_err(ArgumentError::unknown)?,))
            }

            fn parameters() -> Parameters {
                Parameters::build().with($first::typed()).build()
            }
        }
    };

    ($first: ident $($rest: ident)*) => {

        arguments!($($rest)*);

        impl<'a, $first: FromValueRef<'a> + HasType + Send, $($rest: FromValueRef<'a> + HasType + Send),*> FromArguments<'a> for ($first,$($rest),*)
        where
            $first::Error: Into<ArgumentError>,
            $(
                $rest::Error: Into<ArgumentError>
            ),*
        {
            type Error = ArgumentError;
            #[allow(non_snake_case)]
            fn from_arguments(args: &'a Arguments) -> Result<Self, Self::Error> {

                count!(@step 0, args, $first, $($rest),*);

                Ok((
                    $first, $($rest),*
                ))
            }

            fn parameters() -> Parameters {
               let mut params = Parameters::build();
               params.add($first::typed());
               $(
                params.add($rest::typed());
               )*

               params.build()
            }
        }
    };
}

arguments!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);
