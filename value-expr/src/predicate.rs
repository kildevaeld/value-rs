use std::marker::PhantomData;
use std::ops::Index;
use value::Value;

use crate::{
    BinaryExpr, BinaryOperator, EntityExpr, ExprVisitor, FieldExpr, RelationExpr, ValueExpr,
};

#[derive(Debug)]
pub enum ValueRef<'a> {
    Owned(Value),
    Borrowed(&'a Value),
}

impl<'a> std::ops::Deref for ValueRef<'a> {
    type Target = Value;
    fn deref(&self) -> &Value {
        match self {
            ValueRef::Owned(v) => v,
            ValueRef::Borrowed(v) => v,
        }
    }
}

impl<'a> From<Value> for ValueRef<'a> {
    fn from(value: Value) -> ValueRef<'a> {
        ValueRef::Owned(value)
    }
}

impl<'a> From<&'a Value> for ValueRef<'a> {
    fn from(value: &'a Value) -> ValueRef<'a> {
        ValueRef::Borrowed(value)
    }
}

pub trait Predicator<'a> {
    fn call(&self, value: &'a Value) -> ValueRef<'a>;
}

impl<'a, F> Predicator<'a> for F
where
    F: Fn(&'a Value) -> ValueRef<'a>,
{
    fn call(&self, value: &'a Value) -> ValueRef<'a> {
        (self)(value)
    }
}

#[derive(Debug)]
pub enum Error {}

pub type Predicate<'a> = Box<dyn Predicator<'a> + 'a>;

pub struct PredicateVistior<'a> {
    _a: PhantomData<&'a dyn Fn()>,
}

impl<'a> Default for PredicateVistior<'a> {
    fn default() -> PredicateVistior<'a> {
        PredicateVistior { _a: PhantomData }
    }
}

impl<'a, T> ExprVisitor<T> for PredicateVistior<'a>
where
    T: AsRef<str> + 'a,
{
    type Output = Result<Predicate<'a>, Error>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr<T>) -> Self::Output {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;

        Ok(Box::new(BinaryPredicator {
            left,
            right,
            op: expr.op,
        }))
    }
    fn visit_field_expr(&mut self, expr: &FieldExpr<T>) -> Self::Output {
        let name = expr.name.as_ref().to_string();
        Ok(Box::new(move |value: &'a Value| value.index(&name).into()))
    }
    fn visit_relation_expr(&mut self, expr: &RelationExpr<T>) -> Self::Output {
        let parent = expr.relation.accept(self)?;
        let field = expr.field.accept(self)?;

        Ok(Box::new(RelationPredicator { parent, field }))
    }

    fn visit_value_expr(&mut self, expr: &ValueExpr) -> Self::Output {
        Ok(Box::new(ValuePredicate {
            value: expr.value.clone(),
            _a: PhantomData,
        }))
    }
    fn visit_entity_expr(&mut self, expr: &EntityExpr<T>) -> Self::Output {
        let name = expr.name.as_ref().to_string();
        Ok(Box::new(move |value: &'a Value| value.index(&name).into()))
    }
}

struct RelationPredicator<'a> {
    parent: Predicate<'a>,
    field: Predicate<'a>,
}

impl<'a> Predicator<'a> for RelationPredicator<'a> {
    fn call(&self, value: &'a Value) -> ValueRef<'a> {
        let parent = self.parent.call(value);
        match parent {
            ValueRef::Borrowed(value) => self.field.call(value),
            ValueRef::Owned(value) => value.into(),
        }
    }
}

struct ValuePredicate<'a> {
    value: Value,
    _a: PhantomData<&'a dyn Fn()>,
}

impl<'a> Predicator<'a> for ValuePredicate<'a> {
    fn call(&self, _value: &'a Value) -> ValueRef<'a> {
        self.value.into()
    }
}

struct BinaryPredicator<'a> {
    left: Predicate<'a>,
    right: Predicate<'a>,
    op: BinaryOperator,
}

impl<'a> Predicator<'a> for BinaryPredicator<'a> {
    fn call(&self, value: &'a Value) -> ValueRef<'a> {
        let left = self.left.call(value);
        let right = self.right.call(value);
        match self.op {
            BinaryOperator::Or => match (&*left, &*right) {
                (Value::Bool(l), Value::Bool(r)) => Value::Bool(*l || *r).into(),
                _ => {
                    println!("lefft: {:?}, right: {:?}", left, right);
                    Value::Bool(false).into()
                }
            },
            BinaryOperator::Eq => Value::Bool(&*left == &*right).into(),
            BinaryOperator::Lte => Value::Bool(&*left <= &*right).into(),
            _ => panic!("not implmented {:?}", self.op),
        }
    }
}

pub struct Filter<'a, I> {
    iter: I,
    predicate: Predicate<'a>,
}

impl<'a, I> Filter<'a, I>
where
    I: Iterator<Item = Value>,
{
    fn filter(&self) -> Vec<I::Item> {
        self.iter
            .filter(|item| {
                let value = self.predicate.call(item);

                match &*value {
                    Value::Bool(b) => *b,
                    _ => false,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::builder::*;

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

        let query = "name"
            .eql("Rasmus")
            .or("age".lte(12))
            .or("pet".eql("guinea pig"))
            .to_ast();
        let mut visitor = PredicateVistior::default();

        let predicate = query.accept(&mut visitor).unwrap();

        let ret = list.iter().filter_map();

        // let filter = Filter {
        //     iter: list.iter(),
        //     predicate: predicate,
        // };

        // let ret = filter.filter();

        // let ret = {
        //     let filter = Filter {
        //         iter: list.iter(),
        //         predicate: predicate,
        //     };

        //     let ret = filter.filter();
        //     ret
        // };

        // let filtered = list.into_iter().filter(move |value| {
        //     //
        //     let ret = predicate.call(value);
        //     match &*ret {
        //         Value::Bool(b) => *b,
        //         _ => false
        //     }

        // }).collect::<Vec<_>>();

        println!("Filted {:#?}", ret);
    }
}
