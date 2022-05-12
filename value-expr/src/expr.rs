#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait ExprVisitor<T, V> {
    type Output;
    fn visit_binary_expr(&mut self, expr: BinaryExpr<T, V>) -> Self::Output;
    fn visit_field_expr(&mut self, expr: FieldExpr<T>) -> Self::Output;
    fn visit_relation_expr(&mut self, expr: RelationExpr<T, V>) -> Self::Output;
    fn visit_value_expr(&mut self, expr: ValueExpr<V>) -> Self::Output;
    fn visit_entity_expr(&mut self, expr: EntityExpr<T>) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub enum Expr<T, V> {
    Binary(BinaryExpr<T, V>),
    Field(FieldExpr<T>),
    Relation(RelationExpr<T, V>),
    Value(ValueExpr<V>),
    Entity(EntityExpr<T>),
}

impl<T, V> Expr<T, V> {
    pub fn accept<Visitor: ExprVisitor<T, V>>(self, visitor: &mut Visitor) -> Visitor::Output {
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinaryExpr<T, V> {
    pub left: Box<Expr<T, V>>,
    pub right: Box<Expr<T, V>>,
    pub op: BinaryOperator,
}

impl<T, V> BinaryExpr<T, V> {
    pub fn new(left: Expr<T, V>, right: Expr<T, V>, op: BinaryOperator) -> BinaryExpr<T, V> {
        BinaryExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub struct FieldExpr<T> {
    pub name: T,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub struct RelationExpr<T, V> {
    pub relation: Box<Expr<T, V>>,
    pub field: Box<Expr<T, V>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub struct ValueExpr<V> {
    pub value: V,
}

impl<V> ValueExpr<V> {
    pub fn new(val: impl Into<V>) -> ValueExpr<V> {
        ValueExpr { value: val.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub struct EntityExpr<T> {
    pub name: T,
}
