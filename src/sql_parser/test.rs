use super::*;

#[test]
fn test_basic_select() {
    let sql = "select sales from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![SelectField {
                    field: "sales".to_string(),
                    alias: None
                }],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_with_single_alias() {
    let sql = "select sales as s from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![SelectField {
                    field: "sales".to_string(),
                    alias: Some("s".to_string())
                }],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_two_fields_one_alias() {
    let sql = "select sales as s, num from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![
                    SelectField {
                        field: "sales".to_string(),
                        alias: Some("s".to_string())
                    },
                    SelectField {
                        field: "num".to_string(),
                        alias: None
                    }
                ],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_two_fields_second_alias() {
    let sql = "select sales, num as n from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![
                    SelectField {
                        field: "sales".to_string(),
                        alias: None
                    },
                    SelectField {
                        field: "num".to_string(),
                        alias: Some("n".to_string())
                    }
                ],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_two_fields_both_with_as_alias() {
    let sql = "select sales as s, num as n from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![
                    SelectField {
                        field: "sales".to_string(),
                        alias: Some("s".to_string())
                    },
                    SelectField {
                        field: "num".to_string(),
                        alias: Some("n".to_string())
                    }
                ],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_two_fields_space_alias() {
    let sql = "select sales s, num n from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![
                    SelectField {
                        field: "sales".to_string(),
                        alias: Some("s".to_string())
                    },
                    SelectField {
                        field: "num".to_string(),
                        alias: Some("n".to_string())
                    }
                ],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_count_star() {
    let sql = "select count(*) from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![SelectField {
                    field: "count(*)".to_string(),
                    alias: None
                }],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_count_star_with_alias() {
    let sql = "select count(*) c from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![SelectField {
                    field: "count(*)".to_string(),
                    alias: Some("c".to_string())
                }],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_count_star_and_field() {
    let sql = "select count(*),sales from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![
                    SelectField {
                        field: "count(*)".to_string(),
                        alias: None
                    },
                    SelectField {
                        field: "sales".to_string(),
                        alias: None
                    }
                ],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_count_star_and_field_with_aliases() {
    let sql = "select count(*) c,sales s from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![
                    SelectField {
                        field: "count(*)".to_string(),
                        alias: Some("c".to_string())
                    },
                    SelectField {
                        field: "sales".to_string(),
                        alias: Some("s".to_string())
                    }
                ],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_three_fields_with_aliases() {
    let sql = "select count(*) c,sales s,num as n from test";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![
                    SelectField {
                        field: "count(*)".to_string(),
                        alias: Some("c".to_string())
                    },
                    SelectField {
                        field: "sales".to_string(),
                        alias: Some("s".to_string())
                    },
                    SelectField {
                        field: "num".to_string(),
                        alias: Some("n".to_string())
                    }
                ],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_with_table_alias() {
    let sql = "select t.sales from test t";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![SelectField {
                    field: "t.sales".to_string(),
                    alias: None
                }],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}

#[test]
fn test_select_different_field_with_table_alias() {
    let sql = "select t.num from test t";
    assert_eq!(
        Ok((
            "",
            Select {
                fields: vec![SelectField {
                    field: "t.num".to_string(),
                    alias: None
                }],
                table: "test".to_string()
            }
        )),
        select_clause(sql)
    );
}
