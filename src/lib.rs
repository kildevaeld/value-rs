#[cfg(feature = "datetime")]
pub extern crate chrono;
#[cfg(feature = "serde_enc")]
extern crate serde;

#[macro_use]
pub mod macros;
mod map;
mod number;
#[cfg(feature = "serde_enc")]
mod serde_enc;
mod to_value;
mod value;

pub use self::map::*;
pub use self::number::*;
#[cfg(feature = "serde_enc")]
pub use self::serde_enc::*;
pub use self::to_value::ToValue;
pub use self::value::*;

#[cfg(test)]
mod tests {

    use super::Value;
    #[test]
    fn it_works() {
        val!("Test mig");
        val!(32);
        val!(val!{
            "test" => "mig",
            "test2" => val!{
                "test" => 200,
                "rapper" => val!["rapp", false, Value::Bool(true)]
            }
        });
        val!(val!["test", 200, true]);

        val!["test", 200, false];
        val!{
            "rapper" => true,
            "test" => val!["ost", 2.3455533, -2.3]
        };

        assert_eq!(2 + 2, 4);
    }
}
