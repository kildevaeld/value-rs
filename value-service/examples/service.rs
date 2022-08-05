use futures_executor::block_on;
use value_service::{HandleFn, HandlerExt, ValueService, ValueServiceBuilder};

fn main() {
    let mut builder = ValueServiceBuilder::default();

    builder.add(
        "method",
        HandleFn::new(|subject: String| async move {
            //
            format!("Hello, {subject}!")
        })
        .action(),
    );

    block_on(async move {
        let output = builder
            .call::<String, _>(100, "method2", ("World",))
            .await
            .expect("Hello, World");

        println!("Got: {}", output);
    });
}
