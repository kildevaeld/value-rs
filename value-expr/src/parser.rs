use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take, take_while},
    character::{
        complete::{alpha1, alphanumeric1, digit1, one_of},
        is_digit,
    },
    combinator::{opt, recognize},
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};
use value::Number;

use crate::{
    expr::{
        Expr, FieldExpr, LogicalExpr, LogicalOperator, RelationalExpr, RelationalOperator,
        ValueExpr,
    },
    query::Query,
};

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn url_code_points<T>(i: T) -> Res<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum() && !(char_item == '.')
            // ... actual ascii code points and url encoding...: https://infra.spec.whatwg.org/#ascii-code-point
        },
        ErrorKind::AlphaNumeric,
    )
}

fn field<'a>(input: &'a str) -> Res<&'a str, Expr<&'a str>> {
    context("field", url_code_points)(input).map(|(next_input, ret)| {
        //

        (next_input, Expr::Field(FieldExpr { name: ret }))
    })
}

fn lookup<'a>(input: &'a str) -> Res<&'a str, Expr<&'a str>> {
    context("lookup", field)(input)
}

fn number<'a>(input: &'a str) -> Res<&'a str, ValueExpr> {
    context("number", integer)(input).map(|(next, number)| (next, ValueExpr::new(number)))
}

fn string<'a>(input: &'a str) -> Res<&'a str, ValueExpr> {
    context("string", recognize(alphanumeric1))(input)
        .map(|(next, number)| (next, ValueExpr::new(number)))
}

fn integer<'a>(input: &'a str) -> Res<&'a str, Number> {
    recognize(digit1)(input).and_then(|(next, num)| {
        //
        match num.parse::<u64>() {
            Ok(s) => Ok((next, s.into())),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

fn literal<'a>(input: &'a str) -> Res<&'a str, ValueExpr> {
    context("literal", alt((number, string)))(input)
}

fn relational<'a>(input: &'a str) -> Res<&'a str, Expr<&'a str>> {
    context("relational", tuple((lookup, operator, literal)))(input).map(|(next, res)| {
        let (lookup, op, literal) = res;

        (
            next,
            Expr::Relational(RelationalExpr {
                left: Box::new(lookup),
                right: Box::new(Expr::Value(literal)),
                op,
            }),
        )
    })
}

fn operator<'a>(input: &'a str) -> Res<&'a str, RelationalOperator> {
    context(
        "operator",
        tuple((
            opt(tuple((
                tag("__"),
                alt((
                    tag("eq"),
                    tag("neq"),
                    tag("lt"),
                    tag("lte"),
                    tag("gt"),
                    tag("gte"),
                    tag("in"),
                )),
            ))),
            tag("="),
        )),
    )(input)
    .map(|(next, ret)| {
        //
        let (op, _) = ret;

        let op = if let Some((_, op)) = op {
            match op {
                "eq" => RelationalOperator::Eq,
                "neq" => RelationalOperator::Neq,
                "lt" => RelationalOperator::Lt,
                "lte" => RelationalOperator::Lte,
                "gt" => RelationalOperator::Gt,
                "gte" => RelationalOperator::Gte,
                "in" => RelationalOperator::In,
                _ => unreachable!("Should not happen"),
            }
        } else {
            RelationalOperator::Eq
        };

        (next, op)
    })
}

// fn relation<'a>(input: &'a str) -> Res<&'a str, Expr<&'a str>> {
//     context("field", alt((url_code_points, relation)))
// }

pub fn parse<'a>(input: &'a str) -> Res<&'a str, Option<Expr<&'a str>>> {
    let out = context(
        "qs",
        tuple((
            relational,
            many0(tuple((
                tag("&"),
                //
                relational,
            ))),
        )),
    )(input)
    .map(|(next, ret)| {
        //
        let (first, rest) = ret;

        let expr = std::iter::once(first)
            .chain(rest.into_iter().map(|m| m.1))
            .fold(None, |acc: Option<Expr<&'a str>>, cur| {
                //
                match acc {
                    Some(acc) => Some(Expr::Logical(LogicalExpr {
                        left: Box::new(acc),
                        right: Box::new(cur),
                        op: LogicalOperator::And,
                    })),
                    None => Some(cur),
                }
            });

        (next, expr)
    });

    out
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test() {
        let out = parse("hello__neq=rasmus&nemmelig=hans").expect("");

        println!("OUT {:#?}", out);
    }
}
