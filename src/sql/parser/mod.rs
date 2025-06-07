use crate::sql::error::{SelectError, SqlParseError};
use crate::sql::expression::{BinOp, BinaryOperator, Exp, Expression, LiteralValue};
use crate::sql::select::{FieldId, SelectClause};
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, tag, tag_no_case};
use nom::character::complete::{
    alpha1, alphanumeric1, char, multispace0, multispace1, none_of, one_of,
};
use nom::combinator::{consumed, map, peek, recognize, value, verify};
use nom::error::ParseError;
use nom::multi::{many0, many0_count, many1, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, terminated};
use nom::{IResult, Parser};

pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

macro_rules! wss {
    ($inner:expr) => {
        delimited(multispace0, $inner, multispace0)
    };
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
                    $key => BinaryOperator::new($value, op),
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
                    $($key => BinaryOperator::new($value, op),)*
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

declare_binary_operator!(binop5, {"*" => BinOp::Multiply, "/" => BinOp::Divide, "%" => BinOp::Modulo});
declare_binary_operator!(binop4, {"+" => BinOp::Add, "-" => BinOp::Subtract});
declare_binary_operator!(binop3, {">=" => BinOp::GtEq, "<=" => BinOp::LtEq, "<" => BinOp::Lt, ">" => BinOp::Gt, "=" => BinOp::Eq, "!=" => BinOp::NotEq, "<>" => BinOp::NotEq});
declare_binary_operator!(binop2, {"AND" => BinOp::And});
declare_binary_operator!(binop1, {"OR" => BinOp::Or});

pub fn unary_op(input: &str) -> IResult<&str, Expression> {
    todo!()
}

pub fn function_call(input: &str) -> IResult<&str, Expression> {
    let (input, (consumed_input, (name, args))) = consumed((
        identifier,
        delimited(
            tag("("),
            wss!(separated_list0(wss!(tag(",")), expression)),
            tag(")"),
        ),
    ))
    .parse(input)?;

    Ok((
        input,
        Expression::new_function_call(name, args, consumed_input),
    ))
}

pub fn decimal(input: &str) -> IResult<&str, i128> {
    let (i, o) =
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)?;
    Ok((i, o.parse().unwrap()))
}

pub fn string_literal(input: &str) -> IResult<&str, String> {
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

pub fn literal(input: &str) -> IResult<&str, Expression> {
    alt((
        map(consumed(string_literal), |(consumed_input, s)| {
            Expression::new_literal(LiteralValue::String(s), consumed_input)
        }),
        map(consumed(decimal), |(consumed_input, i)| {
            Expression::new_literal(LiteralValue::Integer(i), consumed_input)
        }),
    ))
    .parse(input)
}

macro_rules! binary_op_fn {
    ($prev_name:ident:$prev_parser:ident $cur_name:ident:$cur_parser:ident $($rest_name:ident:$rest_parsers:ident)*) => {
        binary_op_fn!(__impl $cur_name $cur_parser $prev_name);
        binary_op_fn!($cur_name:$cur_parser $($rest_name:$rest_parsers)*);
    };
    (__impl $cur_name:ident $cur_parser:ident $prev_name:ident) => {
        pub fn $cur_name(input: &str) -> IResult<&str, Expression> {
            alt((map(consumed((wss!($prev_name),$cur_parser,wss!($cur_name))), |(consumed_input, (exp1, op, exp2))| Expression::new_binary_op(exp1, op, exp2, consumed_input)),
                $prev_name)).parse(input)
        }
    };
    ($last_name:ident:$last_parser:ident) => {};
}

macro_rules! declare_binary_op {
    ($first_name:ident:$first_parser:ident $($names:ident:$parsers:ident)+) => {
        pub fn $first_name(input: &str) -> IResult<&str, Expression> {
            alt((
                map(
                    consumed((
                        wss!(alt((expression_in_brackets, expression_except_binary_op))),
                        $first_parser,
                        wss!($first_name),
                    )), |(consumed_input, (exp1, op, exp2))| Expression::new_binary_op(exp1, op, exp2, consumed_input)
                ),
                wss!(alt((expression_in_brackets, expression_except_binary_op)))
            )).parse(input)
        }
        binary_op_fn!($first_name:$first_parser $($names:$parsers)+);
    };
}

declare_binary_op!(a:binop5 b:binop4 c:binop3 d:binop2 binary_op:binop1);

pub fn expression_in_brackets(input: &str) -> IResult<&str, Expression> {
    delimited(
        tag("("),
        wss!(alt((expression_in_brackets, expression))),
        tag(")"),
    )
    .parse(input)
}

pub fn expression_except_binary_op(input: &str) -> IResult<&str, Expression> {
    alt((literal, function_call)).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    alt((binary_op, expression_except_binary_op)).parse(input)
}

pub fn table(input: &str) -> IResult<&str, &str> {
    identifier(input)
}

pub fn select_fields(input: &str) -> IResult<&str, Vec<(Expression, Option<FieldId>)>> {
    let expression_as_alias = map(
        (expression, multispace1, tag("as"), identifier),
        |(exp, _, _, alias)| (exp, Some(FieldId::Identifier(alias))),
    );
    let expression_alias = map((expression, multispace1, identifier), |(exp, _, alias)| {
        (exp, Some(FieldId::Identifier(alias)))
    });
    let expression_no_alias = map(expression, |exp| (exp, Option::<FieldId>::None));
    separated_list1(
        wss!(tag(",")),
        alt((expression_as_alias, expression_alias, expression_no_alias)),
    )
    .parse(input)
}

pub fn select_clause(input: &str) -> IResult<&str, SelectClause, SqlParseError<&str>> {
    // TODO: remove error type dedication here
    if peek((multispace0::<&str, nom::error::Error<&str>>, tag_no_case("select"), multispace1))
        .parse(input)
        .is_err()
    {
        // not a select clause, will not do further parsing
        // try other branches maybe
        return Err(nom::Err::Error(SqlParseError::SelectError(
            SelectError::NotSelectClause,
        )));
    }

    // failure will cause a Failure (unrecoverable error)
    let (input, (_, _, _, fields, _, _, _, tb)) = (
        multispace0,
        tag_no_case("select"),
        multispace1,
        select_fields,
        multispace1,
        tag_no_case("from"),
        multispace1,
        table,
    )
        .parse(input)
        .map_err(|e| SelectError::from(e))?;
    Ok((input, SelectClause::new(fields, tb)?))
}
