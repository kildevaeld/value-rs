use super::expr::*;
use peg::{error::ParseError, str::LineCol, ParseLiteral};

#[derive(Debug)]
pub enum Stmt<T> {
    Expr(Expr<T>),
    Limit(u64),
    Offset(u64),
}

peg::parser! {
    grammar query_parser() for str {

    pub rule qs() -> Vec<Stmt<&'input str>>
      = l:expr() ** "&" {
          l
      }

    rule expr() -> Stmt<&'input str>
      = e:(e:expr2() { Stmt::Expr(e) } / limit() / offset()) { e }

    rule expr2() -> Expr<&'input str>
      = precedence!{
          l:(@) op:operator() r:@ { Expr::Relational(RelationalExpr {
              left: Box::new(l),
              right: Box::new(r),
              op: op
          }) }
          --
          l:(@) "__" e:@ { Expr::Relation(RelationExpr {
              relation: Box::new(l),
              field: Box::new(e)
          }) }
          --
          p:primary_expr() { p }
      }


    rule primary_expr() -> Expr<&'input str>
      = l:literal() { l }
      / f:$field() { Expr::Field(FieldExpr {
          name: f
      }) }

    rule limit() -> Stmt<&'input str>
        = "$limit" "=" v:integer_literal() { Stmt::Limit(v) }

    rule offset() -> Stmt<&'input str>
        = "$offset" "=" v:integer_literal() { Stmt::Offset(v) }


    rule keywords()
        = operator()
        / separator()
        / keyword("$limit")
        / keyword("$offset")

    rule operator() -> RelationalOperator
        = "=" { RelationalOperator::Eq }
        / keyword("__eq") "=" { RelationalOperator::Eq }
        / keyword("__lt") "=" { RelationalOperator::Lt }
        / keyword("__lte")"="  { RelationalOperator::Lte }
        / keyword("__gt") "="{ RelationalOperator::Gt }
        / keyword("__gte") "=" { RelationalOperator::Gte }
        / keyword("__neq") "=" { RelationalOperator::Neq }
        / keyword("__in") "=" { RelationalOperator::In }



    rule field()
        = (!( keywords() )  alphanum()) ++ ("_" !"_")


    rule keyword(id: &'static str) = ##parse_string_literal(id) !['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']


    rule literal() -> Expr<&'input str>
      = n:(number_literal() / bool_literal() / list_literal() / string_literal()) { Expr::Value(n)}


    rule number_literal() -> ValueExpr<&'input str>
      = i:integer_literal() { ValueExpr::Number(i.into()) }

    rule integer_literal() -> u64
        = i:$(['0'..='9']+) {
            i.parse().unwrap()
        }

    rule string_literal() -> ValueExpr<&'input str>
        =  e:$((!( keywords() / "," ) [_])+) {
            ValueExpr::String(e)
        }

    rule bool_literal() -> ValueExpr<&'input str>
        = b:("true" {true }/ "false" { false }) { ValueExpr::Bool(b) }

    rule list_literal() -> ValueExpr<&'input str>
        = l:(bool_literal() / number_literal() / string_literal()) ++ "," { ValueExpr::List(l) }

    rule alphanum()
        =  ['a'..='z' | 'A'..='Z' | '0'..='9' ]+

    rule separator()
        = "&"
  }
}

pub fn query<'a>(input: &'a str) -> Result<Vec<Stmt<&'a str>>, ParseError<LineCol>> {
    query_parser::qs(input)
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test() {
        let q =
            query("title__eq=naste&author__id__lte=100&$limit=100&test=false&list__in=hey,mig,dig")
                .expect("fail");

        // let q = query("test__mig=200").expect("fail");
        println!("{:#?}", q);
    }
}
