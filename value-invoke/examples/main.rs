use core::fmt;

use value_invoke::{prelude::*, service};
use value_validate::Validatable;

#[derive(Debug)]

pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(Validatable, serde::Serialize, serde::Deserialize)]
struct Test {
    name: String,
    age: u8,
}

pub struct TestService;

#[service]
impl TestService {
    async fn run(&self, arg: String, _age: u32) -> Result<(), Error> {
        println!("{}", arg);
        Ok(())
    }

    // #[service(unblock = smol::unblock)]
    // fn test(&self, arg: String, _age: u32) -> Result<(), Error> {
    //     println!("{}", arg);
    //     Ok(())
    // }
}

#[derive(Validatable, serde::Serialize, serde::Deserialize)]
struct Test2(String);

impl std::error::Error for Error {}

async fn test(_arg: String, arg2: Test, _args: Vec<String>) -> Result<String, Error> {
    Ok(format!("Hello, {}", arg2.name))
}

fn main() {
    futures_executor::block_on(async {
        //
        // let mut service = Service::default();

        // service.register_box("test", test.action());

        // let _ret = serde_json::to_string_pretty(&service.interface()).unwrap();

        // let ret = service
        //     .call(
        //         "test",
        //         (
        //             "hello",
        //             Test {
        //                 name: "Rasmys".to_owned(),
        //                 age: 18,
        //             },
        //             vec!["Hello"],
        //             "Hello2",
        //         ),
        //     )
        //     .await;

        // println!("VALUES {:?}", ret);

        let service = TestService.into_service();

        service.call("run", ("Hello", 20)).await;
    });
}
