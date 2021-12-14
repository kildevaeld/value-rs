use value::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    ty: ValueType,
    name: String,
    required: bool,
}

pub struct Arguments {}
