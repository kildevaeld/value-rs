mod builder;
mod expr;
mod query;

mod predicate;

pub use self::{builder::*, expr::*, query::*};

#[cfg(test)]
mod test {
    use super::*;

    fn test() {
        Query::new("id".eql("Rasmus").and(("auther", "id").lte(20)));
    }
}
