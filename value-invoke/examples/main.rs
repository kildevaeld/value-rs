use core::fmt;

use value::{to_value, Value};
use value_invoke::{Arguments, IntoAction, Service};
use value_validate::Validatable;

#[derive(Debug)]

pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(Validatable, serde::Serialize, serde::Deserialize)]
struct Test {
    name: String,
    age: u8,
}

#[derive(Validatable, serde::Serialize, serde::Deserialize)]
struct Test2(String);

impl std::error::Error for Error {}

async fn test(arg: String, arg2: Test, args: Vec<String>) -> Result<String, Error> {
    Ok(format!("Hello, {}", arg2.name))
}

fn main() {
    smol::block_on(async {
        //
        let mut service = Service::default();

        service.register_box("test", test.action());

        let ret = serde_json::to_string_pretty(&service.interface()).unwrap();

        let ret = service
            .call(
                "test",
                (
                    "hello",
                    Test {
                        name: "Rasmys".to_owned(),
                        age: 18,
                    },
                    vec!["Hello"],
                    "Hello2",
                ),
            )
            .await;

        println!("VALUES {:?}", ret);
    });
}
