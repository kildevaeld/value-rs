use rquickjs::{
    Array as JsArray, Array, Ctx, Null as JsNull, Object, Result, String as JsString, Type,
    TypedArray, Value as JsValue,
};
use value::{Map, Value};

pub fn into_js<'js>(ctx: Ctx<'js>, value: Value) -> Result<JsValue> {
    match value {
        Value::Bool(b) => Ok(JsValue::new_bool(ctx, b)),
        Value::String(str) => Ok(JsString::from_str(ctx, &str)?.into_value()),
        Value::List(list) => {
            //
            let array = Array::new(ctx)?;

            for (idx, item) in list.into_iter().enumerate() {
                array.set(idx, into_js(ctx, item)?)?;
            }

            Ok(array.into_value())
        }
        Value::Number(n) => {
            if n.is_float() {
                Ok(JsValue::new_float(ctx, n.as_f64()))
            } else {
                Ok(JsValue::new_int(ctx, n.as_i64() as i32))
            }
        }
        Value::None => Ok(JsNull.into_value(ctx)),
        Value::Map(map) => {
            let o = Object::new(ctx)?;
            for (k, v) in map.into_iter() {
                o.set(k, into_js(ctx, v)?)?;
            }

            Ok(o.into_value())
        }
        Value::Bytes(bs) => Ok(TypedArray::new(ctx, bs)?.into_value()),
        #[cfg(feature = "datetime")]
        Value::DateTime(datetime) => {
            let ts = datetime.timestamp_millis();

            let ctor = ctx.globals().get::<_, rquickjs::Function>("Date")?;

            let date = ctor.call::<_, JsValue>((ts,))?;

            Ok(date)
        }
        #[cfg(feature = "datetime")]
        Value::Date(date) => {
            let ts = date.and_hms(24, 0, 0).timestamp_millis();

            let ctor = ctx.globals().get::<_, rquickjs::Function>("Date")?;

            let date = ctor.call::<_, JsValue>((ts,))?;

            Ok(date)
        }
        v => panic!("{:?}", v),
    }
}

macro_rules! call {
    ($obj: expr, $method: expr) => {{
        let func: rquickjs::Function = $obj.get($method)?;
        func.call::<_, u32>((rquickjs::This($obj.clone()),))?
    }};
}

pub fn from_js<'js>(ctx: Ctx<'js>, value: JsValue<'js>) -> Result<Value> {
    match value.type_of() {
        Type::Array => {
            let array = value.into_array().unwrap();
            let list = array
                .iter::<JsValue>()
                .map(|m| from_js(ctx, m.unwrap()))
                .collect::<Result<_>>()?;
            Ok(Value::List(list))
        }
        Type::String => Ok(Value::String(value.get()?)),
        Type::Null | Type::Undefined | Type::Uninitialized => Ok(Value::None),
        Type::Object => {
            let date = ctx.globals().get::<_, JsValue>("Date")?;

            if let Ok(bytes) = TypedArray::<u8>::from_value(value.clone()) {
                let bs: &[u8] = bytes.as_ref();
                return Ok(Value::Bytes(bs.to_vec()));
            } else if value.as_object().unwrap().is_instance_of(&date) {
                #[cfg(feature = "datetime")]
                {
                    use chrono::TimeZone;
                    let date = value.as_object().unwrap();
                    let year = call!(date, "getUTCFullYear");
                    let month = call!(date, "getUTCMonth");
                    let day = call!(date, "getUTCDate");
                    let hours = call!(date, "getUTCHours");
                    let mins = call!(date, "getUTCMinutes");
                    let secs = call!(date, "getUTCSeconds");
                    let ms = call!(date, "getUTCMilliseconds");
                    let date = chrono::Utc
                        .ymd(year as i32, month, day)
                        .and_hms_milli(hours, mins, secs, ms);

                    return Ok(Value::DateTime(date.naive_utc()));
                }
            }

            let obj = value.into_object().unwrap();
            let mut out = Map::default();
            for v in obj.into_iter() {
                let (k, v) = v?;
                let k = k.to_string()?;
                out.insert(k, from_js(ctx, v)?);
            }

            Ok(Value::Map(out))
        }
        Type::Int => Ok(Value::Number(value.as_int().unwrap().into())),
        t => {
            todo!("from js: {:?}", t)
        }
    }
}

pub fn into_args<'js>(ctx: Ctx<'js>, value: JsArray<'js>) -> Result<Vec<Value>> {
    value
        .iter::<JsValue>()
        .map(|m| from_js(ctx, m.unwrap()))
        .collect::<Result<_>>()
}
