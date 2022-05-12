use rquickjs::{
    Array as JsArray, Array, Ctx, Object, Result, String as JsString, Type, Value as JsValue,
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
        Value::Map(map) => {
            let o = Object::new(ctx)?;
            for (k, v) in map.into_iter() {
                o.set(k, into_js(ctx, v)?)?;
            }

            Ok(o.into_value())
        }
        v => panic!("{:?}", v),
    }
}

pub fn from_js<'js>(value: JsValue<'js>) -> Result<Value> {
    match value.type_of() {
        Type::Array => {
            let array = value.into_array().unwrap();
            let list = array
                .iter::<JsValue>()
                .map(|m| from_js(m.unwrap()))
                .collect::<Result<_>>()?;
            Ok(Value::List(list))
        }
        Type::String => Ok(Value::String(value.get()?)),
        Type::Null | Type::Undefined | Type::Uninitialized => Ok(Value::None),
        Type::Object => {
            let obj = value.into_object().unwrap();
            let mut out = Map::default();
            for v in obj.into_iter() {
                let (k, v) = v?;
                let k = k.to_string()?;
                out.insert(k, from_js(v)?);
            }

            Ok(Value::Map(out))
        }
        Type::Int => Ok(Value::Number(value.as_int().unwrap().into())),
        t => {
            todo!("from js: {:?}", t)
        }
    }
    // if value.is_array() {
    // } else if value.is_string() {
    // } else if value.is_bool() {
    // } else if value.is_float() {
    // } else if value.is_number() {
    // }
}

pub fn into_args<'js>(value: JsArray<'js>) -> Result<Vec<Value>> {
    value
        .iter::<JsValue>()
        .map(|m| from_js(m.unwrap()))
        .collect::<Result<_>>()
}
