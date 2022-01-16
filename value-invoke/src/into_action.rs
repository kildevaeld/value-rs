// #![allow(non_snake_case)]

use std::{future::Future, marker::PhantomData};

use futures_core::TryFuture;
use value::Value;
use value_validate::Validatable;

use crate::{
    action::{action_box, Action, ActionBox},
    types::{Arguments, Parameter, Parameters},
};

//

pub struct ActionFn<F> {
    params: Parameters,
    func: F,
}

impl<F, U> Action for ActionFn<F>
where
    F: Fn(Arguments) -> U + Send + Sync,
    U: TryFuture + Send,
    U: Future<Output = Result<Value, U::Error>>,
{
    type Error = U::Error;
    type Future = U;
    fn parameters(&self) -> &crate::types::Parameters {
        &self.params
    }

    fn call(&self, args: Arguments) -> Self::Future {
        (self.func)(args)
    }
}

//

// pub struct ActionInto<F, A> {
//     func: F,
//     _a: PhantomData<A>,
// }

// macro_rules! action_impl {
//     ($type:ident) => {
//         impl<F, $type, U> Action for ActionInto<F, ($type,)>
//         where
//         F: Fn() -> U + Send + Sync + 'static,
//         U: TryFuture + Send,
//         U::Error: std::error::Error + Send + Sync + 'static,
//         U::Ok: serde::Serialize,
//         U: Future<Output = Result<U::Ok, U::Error>> + Send,
//         {
//             type Error = U::Error;
//             type Future = BoxFuture<'static, Result<Value, Self::Error>>;

//             fn parameters(&self) -> &Parameters {
//                 let params = Parameters::default().add(Parameter::new($type::validator()));
//                 params
//             }

//             fn call(&self, args: Arguments) -> Self::Future {
//                 let future = self.action.call(args);
//                 Box::pin(async move {
//                     match future.await {
//                         Ok(ret) => Ok(ret),
//                         Err(err) => Err(box_error(err)),
//                     }
//                 })
//             }
//         }
//     }
//     ($type1:ident, $( $type:ident ),*) => {
//         action_impl!($($type),*);

//     };
// }

pub trait IntoAction<A> {
    type Action;
    fn action(self) -> Self::Action;
}

impl<F, U> IntoAction<()> for F
where
    F: Fn() -> U + Send + Sync + 'static,
    U: TryFuture + Send,
    U::Error: std::error::Error + Send + Sync + 'static,
    U::Ok: serde::Serialize,
    U: Future<Output = Result<U::Ok, U::Error>> + Send,
{
    type Action = ActionBox;
    #[allow(non_snake_case)]
    fn action(self) -> Self::Action {
        let params = Parameters::default();
        let func = self;
        let action = ActionFn {
            params,
            func: move |_: Arguments| {
                //

                let future = func();

                async move {
                    //
                    let ret = future.await?;

                    let value = value::to_value(ret).unwrap();

                    Result::<_, U::Error>::Ok(value)
                }
            },
        };

        action_box(action)
    }
}

macro_rules! into_action {
    ($type:ident) => {
        impl<F, $type, U> IntoAction<($type,)> for F
        where
            F: Fn($type) -> U + Send + Sync + 'static,
            U: TryFuture + Send,
            U::Error: std::error::Error + Send + Sync + 'static,
            U::Ok: serde::Serialize,
            U: Future<Output = Result<U::Ok, U::Error>> + Send,
            $type: Validatable + Send,
            $type: for<'a> serde::Deserialize<'a>,
        {
            type Action = ActionBox;
            #[allow(non_snake_case)]
            fn action(self) -> Self::Action {
                let params = Parameters::default().add(Parameter::new($type::validator()));
                let func = self;
                let action = ActionFn {
                    params,
                    func: move |args: Arguments| {
                        //
                        into_action!(@step 0usize, args, $type);

                        let future = func($type);

                        async move {
                            //
                            let ret = future.await?;

                            let value = value::to_value(ret).unwrap();

                            Result::<_, U::Error>::Ok(value)
                        }
                    },
                };

                action_box(action)
            }
        }
    };
    ($type1:ident, $( $type:ident ),*) => {
        into_action!($( $type ),*);


        impl<F, $type1, $( $type ),*, U> IntoAction<($type1, $($type),*)> for F
        where
            F: Fn($type1, $($type),*) -> U + Send + Sync + 'static,
            U: TryFuture + Send,
            U::Error: std::error::Error + Send + Sync + 'static,
            U::Ok: serde::Serialize,
            U: Future<Output = Result<U::Ok, U::Error>> + Send,
            $type1: Validatable + Send,
            $type1: for<'a> serde::Deserialize<'a>,
            $( $type: Validatable + Send, $type: for<'a> serde::Deserialize<'a> ),*
        {
            type Action = ActionBox;
            #[allow(non_snake_case)]
            fn action(self) -> Self::Action {
                let mut params = Parameters::default().add(Parameter::new($type1::validator()));

                $(
                    params = params.add(Parameter::new($type::validator()));
                 )*

                let func = self;
                let action = ActionFn {
                    params,
                    func: move |args: Arguments| {

                        into_action!(@step 0usize, args, $type1, $($type),*);
                        let future = func($type1, $($type),*);
                        async move {

                            let ret = future.await?;
                            let value = value::to_value(ret).unwrap();

                            Result::<_, U::Error>::Ok(value)
                        }
                    },
                };

                action_box(action)
            }
        }

    };


    (@step $idx: expr, $args:expr, $type1:ident, $( $type:ident ),*) => {
        let $type1 = $args.try_get::<$type1>($idx).unwrap();
        into_action!(@step $idx + 1usize, $args, $($type),*);
    };

    (@step $idx: expr, $args:expr, $type1:ident) => {
        let $type1 = $args.try_get::<$type1>($idx).unwrap();
    };

    (@step $_idx:expr, $args: expr,) => {};
}

into_action!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
