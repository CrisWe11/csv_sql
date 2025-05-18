mod expression;
#[cfg(test)]
mod test;

use std::os::unix::raw::ino_t;
use crate::sql_parser::expression::Expression::Literal;
use crate::sql_parser::expression::{BinaryOperator, LiteralValue, UnaryOperator};
use expression::Expression;
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, tag, tag_no_case, take_until, take_until1};
use nom::character::complete::{
    alpha1, alphanumeric1, char, multispace0, multispace1, none_of, one_of, space1,
};
use nom::combinator::{map, recognize, value, verify};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0, many0_count, many1, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, separated_pair, terminated};
use nom::Err;
use nom::{Compare, IResult, Parser};

trait SelectField {}

#[derive(Debug, PartialEq)]
struct SelectClause {
    exprs: Vec<Expression>,
    table: String,
}

impl SelectClause {
    fn new(fields: Vec<Expression>, table: String) -> Self {
        SelectClause {
            exprs: fields,
            table,
        }
    }
}

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

macro_rules! wss {
    ($inner:expr) => {delimited(multispace0, $inner, multispace0)};
}

pub fn raw_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

macro_rules! identifier {
    ($($x: literal),*) => {
        pub fn identifier(
            input: &str,
        ) -> IResult<&str, &str> {
            verify(raw_identifier, |output: &str| ![$($x, $x.to_uppercase().as_str()),*].contains(&output)).parse(input)
        }
    };
}

identifier!("from", "as", "select", "into", "delete", "alter");

macro_rules! declare_binary_operator {
    ($name:ident, { $key:literal => $value:expr }) => {
        fn $name(input: &str) -> IResult<&str, BinaryOperator> {
            map(
                tag_no_case($key),
                |op| match op {
                    $key => $value,
                    _ => unreachable!("unrecognized binary operator"),
                },
            ).parse(input)
        }
    };
    ($name:ident, { $($key:literal => $value:expr),* $(,)? }) => {
        fn $name(input: &str) -> IResult<&str, BinaryOperator> {
            map(
                alt((
                    $(tag_no_case($key)),*
                ),),
                |op| match op {
                    $($key => $value,)*
                    _ => unreachable!("unrecognized binary operator"),
                },
            ).parse(input)
        }
    };
}

// 以下是 ClickHouse SQL 中运算符的优先级，从高到低排列：
//
// | 优先级 | 运算符类别 | 运算符 | 结合性 |
// |--------|------------|--------|--------|
// | 1 | 访问运算符 | `.`, `[]` | 从左到右 |
// | 2 | 一元运算符 | `+`, `-`(一元), `NOT`, `~` | 从右到左 |
// | 3 | 乘法和除法 | `*`, `/`, `%` | 从左到右 |
// | 4 | 加法和减法 | `+`, `-`(二元) | 从左到右 |
// | 5 | 位移运算符 | `<<`, `>>` | 从左到右 |
// | 6 | 位与 | `&` | 从左到右 |
// | 7 | 位异或 | `^` | 从左到右 |
// | 8 | 位或 | `\|` | 从左到右 |
// | 9 | 比较运算符 | `=`, `!=`, `<>`, `<`, `<=`, `>`, `>=`, `LIKE`, `IN`, `GLOBAL IN`, `NOT IN`, `GLOBAL NOT IN`, `IS NULL`, `IS NOT NULL` | 从左到右 |
// | 10 | 逻辑与 | `AND` | 从左到右 |
// | 11 | 逻辑或 | `OR` | 从左到右 |
// | 12 | 条件运算符 | `CASE`, `WHEN`, `THEN`, `ELSE` | 从右到左 |
// | 13 | 赋值运算符 | `:=` | 从右到左 |

declare_binary_operator!(binop5, {"*" => BinaryOperator::Multiply, "/" => BinaryOperator::Divide, "%" => BinaryOperator::Modulo});
declare_binary_operator!(binop4, {"+" => BinaryOperator::Add, "-" => BinaryOperator::Subtract});
declare_binary_operator!(binop3, {">=" => BinaryOperator::GtEq, "<=" => BinaryOperator::LtEq, "<" => BinaryOperator::Lt, ">" => BinaryOperator::Gt, "=" => BinaryOperator::Eq, "!=" => BinaryOperator::NotEq, "<>" => BinaryOperator::NotEq});
declare_binary_operator!(binop2, {"AND" => BinaryOperator::And});
declare_binary_operator!(binop1, {"OR" => BinaryOperator::Or});


fn unary_op(input: &str) -> IResult<&str, Expression> {
    todo!()
}

fn function_call(input: &str) -> IResult<&str, Expression> {
    let (input, (name, args)) = (
        identifier,
        delimited(
            tag("("),
            wss!(separated_list0(wss!(tag(",")), expression)),
            tag(")"),
        ),
    )
        .parse(input)?;

    Ok((
        input,
        Expression::FunctionCall {
            name: name.into(),
            args,
        },
    ))
}

fn decimal(input: &str) -> IResult<&str, i128> {
    let (i, o) =
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)?;
    Ok((i, o.parse().unwrap()))
}

