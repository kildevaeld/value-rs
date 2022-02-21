use super::expr::*;
use value::Value;

pub trait Expression<S, V> {
    fn to_ast(self) -> Expr<S, V>;
}

pub trait ExpressionExt<S, V>: Expression<S, V> + Sized {
    fn and<E: Expression<S, V>>(self, e: E) -> BinaryExpr<S, V> {
        BinaryExpr::new(self.to_ast(), e.to_ast(), BinaryOperator::And)
    }

    fn or<E: Expression<S, V>>(self, e: E) -> BinaryExpr<S, V> {
        BinaryExpr::new(self.to_ast(), e.to_ast(), BinaryOperator::Or)
    }

    // fn and_group<E>(self, e: E) -> BinaryExpr<Self, GroupExpression<E>> {
    //     BinaryExpr {
    //         op: BinaryOperator::And,
    //         left: self,
    //         right: GroupExpression(e),
    //     }
    // }

    // fn or_group<E>(self, e: E) -> BinaryExpr<Self, GroupExpression<E>> {
    //     BinaryExpr {
    //         operator: BinaryOperator::Or,
    //         left: self,
    //         right: GroupExpression(e),
    //     }
    // }
}

impl<'a, S, V, E> ExpressionExt<S, V> for E where E: Expression<S, V> {}

impl<S, V> Expression<S, V> for BinaryExpr<S, V> {
    fn to_ast(self) -> Expr<S, V> {
        Expr::Binary(self)
    }
}

// impl<S> Expression<S> for BinaryExpr<S> {
//     fn to_ast(self) -> Expr<S> {
//         Expr::Relational(self)
//     }
// }

impl<S, V> Expression<S, V> for Expr<S, V> {
    fn to_ast(self) -> Expr<S, V> {
        self
    }
}

pub trait ToAst<S, V> {
    fn to_ast(self) -> Expr<S, V>;
}

pub trait Col<S, V> {
    fn to_ast(self) -> Expr<S, V>;
}

impl<V> Col<String, V> for String {
    fn to_ast(self) -> Expr<String, V> {
        Expr::Field(FieldExpr { name: self })
    }
}

impl<'a, V> Col<&'a str, V> for &'a str {
    fn to_ast(self) -> Expr<&'a str, V> {
        Expr::Field(FieldExpr { name: self })
    }
}

impl<'a, V> Col<&'a str, V> for (&'a str, &'a str) {
    fn to_ast(self) -> Expr<&'a str, V> {
        Expr::Relation(RelationExpr {
            relation: Box::new(Expr::Entity(EntityExpr { name: self.0 })),
            field: Box::new(Expr::Field(FieldExpr { name: self.1 })),
        })
    }
}

impl<V> Col<String, V> for (String, String) {
    fn to_ast(self) -> Expr<String, V> {
        Expr::Relation(RelationExpr {
            relation: Box::new(Expr::Entity(EntityExpr { name: self.0 })),
            field: Box::new(Expr::Field(FieldExpr { name: self.1 })),
        })
    }
}

pub trait ColExt<S, Value>: Col<S, Value> + Sized
where
    Value: Val<S>,
{
    fn eql<V: Into<Value>>(self, value: V) -> BinaryExpr<S, Value> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Eq)
    }

    fn lt<V: Into<Value>>(self, value: V) -> BinaryExpr<S, Value> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Lt)
    }

    fn lte<V: Into<Value>>(self, value: V) -> BinaryExpr<S, Value> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Lte)
    }

    fn gt<V: Into<Value>>(self, value: V) -> BinaryExpr<S, Value> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Gt)
    }

    fn gte<V: Into<Value>>(self, value: V) -> BinaryExpr<S, Value> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Gte)
    }

    fn neq<V: Into<Value>>(self, value: V) -> BinaryExpr<S, Value> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Neq)
    }
}

impl<S, V, C: Col<S, V>> ColExt<S, V> for C where V: Val<S> {}

pub trait Val<S>: Sized {
    fn to_ast(self) -> Expr<S, Self>;
}

impl<S> Val<S> for Value {
    fn to_ast(self) -> Expr<S, Value> {
        Expr::Value(ValueExpr::new(self))
    }
}
