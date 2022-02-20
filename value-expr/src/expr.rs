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
    fn visit_binary_expr(&mut self, expr: &BinaryExpr<T>) -> Self::Output;
    fn visit_field_expr(&mut self, expr: &FieldExpr<T>) -> Self::Output;
    fn visit_relation_expr(&mut self, expr: &RelationExpr<T>) -> Self::Output;
    fn visit_value_expr(&mut self, expr: &ValueExpr) -> Self::Output;
    fn visit_entity_expr(&mut self, expr: &EntityExpr<T>) -> Self::Output;
}

pub enum Error {}

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum BinaryOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T> {
    Binary(BinaryExpr<T>),
    Field(FieldExpr<T>),
    Relation(RelationExpr<T>),
    Value(ValueExpr),
    Entity(EntityExpr<T>),
}

impl<T> Expr<T> {
    pub fn accept<V: ExprVisitor<T>>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::Field(field) => visitor.visit_field_expr(field),
            Expr::Binary(logical) => visitor.visit_binary_expr(logical),
            Expr::Relation(rel) => visitor.visit_relation_expr(rel),
            Expr::Value(val) => visitor.visit_value_expr(val),
            Expr::Entity(e) => visitor.visit_entity_expr(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]

pub struct BinaryExpr<T> {
    pub left: Box<Expr<T>>,
    pub right: Box<Expr<T>>,
    pub op: BinaryOperator,
}

impl<T> BinaryExpr<T> {
    pub fn new(left: Expr<T>, right: Expr<T>, op: BinaryOperator) -> BinaryExpr<T> {
        BinaryExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
        }
    }
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
pub struct ValueExpr {
    pub value: Value,
}

impl ValueExpr {
    pub fn new(val: impl Into<Value>) -> ValueExpr {
        ValueExpr { value: val.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityExpr<T> {
    pub name: T,
}
