use value::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    ty: Type,
    name: String,
    required: bool,
}

pub struct Arguments {}
