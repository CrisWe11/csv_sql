use crate::sql::expression::{BinOp, BinaryOperator, Expression, LiteralValue};
use crate::sql::parser::{function_call, literal, binary_op};
use crate::sql::*;
// #[test]
// fn test_basic_select() {
//     let sql = "select 1 from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause::new(
//                 vec![Expression::Literal(LiteralValue::Integer(1))],
//                 String::from("test")
//             )
//         )),
//         select_clause(sql)
//     );
// }

#[test]
fn test_literal() {
    let integer = "1";
    let str = "'this is a test'";
    assert_eq!(
        literal(integer),
        Ok(("", Expression::new_literal(LiteralValue::Integer(1), "1"))),
    );
    assert_eq!(
        literal(str),
        Ok((
            "",
            Expression::new_literal(
                LiteralValue::String("this is a test".to_string()),
                "'this is a test'"
            )
        )),
    );
}

#[test]
fn test_func1() {
    let exp = "a()";
    assert_eq!(
        function_call(exp),
        Ok(("", Expression::new_function_call("a", vec![], "a()"))),
    )
}

#[test]
fn test_func2() {
    let exp = "a(a())";
    assert_eq!(
        function_call(exp),
        Ok((
            "",
            Expression::new_function_call(
                "a",
                vec![Expression::new_function_call("a", vec![], "a()")],
                "a(a())"
            )
        )),
    )
}

#[test]
fn test_func3() {
    let exp = "a(1,2, 'test')";
    assert_eq!(
        function_call(exp),
        Ok((
            "",
            Expression::new_function_call(
                "a",
                vec![
                    Expression::new_literal(LiteralValue::Integer(1), "1"),
                    Expression::new_literal(LiteralValue::Integer(2), "2"),
                    Expression::new_literal(LiteralValue::String(String::from("test")), "'test'"),
                ],
                exp
            )
        )),
    )
}

#[test]
fn test_string_literals() {
    let sql = r"'test\\test'";
    assert_eq!(literal(sql), Ok(("", Expression::new_literal(LiteralValue::String(String::from(r"test\test")), sql))));
}

#[test]
fn test_binary_op() {
    let sql = r"1 + 1 + 1";
    assert_eq!(
        binary_op(sql),
        Ok((
            "",
            Expression::new_binary_op(
                Expression::new_literal(LiteralValue::Integer(1), "1"),
                BinaryOperator::new(BinOp::Add, "+"),
                Expression::new_binary_op(
                    Expression::new_literal(LiteralValue::Integer(1), "1"),
                    BinaryOperator::new(BinOp::Add, "+"),
                    Expression::new_literal(LiteralValue::Integer(1), "1"),
                    "1 + 1"
                ),
                sql
            )
        ))
    );
}

#[test]
fn test_binary_op2() {
    let sql = r"1 * 2 + 1";
    assert_eq!(
        binary_op(sql),
        Ok((
            "",
            Expression::new_binary_op(
                Expression::new_binary_op(
                    Expression::new_literal(LiteralValue::Integer(1), "1"),
                    BinaryOperator::new(BinOp::Multiply, "*"),
                    Expression::new_literal(LiteralValue::Integer(2), "2"),
                    "1 * 2"
                ),
                BinaryOperator::new(BinOp::Add, "+"),
                Expression::new_literal(LiteralValue::Integer(1), "1"),
                sql
            )
        ))
    );
}

// #[test]
// fn test_binary_op3() {
//     let sql = r"1 + 1 * 2 + 1";
//     assert_eq!(
//         binary_op(sql),
//         Ok((
//             "",
//             BinaryOp {
//                 left: Box::new(Literal(LiteralValue::Integer(1))),
//                 op: BinaryOperator::Add,
//                 right: Box::new(BinaryOp {
//                     left: Box::new(BinaryOp {
//                         left: Box::new(Literal(LiteralValue::Integer(1))),
//                         op: BinaryOperator::Multiply,
//                         right: Box::new(Literal(LiteralValue::Integer(2))),
//                     }),
//                     op: BinaryOperator::Add,
//                     right: Box::new(Literal(LiteralValue::Integer(1))),
//                 }),
//             }
//         ))
//     );
// }
//
// #[test]
// fn test_binary_op4() {
//     let sql = r"2*2+2*2";
//     assert_eq!(
//         binary_op(sql),
//         Ok((
//             "",
//             BinaryOp {
//                 left: Box::new(BinaryOp {
//                     left: Box::new(Literal(LiteralValue::Integer(2))),
//                     op: BinaryOperator::Multiply,
//                     right: Box::new(Literal(LiteralValue::Integer(2))),
//                 }),
//                 op: BinaryOperator::Add,
//                 right: Box::new(BinaryOp {
//                     left: Box::new(Literal(LiteralValue::Integer(2))),
//                     op: BinaryOperator::Multiply,
//                     right: Box::new(Literal(LiteralValue::Integer(2))),
//                 }),
//             }
//         ))
//     );
// }
//
// #[test]
// fn test_binary_op5() {
//     let sql = r"(1 + 2)*2+(2+3*5)*2";
//     assert_eq!(
//         binary_op(sql),
//         Ok((
//             "",
//             BinaryOp {
//                 left: Box::new(BinaryOp {
//                     left: Box::new(BinaryOp {
//                         left: Box::new(Literal(LiteralValue::Integer(1))),
//                         op: BinaryOperator::Add,
//                         right: Box::new(Literal(LiteralValue::Integer(2))),
//                     }),
//                     op: BinaryOperator::Multiply,
//                     right: Box::new(Literal(LiteralValue::Integer(2))),
//                 }),
//                 op: BinaryOperator::Add,
//                 right: Box::new(BinaryOp {
//                     left: Box::new(BinaryOp {
//                         left: Box::new(Literal(LiteralValue::Integer(2))),
//                         op: BinaryOperator::Add,
//                         right: Box::new(BinaryOp {
//                             left: Box::new(Literal(LiteralValue::Integer(3))),
//                             op: BinaryOperator::Multiply,
//                             right: Box::new(Literal(LiteralValue::Integer(5))),
//                         }),
//                     }),
//                     op: BinaryOperator::Multiply,
//                     right: Box::new(Literal(LiteralValue::Integer(2))),
//                 }),
//             }
//         ))
//     );
// }

