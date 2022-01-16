use validator::*;
use value::{value, Value};
use value_validate::*;

fn main() {
    let o = object()
        .field(
            "name",
            string()
                .required()
                .one_of((equal("Rasmus"), equal("Feja"), any()))
                .max(100),
        )
        .field("age", number().min(18).max(100).required())
        .field("list", list().and(tuple((string(), object()))).required());

    // let o: Validator = any_of(vec![o.into(), number().into()]).into();
    // let o: ValidationBox = Box::new(o);

    let json = serde_json::to_string_pretty(&o).unwrap();

    println!("{}", json);
    o.validate(&value!({
        "name": "Rasmus1",
        "age": 18,
        "list": [
            "test",
            {
                "test": ""
            }
        ]
    }))
    .expect("Hello");
}
