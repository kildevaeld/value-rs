use std::sync::Arc;

use rquickjs::{
    Array as JsArray, Async, Class, ClassDef, ClassId, FromJs, IntoJs, Method, Object, ObjectDef,
    Result, Value as JsValue,
};
use rquickjs::{Ctx, Func};
use value::{Map, Value};
use value_invoke::{IntoArguments, Service, ServiceExt};

use crate::convert::{self, from_js, into_args, into_js};
use crate::response::Response;

#[derive(Debug)]
pub struct JsService<S> {
    service: Arc<S>,
}

impl<S: Service + Send + Sync> JsService<S> {
    pub fn new(service: S) -> JsService<S> {
        JsService {
            service: Arc::new(service),
        }
    }
}

impl<S> Clone for JsService<S> {
    fn clone(&self) -> Self {
        JsService {
            service: self.service.clone(),
        }
    }
}

impl<S: Service + Sync> JsService<S> {
    pub async fn call<A: IntoArguments + Send>(&self, name: &str, args: A) -> Result<Response> {
        let ret = self.service.call(name, args).await.unwrap();
        Ok(Response {
            state: crate::response::ResponseState::Uninit(ret),
        })
    }
}

impl<S: Service + Send + Sync + 'static> ClassDef for JsService<S> {
    const CLASS_NAME: &'static str = "Service";

    unsafe fn class_id() -> &'static mut rquickjs::ClassId {
        static mut CLASS_ID: ClassId = ClassId::new();
        &mut CLASS_ID
    }

    const HAS_PROTO: bool = true;

    // fn init_static<'js>(_ctx: Ctx<'js>, _static: &Object<'js>) -> Result<()> {
    //     println!("init static");
    // }

    fn init_proto<'js>(_ctx: Ctx<'js>, proto: &rquickjs::Object<'js>) -> rquickjs::Result<()> {
        proto.set(
            "description",
            Func::from(Method(|this: &JsService<S>, ctx: Ctx<'js>| {
                //
                // let o = JsArray::new(ctx)?;
                let mut out = Vec::default();
                for i in this.service.interface() {
                    let mut ret = Map::default();

                    ret.insert("name", i.name.clone());
                    let mut params = Vec::default();
                    for p in i.parameters.params.iter() {
                        // params.push(p.t)
                    }

                    ret.insert("params", Value::List(params));

                    out.push(Value::Map(ret));
                }

                convert::into_js(ctx, Value::List(out))
            })),
        )?;
        proto.set(
            "invoke",
            Func::from(Async(Method(
                |this: &JsService<S>, ctx: Ctx<'js>, name: String, value: JsArray<'js>| {
                    //
                    let args = into_args(ctx, value);
                    let this = this.clone();

                    async move {
                        let args = args?;
                        this.call(&name, args).await
                    }
                },
            ))),
        )?;

        Ok(())
    }

    fn into_js_obj<'js>(mut self, ctx: Ctx<'js>) -> Result<JsValue<'js>>
    where
        Self: Sized,
    {
        // self = self.init(ctx);
        Class::<JsService<S>>::instance(ctx, self).map(|val| val.into_value())
    }

    const HAS_STATIC: bool = false;

    const HAS_REFS: bool = false;
}

impl<'js, S: Send + Sync + Service + 'static> IntoJs<'js> for JsService<S> {
    fn into_js(self, ctx: Ctx<'js>) -> Result<JsValue<'js>> {
        self.into_js_obj(ctx)
    }
}

impl<'js, S: Send + Sync + Service + 'static> FromJs<'js> for &'js JsService<S> {
    fn from_js(ctx: Ctx<'js>, value: JsValue<'js>) -> Result<Self> {
        JsService::<S>::from_js_ref(ctx, value)
    }
}

impl<'js, S: Send + Sync + Service + 'static> FromJs<'js> for &'js mut JsService<S> {
    fn from_js(ctx: Ctx<'js>, value: JsValue<'js>) -> Result<Self> {
        JsService::<S>::from_js_mut(ctx, value)
    }
}

// pub fn into_js<'js, S: Service + 'static>(ctx: Ctx<'js>, service: S) {
//     ctx.
// }