// #[test]
// fn test_take_until() {
//     let sql = "string";
//     take_until1(r"a")
//
//     assert_eq!(
//         Ok((
//             "",
//             String::from("string")
//         )),
//
//     );
// }

// #[test]
// fn test_basic_select() {
//     let sql = "select sales from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![SelectField {
//                     field: "sales".to_string(),
//                     alias: None
//                 }],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_with_single_alias() {
//     let sql = "select sales as s from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![SelectField {
//                     field: "sales".to_string(),
//                     alias: Some("s".to_string())
//                 }],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_two_fields_one_alias() {
//     let sql = "select sales as s, num from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![
//                     SelectField {
//                         field: "sales".to_string(),
//                         alias: Some("s".to_string())
//                     },
//                     SelectField {
//                         field: "num".to_string(),
//                         alias: None
//                     }
//                 ],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_two_fields_second_alias() {
//     let sql = "select sales, num as n from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![
//                     SelectField {
//                         field: "sales".to_string(),
//                         alias: None
//                     },
//                     SelectField {
//                         field: "num".to_string(),
//                         alias: Some("n".to_string())
//                     }
//                 ],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_two_fields_both_with_as_alias() {
//     let sql = "select sales as s, num as n from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![
//                     SelectField {
//                         field: "sales".to_string(),
//                         alias: Some("s".to_string())
//                     },
//                     SelectField {
//                         field: "num".to_string(),
//                         alias: Some("n".to_string())
//                     }
//                 ],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_two_fields_space_alias() {
//     let sql = "select sales s, num n from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![
//                     SelectField {
//                         field: "sales".to_string(),
//                         alias: Some("s".to_string())
//                     },
//                     SelectField {
//                         field: "num".to_string(),
//                         alias: Some("n".to_string())
//                     }
//                 ],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_count_star() {
//     let sql = "select count(*) from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![SelectField {
//                     field: "count(*)".to_string(),
//                     alias: None
//                 }],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_count_star_with_alias() {
//     let sql = "select count(*) c from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![SelectField {
//                     field: "count(*)".to_string(),
//                     alias: Some("c".to_string())
//                 }],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_count_star_and_field() {
//     let sql = "select count(*),sales from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![
//                     SelectField {
//                         field: "count(*)".to_string(),
//                         alias: None
//                     },
//                     SelectField {
//                         field: "sales".to_string(),
//                         alias: None
//                     }
//                 ],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_count_star_and_field_with_aliases() {
//     let sql = "select count(*) c,sales s from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![
//                     SelectField {
//                         field: "count(*)".to_string(),
//                         alias: Some("c".to_string())
//                     },
//                     SelectField {
//                         field: "sales".to_string(),
//                         alias: Some("s".to_string())
//                     }
//                 ],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_three_fields_with_aliases() {
//     let sql = "select count(*) c,sales s,num as n from test";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![
//                     SelectField {
//                         field: "count(*)".to_string(),
//                         alias: Some("c".to_string())
//                     },
//                     SelectField {
//                         field: "sales".to_string(),
//                         alias: Some("s".to_string())
//                     },
//                     SelectField {
//                         field: "num".to_string(),
//                         alias: Some("n".to_string())
//                     }
//                 ],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_with_table_alias() {
//     let sql = "select t.sales from test t";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![SelectField {
//                     field: "t.sales".to_string(),
//                     alias: None
//                 }],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
//
// #[test]
// fn test_select_different_field_with_table_alias() {
//     let sql = "select t.num from test t";
//     assert_eq!(
//         Ok((
//             "",
//             SelectClause {
//                 exprs: vec![SelectField {
//                     field: "t.num".to_string(),
//                     alias: None
//                 }],
//                 table: "test".to_string()
//             }
//         )),
//         select_clause(sql)
//     );
// }
