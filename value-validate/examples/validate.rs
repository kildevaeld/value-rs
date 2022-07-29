use validator::*;
use value::{value, Value};
use value_validate::*;

fn main() {
    let o = object()
        .field(
            "name",
            string()
                .required()
                .one_of((equal("Rasmus"), equal("Feja")))
                .max(6)
                .min(3),
        )
        .field("age", number().min(18).max(100).required())
        .field(
            "list",
            list()
                .and(tuple((string(), object().and(item(string())))))
                .required(),
        );

    // let o: Validator = any_of(vec![o.into(), number().into()]).into();
    // let o: ValidationBox = Box::new(o);

    // let json = serde_json::to_string_pretty(&o).unwrap();
    // let json = serde_json::to_string_pretty(&value!({
    //     "name": "Rasmus1",
    //     "age": 18,
    //     "list": [
    //         "test",
    //         {
    //             "test": ""
    //         }
    //     ]
    // }))
    // .unwrap();

    println!("validation: {}", serde_json::to_string_pretty(&o).unwrap());

    println!(
        "{:#?}",
        &value!({
            "name": "Rasmus1",
            "age": 18,
            "list": [
                "test",
                {
                    "test": ""
                }
            ]
        })
    );
    o.validate(&value!({
        "name": "Rasmus2",
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
