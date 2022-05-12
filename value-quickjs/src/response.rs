use rquickjs::{
    Accessor, Async, Class, ClassDef, ClassId, Ctx, FromJs, Func, HasRefs, IntoJs, Method, Object,
    Persistent, RefsMarker, Result, Value as JsValue,
};
use value::Value;

use crate::convert;

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

#[derive(Debug)]

pub enum ResponseState {
    Uninit(Value),
    Init { value: Persistent<JsValue<'static>> },
}

// #[bind(object)]
#[derive(Debug)]
pub struct Response {
    // headers: Persistent<Class<'static, Headers>>,
    // method: reqwest::Method,
    pub state: ResponseState,
}

impl ResponseState {
    pub fn value(&self) -> Result<&Persistent<JsValue<'static>>> {
        match self {
            ResponseState::Init { value, .. } => Ok(value),
            _ => Err(throw!("not initialized")),
        }
    }
}

impl Response {
    pub fn new(value: Value) -> Response {
        Response {
            state: ResponseState::Uninit(value),
        }
    }

    // pub async fn text(&mut self) -> Result<String, Error> {
    //     self.resp.take().unwrap().text().await.map_err(throw!())
    // }

    // pub fn take(&mut self) -> Result<reqwest::Response> {
    //     self.state.take()
    // }

    fn init(self, ctx: Ctx) -> Result<Response> {
        let resp = match self.state {
            ResponseState::Uninit(resp) => resp,
            _ => panic!("already intiaillized"),
        };

        let val = convert::into_js(ctx, resp)?;

        Ok(Response {
            state: ResponseState::Init {
                value: Persistent::save(ctx, val),
            },
        })
    }

    // pub fn headers(&self) -> Persistent<Class<'static, Headers>> {
    //     self.headers.clone()
    // }
}

// class_def! {
//     Response
//     (proto) {
//         proto.set("text", Func::from(Async(Method(|this: &mut Response, ctx: Ctx| {

//             //
//             let resp = match this.take() {
//                 Some(resp) => resp,
//                 None => panic!("")
//             };

//             async move {
//                 resp.text().await.map_err(throw!())
//             }

//         }))))?;
//     }

// ~(this, marker) {
//     // this.headers.mark_refs(marker);
//     // mark internal refs if exists
// }
// }

impl ClassDef for Response {
    const CLASS_NAME: &'static str = "Response";

    unsafe fn class_id() -> &'static mut ClassId {
        static mut CLASS_ID: ClassId = ClassId::new();
        &mut CLASS_ID
    }

    // With prototype
    const HAS_PROTO: bool = true;
    fn init_proto<'js>(_ctx: Ctx<'js>, proto: &Object<'js>) -> Result<()> {
        proto.prop(
            "value",
            Accessor::from(Method(|this: &Response, ctx: Ctx<'js>| {
                match this.state.value() {
                    Ok(ret) => ret.clone().restore(ctx),
                    Err(err) => Err(err),
                }
            })),
        )?;
        Ok(())
    }

    // With statics
    const HAS_STATIC: bool = false;
    fn init_static<'js>(_ctx: Ctx<'js>, _ctor: &Object<'js>) -> Result<()> {
        Ok(())
    }

    // With internal references
    const HAS_REFS: bool = true;
    fn mark_refs(&self, marker: &RefsMarker) {
        if let Ok(ret) = self.state.value() {
            ret.mark_refs(marker);
        }
        // marker.mark(&self.some_persistent_value);
    }

    fn into_js_obj<'js>(mut self, ctx: Ctx<'js>) -> Result<JsValue<'js>>
    where
        Self: Sized,
    {
        self = self.init(ctx)?;
        Class::<Self>::instance(ctx, self).map(|val| val.into_value())
    }
}

impl<'js> IntoJs<'js> for Response {
    fn into_js(self, ctx: Ctx<'js>) -> Result<JsValue<'js>> {
        self.into_js_obj(ctx)
    }
}

impl<'js> FromJs<'js> for &'js Response {
    fn from_js(ctx: Ctx<'js>, value: JsValue<'js>) -> Result<Self> {
        Response::from_js_ref(ctx, value)
    }
}

impl<'js> FromJs<'js> for &'js mut Response {
    fn from_js(ctx: Ctx<'js>, value: JsValue<'js>) -> Result<Self> {
        Response::from_js_mut(ctx, value)
    }
}

// impl<'js> FromJs<'js> for Response {
//     fn from_js(ctx: Ctx<'js>, value: Value<'js>) -> Result<Self> {
//         Response::from_js_obj(ctx, value)
//     }
// }
