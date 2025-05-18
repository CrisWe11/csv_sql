#[allow(dead_code, unused_imports, unused_variables)]
mod sql_parser {
    mod expression {
        pub enum BinaryOperator {
            Add,
            Subtract,
            Multiply,
            Divide,
            Modulo,
            Eq,
            NotEq,
            Lt,
            LtEq,
            Gt,
            GtEq,
            And,
            Or,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BinaryOperator {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        BinaryOperator::Add => "Add",
                        BinaryOperator::Subtract => "Subtract",
                        BinaryOperator::Multiply => "Multiply",
                        BinaryOperator::Divide => "Divide",
                        BinaryOperator::Modulo => "Modulo",
                        BinaryOperator::Eq => "Eq",
                        BinaryOperator::NotEq => "NotEq",
                        BinaryOperator::Lt => "Lt",
                        BinaryOperator::LtEq => "LtEq",
                        BinaryOperator::Gt => "Gt",
                        BinaryOperator::GtEq => "GtEq",
                        BinaryOperator::And => "And",
                        BinaryOperator::Or => "Or",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BinaryOperator {
            #[inline]
            fn clone(&self) -> BinaryOperator {
                match self {
                    BinaryOperator::Add => BinaryOperator::Add,
                    BinaryOperator::Subtract => BinaryOperator::Subtract,
                    BinaryOperator::Multiply => BinaryOperator::Multiply,
                    BinaryOperator::Divide => BinaryOperator::Divide,
                    BinaryOperator::Modulo => BinaryOperator::Modulo,
                    BinaryOperator::Eq => BinaryOperator::Eq,
                    BinaryOperator::NotEq => BinaryOperator::NotEq,
                    BinaryOperator::Lt => BinaryOperator::Lt,
                    BinaryOperator::LtEq => BinaryOperator::LtEq,
                    BinaryOperator::Gt => BinaryOperator::Gt,
                    BinaryOperator::GtEq => BinaryOperator::GtEq,
                    BinaryOperator::And => BinaryOperator::And,
                    BinaryOperator::Or => BinaryOperator::Or,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for BinaryOperator {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for BinaryOperator {
            #[inline]
            fn eq(&self, other: &BinaryOperator) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for BinaryOperator {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for BinaryOperator {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        pub enum UnaryOperator {
            Negate,
            Not,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UnaryOperator {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        UnaryOperator::Negate => "Negate",
                        UnaryOperator::Not => "Not",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UnaryOperator {
            #[inline]
            fn clone(&self) -> UnaryOperator {
                match self {
                    UnaryOperator::Negate => UnaryOperator::Negate,
                    UnaryOperator::Not => UnaryOperator::Not,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UnaryOperator {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UnaryOperator {
            #[inline]
            fn eq(&self, other: &UnaryOperator) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for UnaryOperator {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for UnaryOperator {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        pub enum LiteralValue {
            Null,
            Boolean(bool),
            Integer(i128),
            Float(f64),
            String(String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LiteralValue {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    LiteralValue::Null => ::core::fmt::Formatter::write_str(f, "Null"),
                    LiteralValue::Boolean(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Boolean",
                            &__self_0,
                        )
                    }
                    LiteralValue::Integer(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Integer",
                            &__self_0,
                        )
                    }
                    LiteralValue::Float(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Float",
                            &__self_0,
                        )
                    }
                    LiteralValue::String(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "String",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LiteralValue {
            #[inline]
            fn clone(&self) -> LiteralValue {
                match self {
                    LiteralValue::Null => LiteralValue::Null,
                    LiteralValue::Boolean(__self_0) => {
                        LiteralValue::Boolean(::core::clone::Clone::clone(__self_0))
                    }
                    LiteralValue::Integer(__self_0) => {
                        LiteralValue::Integer(::core::clone::Clone::clone(__self_0))
                    }
                    LiteralValue::Float(__self_0) => {
                        LiteralValue::Float(::core::clone::Clone::clone(__self_0))
                    }
                    LiteralValue::String(__self_0) => {
                        LiteralValue::String(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for LiteralValue {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for LiteralValue {
            #[inline]
            fn eq(&self, other: &LiteralValue) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            LiteralValue::Boolean(__self_0),
                            LiteralValue::Boolean(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            LiteralValue::Integer(__self_0),
                            LiteralValue::Integer(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            LiteralValue::Float(__self_0),
                            LiteralValue::Float(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            LiteralValue::String(__self_0),
                            LiteralValue::String(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        pub enum Expression {
            /// 字面量值, e.g., 1, 'hello', TRUE, NULL
            Literal(LiteralValue),
            /// 标识符, e.g., column_name, table.column_name
            /// 使用 Vec<String> 来支持 schema.table.column 这种形式
            Identifier(Vec<String>),
            /// 二元运算, e.g., price * quantity, count(*) > 0
            BinaryOp {
                left: Box<Expression>,
                op: BinaryOperator,
                right: Box<Expression>,
            },
            /// 一元运算, e.g., -price, NOT is_active
            UnaryOp { op: UnaryOperator, expr: Box<Expression> },
            /// 函数调用, e.g., COUNT(*), SUBSTRING(name, 1, 3), NOW()
            FunctionCall { name: String, args: Vec<Expression> },
            /// 特殊的通配符 '*', 主要用于 COUNT(*) 或 SELECT *
            /// 在表达式解析中，主要用于 COUNT(*) 这类场景
            Wildcard,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Expression {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Expression::Literal(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Literal",
                            &__self_0,
                        )
                    }
                    Expression::Identifier(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Identifier",
                            &__self_0,
                        )
                    }
                    Expression::BinaryOp {
                        left: __self_0,
                        op: __self_1,
                        right: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "BinaryOp",
                            "left",
                            __self_0,
                            "op",
                            __self_1,
                            "right",
                            &__self_2,
                        )
                    }
                    Expression::UnaryOp { op: __self_0, expr: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "UnaryOp",
                            "op",
                            __self_0,
                            "expr",
                            &__self_1,
                        )
                    }
                    Expression::FunctionCall { name: __self_0, args: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "FunctionCall",
                            "name",
                            __self_0,
                            "args",
                            &__self_1,
                        )
                    }
                    Expression::Wildcard => {
                        ::core::fmt::Formatter::write_str(f, "Wildcard")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Expression {
            #[inline]
            fn clone(&self) -> Expression {
                match self {
                    Expression::Literal(__self_0) => {
                        Expression::Literal(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::Identifier(__self_0) => {
                        Expression::Identifier(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::BinaryOp {
                        left: __self_0,
                        op: __self_1,
                        right: __self_2,
                    } => {
                        Expression::BinaryOp {
                            left: ::core::clone::Clone::clone(__self_0),
                            op: ::core::clone::Clone::clone(__self_1),
                            right: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    Expression::UnaryOp { op: __self_0, expr: __self_1 } => {
                        Expression::UnaryOp {
                            op: ::core::clone::Clone::clone(__self_0),
                            expr: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Expression::FunctionCall { name: __self_0, args: __self_1 } => {
                        Expression::FunctionCall {
                            name: ::core::clone::Clone::clone(__self_0),
                            args: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Expression::Wildcard => Expression::Wildcard,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Expression {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Expression {
            #[inline]
            fn eq(&self, other: &Expression) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            Expression::Literal(__self_0),
                            Expression::Literal(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Expression::Identifier(__self_0),
                            Expression::Identifier(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Expression::BinaryOp {
                                left: __self_0,
                                op: __self_1,
                                right: __self_2,
                            },
                            Expression::BinaryOp {
                                left: __arg1_0,
                                op: __arg1_1,
                                right: __arg1_2,
                            },
                        ) => {
                            __self_0 == __arg1_0 && __self_1 == __arg1_1
                                && __self_2 == __arg1_2
                        }
                        (
                            Expression::UnaryOp { op: __self_0, expr: __self_1 },
                            Expression::UnaryOp { op: __arg1_0, expr: __arg1_1 },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        (
                            Expression::FunctionCall { name: __self_0, args: __self_1 },
                            Expression::FunctionCall { name: __arg1_0, args: __arg1_1 },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        _ => true,
                    }
            }
        }
    }
    use std::os::unix::raw::ino_t;
    use crate::sql_parser::expression::Expression::Literal;
    use crate::sql_parser::expression::{BinaryOperator, LiteralValue, UnaryOperator};
    use expression::Expression;
    use nom::branch::alt;
    use nom::bytes::complete::{
        escaped_transform, tag, tag_no_case, take_until, take_until1,
    };
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
    struct SelectClause {
        exprs: Vec<Expression>,
        table: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SelectClause {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SelectClause",
                "exprs",
                &self.exprs,
                "table",
                &&self.table,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SelectClause {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SelectClause {
        #[inline]
        fn eq(&self, other: &SelectClause) -> bool {
            self.exprs == other.exprs && self.table == other.table
        }
    }
    impl SelectClause {
        fn new(fields: Vec<Expression>, table: String) -> Self {
            SelectClause {
                exprs: fields,
                table,
            }
        }
    }
    fn ws<'a, F, O, E: ParseError<&'a str>>(
        inner: F,
    ) -> impl Parser<&'a str, Output = O, Error = E>
    where
        F: Parser<&'a str, Output = O, Error = E>,
    {
        delimited(multispace0, inner, multispace0)
    }
    pub fn raw_identifier(input: &str) -> IResult<&str, &str> {
        recognize(
                pair(
                    alt((alpha1, tag("_"))),
                    many0_count(alt((alphanumeric1, tag("_")))),
                ),
            )
            .parse(input)
    }
    pub fn identifier(input: &str) -> IResult<&str, &str> {
        verify(
                raw_identifier,
                |output: &str| {
                    ![
                        "from",
                        "from".to_uppercase().as_str(),
                        "as",
                        "as".to_uppercase().as_str(),
                        "select",
                        "select".to_uppercase().as_str(),
                        "into",
                        "into".to_uppercase().as_str(),
                        "delete",
                        "delete".to_uppercase().as_str(),
                        "alter",
                        "alter".to_uppercase().as_str(),
                    ]
                        .contains(&output)
                },
            )
            .parse(input)
    }
    fn binop5(input: &str) -> IResult<&str, BinaryOperator> {
        map(
                alt((tag_no_case("*"), tag_no_case("/"), tag_no_case("%"))),
                |op| match op {
                    "*" => BinaryOperator::Multiply,
                    "/" => BinaryOperator::Divide,
                    "%" => BinaryOperator::Modulo,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("unrecognized binary operator"),
                            ),
                        );
                    }
                },
            )
            .parse(input)
    }
    fn binop4(input: &str) -> IResult<&str, BinaryOperator> {
        map(
                alt((tag_no_case("+"), tag_no_case("-"))),
                |op| match op {
                    "+" => BinaryOperator::Add,
                    "-" => BinaryOperator::Subtract,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("unrecognized binary operator"),
                            ),
                        );
                    }
                },
            )
            .parse(input)
    }
    fn binop3(input: &str) -> IResult<&str, BinaryOperator> {
        map(
                alt((
                    tag_no_case(">="),
                    tag_no_case("<="),
                    tag_no_case("<"),
                    tag_no_case(">"),
                    tag_no_case("="),
                    tag_no_case("!="),
                    tag_no_case("<>"),
                )),
                |op| match op {
                    ">=" => BinaryOperator::GtEq,
                    "<=" => BinaryOperator::LtEq,
                    "<" => BinaryOperator::Lt,
                    ">" => BinaryOperator::Gt,
                    "=" => BinaryOperator::Eq,
                    "!=" => BinaryOperator::NotEq,
                    "<>" => BinaryOperator::NotEq,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("unrecognized binary operator"),
                            ),
                        );
                    }
                },
            )
            .parse(input)
    }
    fn binop2(input: &str) -> IResult<&str, BinaryOperator> {
        map(
                tag_no_case("AND"),
                |op| match op {
                    "AND" => BinaryOperator::And,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("unrecognized binary operator"),
                            ),
                        );
                    }
                },
            )
            .parse(input)
    }
    fn binop1(input: &str) -> IResult<&str, BinaryOperator> {
        map(
                tag_no_case("OR"),
                |op| match op {
                    "OR" => BinaryOperator::Or,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("unrecognized binary operator"),
                            ),
                        );
                    }
                },
            )
            .parse(input)
    }
    fn unary_op(input: &str) -> IResult<&str, Expression> {
        ::core::panicking::panic("not yet implemented")
    }
    fn function_call(input: &str) -> IResult<&str, Expression> {
        let (input, (name, args)) = (
            identifier,
            delimited(
                tag("("),
                delimited(
                    multispace0,
                    separated_list0(
                        delimited(multispace0, tag(","), multispace0),
                        expression,
                    ),
                    multispace0,
                ),
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
        let (i, o) = recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))
            .parse(input)?;
        Ok((i, o.parse().unwrap()))
    }
    fn string_literal(input: &str) -> IResult<&str, String> {
        delimited(
                tag("'"),
                escaped_transform(
                    none_of(r"\'"),
                    '\\',
                    alt((
                        value("'", tag("'")),
                        value("\"", tag("\"")),
                        value("\\", tag("\\")),
                        value("\0", tag("0")),
                        value("\n", tag("n")),
                        value("\r", tag("r")),
                        value("\t", tag("t")),
                        value("%", tag("%")),
                        value("_", tag("_")),
                    )),
                ),
                tag("'"),
            )
            .parse(input)
    }
    fn literal(input: &str) -> IResult<&str, Expression> {
        alt((
                map(
                    string_literal,
                    |s| { Expression::Literal(LiteralValue::String(s)) },
                ),
                map(decimal, |i| Expression::Literal(LiteralValue::Integer(i))),
            ))
            .parse(input)
    }
    fn a(input: &str) -> IResult<&str, Expression> {
        map(
                (
                    delimited(
                        multispace0,
                        alt((expression_in_brackets, expression_except_binary_op)),
                        multispace0,
                    ),
                    binop5,
                    delimited(
                        multispace0,
                        alt((expression_in_brackets, expression)),
                        multispace0,
                    ),
                ),
                |(exp1, op, exp2)| Expression::BinaryOp {
                    left: Box::new(exp1),
                    op,
                    right: Box::new(exp2),
                },
            )
            .parse(input)
    }
    fn b(input: &str) -> IResult<&str, Expression> {
        alt((
                map(
                    (
                        delimited(multispace0, a, multispace0),
                        binop4,
                        delimited(multispace0, a, multispace0),
                    ),
                    |(exp1, op, exp2)| Expression::BinaryOp {
                        left: Box::new(exp1),
                        op,
                        right: Box::new(exp2),
                    },
                ),
                a,
            ))
            .parse(input)
    }
    fn c(input: &str) -> IResult<&str, Expression> {
        alt((
                map(
                    (
                        delimited(multispace0, b, multispace0),
                        binop3,
                        delimited(multispace0, b, multispace0),
                    ),
                    |(exp1, op, exp2)| Expression::BinaryOp {
                        left: Box::new(exp1),
                        op,
                        right: Box::new(exp2),
                    },
                ),
                b,
            ))
            .parse(input)
    }
    fn d(input: &str) -> IResult<&str, Expression> {
        alt((
                map(
                    (
                        delimited(multispace0, c, multispace0),
                        binop2,
                        delimited(multispace0, c, multispace0),
                    ),
                    |(exp1, op, exp2)| Expression::BinaryOp {
                        left: Box::new(exp1),
                        op,
                        right: Box::new(exp2),
                    },
                ),
                c,
            ))
            .parse(input)
    }
    fn binary_op(input: &str) -> IResult<&str, Expression> {
        alt((
                map(
                    (
                        delimited(multispace0, d, multispace0),
                        binop1,
                        delimited(multispace0, d, multispace0),
                    ),
                    |(exp1, op, exp2)| Expression::BinaryOp {
                        left: Box::new(exp1),
                        op,
                        right: Box::new(exp2),
                    },
                ),
                d,
            ))
            .parse(input)
    }
    fn expression_in_brackets(input: &str) -> IResult<&str, Expression> {
        delimited(
                tag("("),
                delimited(
                    multispace0,
                    alt((expression_in_brackets, expression)),
                    multispace0,
                ),
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
}
