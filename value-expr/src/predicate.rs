use crate::{
    BinaryExpr, BinaryOperator, EntityExpr, Expr, ExprVisitor, FieldExpr, RelationExpr, ValueExpr,
};
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::ops::Index;
use value::Value;

#[derive(Debug)]
pub enum Predication<T> {
    Parent(T),
    Field(T),
    Binary {
        left: Box<Predication<T>>,
        right: Box<Predication<T>>,
        operator: BinaryOperator,
    },
    Relation {
        parent: Box<Predication<T>>,
        field: Box<Predication<T>>,
    },
    Value(Value),
}

impl<T: AsRef<str> + core::fmt::Debug> Predication<T> {
    fn call<'b>(&'b self, value: &'b Value) -> ValueRef<'b, Value> {
        use Predication::*;

        match self {
            Parent(parent) => value.index(parent.as_ref()).into(),
            Field(field) => value.index(field.as_ref()).into(),
            Relation { parent, field } => {
                //
                match parent.call(value) {
                    ValueRef::Borrowed(v) => field.call(v),
                    ValueRef::Owned(v) => v.into(),
                }
            }
            Binary {
                left,
                right,
                operator,
            } => {
                //
                use value::Value;
                let left = left.call(value);
                let right = right.call(value);

                match operator {
                    BinaryOperator::Or => match (&*left, &*right) {
                        (Value::Bool(l), Value::Bool(r)) => value::Value::Bool(*l || *r).into(),
                        _ => Value::Bool(false).into(),
                    },
                    BinaryOperator::And => match (&*left, &*right) {
                        (Value::Bool(l), Value::Bool(r)) => value::Value::Bool(*l && *r).into(),
                        _ => Value::Bool(false).into(),
                    },
                    BinaryOperator::Eq => Value::Bool(&*left == &*right).into(),
                    BinaryOperator::Neq => Value::Bool(&*left != &*right).into(),
                    BinaryOperator::Lt => Value::Bool(&*left < &*right).into(),
                    BinaryOperator::Lte => Value::Bool(&*left <= &*right).into(),
                    BinaryOperator::Gt => Value::Bool(&*left > &*right).into(),
                    BinaryOperator::Gte => Value::Bool(&*left >= &*right).into(),
                    BinaryOperator::In => {
                        if let Some(list) = right.as_list() {
                            Value::Bool(list.contains(&left)).into()
                        } else {
                            Value::Bool(false).into()
                        }
                    }
                }
            }
            Value(val) => val.into(),
        }
    }

    pub fn filter<I>(self, iter: I) -> Filter<I, T>
    where
        I: IntoIterator,
        I::Item: AsRef<Value>,
    {
        Filter::new(iter.into(), self)
    }
}

#[derive(Debug)]
pub enum ValueRef<'a, V> {
    Owned(V),
    Borrowed(&'a V),
}

impl<'a, V> core::ops::Deref for ValueRef<'a, V> {
    type Target = V;
    fn deref(&self) -> &V {
        match self {
            ValueRef::Owned(v) => v,
            ValueRef::Borrowed(v) => v,
        }
    }
}

impl<'a> From<Value> for ValueRef<'a, Value> {
    fn from(value: Value) -> ValueRef<'a, Value> {
        ValueRef::Owned(value)
    }
}

impl<'a> From<&'a Value> for ValueRef<'a, Value> {
    fn from(value: &'a Value) -> ValueRef<'a, Value> {
        ValueRef::Borrowed(value)
    }
}

#[derive(Debug)]
pub enum Error {}

#[derive(Default)]
pub struct PredicateVistior;

impl<T> ExprVisitor<T, Value> for PredicateVistior
where
    T: AsRef<str>,
{
    type Output = Predication<T>;
    fn visit_binary_expr(&mut self, expr: BinaryExpr<T, Value>) -> Self::Output {
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);

        Predication::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator: expr.op,
        }
    }
    fn visit_field_expr(&mut self, expr: FieldExpr<T>) -> Self::Output {
        Predication::Field(expr.name)
    }
    fn visit_relation_expr(&mut self, expr: RelationExpr<T, Value>) -> Self::Output {
        let parent = expr.relation.accept(self);
        let field = expr.field.accept(self);

        Predication::Relation {
            parent: Box::new(parent),
            field: Box::new(field),
        }
    }

    fn visit_value_expr(&mut self, expr: ValueExpr<Value>) -> Self::Output {
        Predication::Value(expr.value)
    }
    fn visit_entity_expr(&mut self, expr: EntityExpr<T>) -> Self::Output {
        Predication::Parent(expr.name)
    }
}

pub struct Filter<I, S> {
    iter: I,
    predicate: Predication<S>,
}

impl<I, S> Filter<I, S> {
    pub fn new(iter: I, predicate: Predication<S>) -> Filter<I, S> {
        Filter { iter, predicate }
    }
}

impl<I, S> Iterator for Filter<I, S>
where
    I: Iterator,
    I::Item: AsRef<Value>,
    S: AsRef<str> + core::fmt::Debug,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = match self.iter.next() {
                Some(next) => next,
                None => return None,
            };

            let value = self.predicate.call(item.as_ref());

            match &*value {
                Value::Bool(b) if *b => return Some(item),
                _ => continue,
            }
        }
    }
}

pub trait IteratorExt: Iterator {
    fn query<E, S>(self, query: E) -> Filter<Self, S>
    where
        E: Into<Expr<S, Value>>,
        S: AsRef<str>,
        Self: Sized,
        Self::Item: AsRef<Value>,
    {
        let mut visitor = PredicateVistior::default();
        let predicate = query.into().accept(&mut visitor);
        Filter::new(self, predicate)
    }
}

impl<I> IteratorExt for I where I: Iterator {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{builder::*, Expr};

    use value::value;

    #[test]
    fn test() {
        let list = vec![
            value!({
                "name": "Rasmus",
                "age": 38,
                "pet": value!({
                    "name": "Willbur",
                    "type": "cat"
                })
            }),
            value!({
                "name": "Freja",
                "age": 33,
                "pet": value!({
                    "name": "Store Søster",
                    "type": "cat"
                })
            }),
            value!({
                "name": "Alvilda",
                "age": 12,
                "pet": value!({
                    "name": "Bubble",
                    "type": "guinea pig"
                })
            }),
        ];

        let query: Expr<_, Value> = "name"
            .eql("Rasmus")
            .or("age".lte(13))
            .or(("pet", "type").eql("cat"))
            .to_ast();
        let mut visitor = PredicateVistior::default();

        let predicate = query.accept(&mut visitor);

        let filter = Filter::new(list.into_iter(), predicate);

        // println!("Filted {:#?}", filter.collect::<Vec<_>>());
    }
}
