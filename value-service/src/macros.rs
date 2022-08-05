macro_rules! action {
    (|ctx, $($name: ident : $ty: ty),*|  -> $type: ty  $block: block) => {
        //
        {
            let signature = $crate::Signature::default();
            $crate::ActionFn::new(|ctx, $($name:  $ty),*| async move { $block }, signature)
        }
    };
    (|$($name: ident : $ty: ty),*| -> $type: ty $block: block) => {
        {
            action!(|ctx, $($name: ident : $ty: ty),*| -> $type $block)
        }
    };
    (|| -> $ty: ty $block: block) => {
        //
        {
            let signature = $crate::Signature::default();
            $crate::ActionFn::new(|ctx| async move { $block }, signature)
        }
    };
    (async $block: block) => {
        //
        {
            let signature = $crate::Signature::default();
            $crate::ActionFn::new(|ctx| async move { $block }, signature)
        }
    };
}

fn test() -> impl crate::Action<i32> {
    let a = action!(|| -> () { () });

    a
}

// fn test2() -> impl crate::Action<i32> {
//     let a = action!(|ctx, name: &str| -> String { () });

//     a
// }