fn string_literal(input: &str) -> IResult<&str, String> {
    delimited(
        tag("'"),
        escaped_transform(
            none_of(r"\'"),
            '\\',
            alt((
                value("'", tag("'")),   // 输入匹配 "'" -> 输出单引号 "'"
                value("\"", tag("\"")), // 输入匹配 """ -> 输出双引号 """
                value("\\", tag("\\")), // 输入匹配 "\" -> 输出反斜杠 "\"
                value("\0", tag("0")),  // 输入匹配 "0" -> 输出NUL字符
                value("\n", tag("n")),  // 输入匹配 "n" -> 输出换行符
                value("\r", tag("r")),  // 输入匹配 "r" -> 输出回车符
                value("\t", tag("t")),  // 输入匹配 "t" -> 输出制表符
                value("%", tag("%")),   // 输入匹配 "%" -> 输出百分号
                value("_", tag("_")),   // 输入匹配 "_" -> 输出下划线
            )),
        ),
        tag("'"),
    )
    .parse(input)
}

fn literal(input: &str) -> IResult<&str, Expression> {
    alt((
        map(string_literal, |s| {
            Expression::Literal(LiteralValue::String(s))
        }),
        map(decimal, |i| Expression::Literal(LiteralValue::Integer(i))),
    ))
    .parse(input)
}

// macro_rules! binary_op_inner {
//     ($input:ident; $prev_name:ident:$prev_parser:ident $cur_name:ident:$cur_parser:ident $($rest_name:ident:$rest_parsers:ident)*) => {
//         let $cur_name = (
//             wss!($prev_name),
//             $cur_parser,
//             wss!($prev_name),
//         );
//         binary_op_inner!($input; $cur_name:$cur_parser $($rest_name:$rest_parsers)*);
//     };
//     ($input:ident; $last_name:ident:$last_parser:ident) => {
//         let (input, (exp1, op, exp2)) = $last_name.parse($input)?;
//         Ok((
//             input,
//             Expression::BinaryOp {
//                 left: Box::new(exp1),
//                 op,
//                 right: Box::new(exp2),
//             },
//         ))
//     };
// }

macro_rules! binary_op_fn {
    ($prev_name:ident:$prev_parser:ident $cur_name:ident:$cur_parser:ident $($rest_name:ident:$rest_parsers:ident)*) => {
        binary_op_fn!(__impl $cur_name $cur_parser $prev_name);
        binary_op_fn!($cur_name:$cur_parser $($rest_name:$rest_parsers)*);
    };
    (__impl $cur_name:ident $cur_parser:ident $prev_name:ident) => {
        fn $cur_name(input: &str) -> IResult<&str, Expression> {
            alt((map((wss!($prev_name),$cur_parser,wss!($cur_name)), |(exp1, op, exp2)| Expression::BinaryOp {left: Box::new(exp1), op, right: Box::new(exp2)}),
                $prev_name)).parse(input)
        }
    };
    ($last_name:ident:$last_parser:ident) => {};
}

macro_rules! declare_binary_op {
    ($first_name:ident:$first_parser:ident $($names:ident:$parsers:ident)+) => {
        fn $first_name(input: &str) -> IResult<&str, Expression> {
            alt((
                map(
                    (
                        wss!(alt((expression_in_brackets, expression_except_binary_op))),
                        $first_parser,
                        wss!($first_name),
                    ), |(exp1, op, exp2)| Expression::BinaryOp {left: Box::new(exp1), op, right: Box::new(exp2)}
                ),
                wss!(alt((expression_in_brackets, expression_except_binary_op)))
            )).parse(input)
        }
        binary_op_fn!($first_name:$first_parser $($names:$parsers)+);
    };
}

declare_binary_op!(a:binop5 b:binop4 c:binop3 d:binop2 binary_op:binop1);

fn expression_in_brackets(input: &str) -> IResult<&str, Expression> {
    delimited(
        tag("("),
        wss!(alt((expression_in_brackets, expression))),
        tag(")"),
    )
    .parse(input)
}

fn expression_except_binary_op(input: &str) -> IResult<&str, Expression> {
    alt((literal, function_call)).parse(input)
}

fn expression(input: &str) -> IResult<&str, Expression> {
    alt((binary_op, expression_except_binary_op)).parse(input)
}

// fn table(input: &str) -> IResult<&str, &str> {
//     identifier(input)
// }

// fn select_fields(input: &str) -> IResult<&str, Vec<Expression>> {
//     separated_list1(wss!(tag(",")), expression)(input)
// }

// fn select_clause(input: &str) -> IResult<&str, SelectClause> {
//     let select = tag_no_case("select");
//     let from = tag_no_case("from");
//     let (input, (_, _, fields, _, _, _, tb)) = (
//         select,
//         multispace1,
//         select_fields,
//         multispace1,
//         from,
//         multispace1,
//         table,
//     )
//         .parse(input)?;
//     Ok((input, SelectClause::new(fields, String::from(tb))))
// }
