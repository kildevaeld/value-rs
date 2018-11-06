#[macro_export]
macro_rules! val {
    ($x:expr) => {{
        use $crate::ToValue;
        $x.to_value()
    }};

    [ $( $x:expr ),* ] => {
        {
            use $crate::ToValue;
            let mut m = Vec::<$crate::Value>::new();
            $(
                m.push($x.to_value());
            )*
            m
        }
     };

     { $($key:expr => $value:expr),+ } => {
        {
            use $crate::ToValue;
            let mut m = $crate::Map::<String, $crate::Value>::new();
            $(
                m.insert($key.to_string(), $value.to_value());
            )+
            m
        }
     };
}
