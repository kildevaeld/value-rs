use rquickjs::{Class, Context, Func, Promise, Promised, Runtime};
use value_invoke::{service, IntoService, ServiceExt};
use value_quickjs::{js_service, JsService, Response};
use value_validate;

macro_rules! throw {
    ($error: expr) => {
        rquickjs::Error::Exception {
            message: $error.to_string(),
            file: String::default(),
            line: 0,
            stack: String::default(),
        }
    };
    () => {
        |err| throw!(err)
    };
}

#[derive(Clone)]
pub struct Test;

#[service]
impl Test {
    async fn hello(&self, who: String) -> Result<String, ()> {
        Ok(format!("Hello, {}!", who))
    }
}

js_service!(Test);

static TEST: &'static str = r#"

export async function main(ctx) {

    print(JSON.stringify(ctx.description()))
    const ret = await ctx.invoke("hello", ["World"]);

    print(ret.value)

    // return ret;

    return ret.value
}   

"#;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test = Test;

    // let test = test.into_service();

    let out = test.call("hello", ("World",)).await.unwrap();

    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;

    rt.spawn_executor(rquickjs::Tokio);

    let ret = ctx
        .with(|ctx| {
            ctx.globals().set(
                "print",
                Func::new("print", |arg: String| {
                    println!("{}", arg);
                }),
            )?;

            let service = test; //JsService::new(test);

            Class::<Test>::register(ctx)?;
            Class::<JsService<TestService>>::register(ctx)?;
            Class::<Response>::register(ctx)?;

            let module = ctx.compile("temp", TEST)?;

            let func = module.get::<_, rquickjs::Function<'_>>("main")?;

            func.call::<_, Promise<String>>((service,))
        })?
        .await?;

    println!("output: {}", ret);

    Ok(())
}
