use super::expr::*;
use value::Value;

pub trait Expression<S> {
    fn to_ast(self) -> Expr<S>;
}

pub trait ExpressionExt<S>: Expression<S> + Sized {
    fn and<E: Expression<S>>(self, e: E) -> BinaryExpr<S> {
        BinaryExpr::new(self.to_ast(), e.to_ast(), BinaryOperator::And)
    }

    fn or<E: Expression<S>>(self, e: E) -> BinaryExpr<S> {
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

impl<'a, S, E> ExpressionExt<S> for E where E: Expression<S> {}

impl<S> Expression<S> for BinaryExpr<S> {
    fn to_ast(self) -> Expr<S> {
        Expr::Binary(self)
    }
}

// impl<S> Expression<S> for BinaryExpr<S> {
//     fn to_ast(self) -> Expr<S> {
//         Expr::Relational(self)
//     }
// }

impl<S> Expression<S> for Expr<S> {
    fn to_ast(self) -> Expr<S> {
        self
    }
}

pub trait ToAst<S> {
    fn to_ast(self) -> Expr<S>;
}

pub trait Col<S> {
    fn to_ast(self) -> Expr<S>;
}

impl Col<String> for String {
    fn to_ast(self) -> Expr<String> {
        Expr::Field(FieldExpr { name: self })
    }
}

impl<'a> Col<&'a str> for &'a str {
    fn to_ast(self) -> Expr<&'a str> {
        Expr::Field(FieldExpr { name: self })
    }
}

impl<'a> Col<&'a str> for (&'a str, &'a str) {
    fn to_ast(self) -> Expr<&'a str> {
        Expr::Relation(RelationExpr {
            relation: Box::new(Expr::Entity(EntityExpr { name: self.0 })),
            field: Box::new(Expr::Field(FieldExpr { name: self.1 })),
        })
    }
}

impl Col<String> for (String, String) {
    fn to_ast(self) -> Expr<String> {
        Expr::Relation(RelationExpr {
            relation: Box::new(Expr::Entity(EntityExpr { name: self.0 })),
            field: Box::new(Expr::Field(FieldExpr { name: self.1 })),
        })
    }
}

pub trait ColExt<S>: Col<S> + Sized {
    fn eql<V: Into<Value>>(self, value: V) -> BinaryExpr<S> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Eq)
    }

    fn lt<V: Into<Value>>(self, value: V) -> BinaryExpr<S> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Lt)
    }

    fn lte<V: Into<Value>>(self, value: V) -> BinaryExpr<S> {
        BinaryExpr::new(
            self.to_ast(),
            value.into().to_ast(),
            BinaryOperator::Lte,
        )
    }

    fn gt<V: Into<Value>>(self, value: V) -> BinaryExpr<S> {
        BinaryExpr::new(self.to_ast(), value.into().to_ast(), BinaryOperator::Gt)
    }

    fn gte<V: Into<Value>>(self, value: V) -> BinaryExpr<S> {
        BinaryExpr::new(
            self.to_ast(),
            value.into().to_ast(),
            BinaryOperator::Gte,
        )
    }

    fn neq<V: Into<Value>>(self, value: V) -> BinaryExpr<S> {
        BinaryExpr::new(
            self.to_ast(),
            value.into().to_ast(),
            BinaryOperator::Neq,
        )
    }
}

impl<S, C: Col<S>> ColExt<S> for C {}

pub trait Val<S> {
    fn to_ast(self) -> Expr<S>;
}

impl<S> Val<S> for Value {
    fn to_ast(self) -> Expr<S> {
        Expr::Value(ValueExpr::new(self))
    }
}
