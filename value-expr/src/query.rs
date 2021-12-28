use crate::expr::Expr;

pub struct Query<T> {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub filter: Option<Expr<T>>,
}
