use value::{Number, Value};

pub trait TryCompare {
    fn try_eq(&self, value: &Value) -> Result<bool, Error>;
    // fn try_lte(&self, value: &Value) -> Result<bool, Error>;
}

impl TryCompare for Value {
    fn try_eq(&self, value: &Value) -> Result<bool, Error> {
        Ok(true)
    }
}

pub trait ExprVisitor<T> {
    type Output;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr<T>) -> Self::Output;
    fn visit_relational_expr(&mut self, expr: &RelationalExpr<T>) -> Self::Output;
    fn visit_field_expr(&mut self, expr: &FieldExpr<T>) -> Self::Output;
    fn visit_relation_expr(&mut self, expr: &RelationExpr<T>) -> Self::Output;
    fn visit_value_expr(&mut self, expr: &ValueExpr<T>) -> Self::Output;
}

// pub trait Queryable {
//     type Item;
//     fn list(&self, )
// }

pub enum Error {}

#[derive(Debug, Clone, PartialEq)]

pub enum RelationalOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
}

#[derive(Debug, Clone, PartialEq)]

pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T> {
    Logical(LogicalExpr<T>),
    Relational(RelationalExpr<T>),
    Field(FieldExpr<T>),
    Relation(RelationExpr<T>),
    Value(ValueExpr<T>),
}

impl<T> Expr<T> {
    pub fn accept<V: ExprVisitor<T>>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::Field(field) => visitor.visit_field_expr(field),
            Expr::Logical(logical) => visitor.visit_logical_expr(logical),
            Expr::Relation(rel) => visitor.visit_relation_expr(rel),
            Expr::Relational(rel) => visitor.visit_relational_expr(rel),
            Expr::Value(val) => visitor.visit_value_expr(val),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]

pub struct LogicalExpr<T> {
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
    pub op: LogicalOperator,
}

#[derive(Debug, Clone, PartialEq)]

pub struct RelationalExpr<T> {
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
    pub op: RelationalOperator,
}

#[derive(Debug, Clone, PartialEq)]

pub struct FieldExpr<T> {
    pub name: T,
}

#[derive(Debug, Clone, PartialEq)]

pub struct RelationExpr<T> {
    pub relation: Box<Expr<T>>,
    pub field: Box<Expr<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueExpr<T> {
    String(T),
    Number(Number),
    Bool(bool),
    List(Vec<ValueExpr<T>>),
}
