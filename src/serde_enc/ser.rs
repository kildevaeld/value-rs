use super::super::value::{Type, Value};
use serde::Serialize;

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match *self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(b),
            //Value::Number(ref n) => n.serialize(serializer),
            Value::String(ref s) => serializer.serialize_str(s),
            Value::Array(ref v) => v.serialize(serializer),
            Value::Object(ref m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_key(k)?;
                    map.serialize_value(v)?;
                }
                map.end()
            }
            _ => panic!("not implemented"),
        }
    }
}

impl Serialize for Type {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
        // match *self {
        //     Value::Null => serializer.serialize_unit(),
        //     Value::Bool(b) => serializer.serialize_bool(b),
        //     //Value::Number(ref n) => n.serialize(serializer),
        //     Value::String(ref s) => serializer.serialize_str(s),
        //     Value::Array(ref v) => v.serialize(serializer),
        //     Value::Object(ref m) => {
        //         use serde::ser::SerializeMap;
        //         let mut map = serializer.serialize_map(Some(m.len()))?;
        //         for (k, v) in m {
        //             map.serialize_key(k)?;
        //             map.serialize_value(v)?;
        //         }
        //         map.end()
        //     }
        //     _ => panic!("not implemented"),
        // }
    }
}
