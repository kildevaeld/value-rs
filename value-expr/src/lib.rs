mod builder;
mod expr;
#[cfg(feature = "parser")]
mod parser;
mod query;

pub use self::{builder::*, expr::*, query::*};

#[cfg(test)]
mod test {
    use super::*;

    fn test() {
        Query::new("id".eql("Rasmus").and(("auther", "id").lte(20)));
    }
}
