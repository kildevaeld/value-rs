use value::Value;

use super::builder::Expression;
use super::expr::Expr;

#[derive(Clone, PartialEq, Debug)]
pub enum Ordering {
    Asc(String),
    Desc(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Filter<S> {
    Expr(Expr<S>),
    Id(Value),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Query<S> {
    pub filter: Option<Expr<S>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub columns: Option<Vec<S>>,
    pub includes: Option<Vec<S>>,
}

impl<S> Default for Query<S> {
    fn default() -> Self {
        Query {
            filter: None,
            limit: None,
            offset: None,
            columns: None,
            includes: None,
        }
    }
}

impl<S> Query<S> {
    pub fn new<E: Expression<S>>(filter: E) -> Query<S> {
        Query {
            filter: Some(filter.to_ast()),
            ..Default::default()
        }
    }

    pub fn columns(mut self, columns: impl Into<Vec<S>>) -> Self {
        self.columns = Some(columns.into());
        self
    }

    pub fn includes(mut self, includes: impl Into<Vec<S>>) -> Self {
        self.includes = Some(includes.into());
        self
    }

    pub fn limit(mut self, limit: impl Into<Option<u64>>) -> Self {
        self.limit = limit.into();
        self
    }

    pub fn offset(mut self, offset: impl Into<Option<u64>>) -> Self {
        self.offset = offset.into();
        self
    }
}
