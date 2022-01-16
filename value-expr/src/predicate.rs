use value::Value;

use crate::{
    EntityExpr, ExprVisitor, FieldExpr, LogicalExpr, RelationExpr, RelationalExpr, ValueExpr,
};

pub enum Error {}

pub type Predicate = Box<dyn Fn(&Value) -> Value>;

pub struct PredicateVistior {}

impl<T> ExprVisitor<T> for PredicateVistior {
    type Output = Result<Predicate, Error>;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr<T>) -> Self::Output {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?:

    
    }
    fn visit_relational_expr(&mut self, expr: &RelationalExpr<T>) -> Self::Output {}
    fn visit_field_expr(&mut self, expr: &FieldExpr<T>) -> Self::Output {}
    fn visit_relation_expr(&mut self, expr: &RelationExpr<T>) -> Self::Output {}
    fn visit_value_expr(&mut self, expr: &ValueExpr) -> Self::Output {}
    fn visit_entity_expr(&mut self, expr: &EntityExpr<T>) -> Self::Output {}
}
