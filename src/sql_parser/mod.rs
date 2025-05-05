#[cfg(test)]
mod test;

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{
    alpha1, alphanumeric1, multispace0, multispace1,
};
use nom::combinator::{recognize, verify};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0_count, separated_list1};
use nom::sequence::{delimited, pair, separated_pair, tuple, Tuple};
use nom::{Compare, IResult, InputIter, InputLength, InputTake, Parser};

#[derive(Debug, PartialEq)]
struct Select {
    fields: Vec<SelectField>,
    table: String,
}

impl Select {
    fn new(fields: Vec<SelectField>, table: String) -> Self {
        Select { fields, table }
    }
}

#[derive(Debug, PartialEq)]
struct SelectField {
    field: String,
    alias: Option<String>,
}

impl SelectField {
    fn new(field: String, alias: Option<String>) -> Self {
        SelectField { field, alias }
    }
}

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
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

fn table(input: &str) -> IResult<&str, &str> {
    identifier(input)
}

fn field_col(input: &str) -> IResult<&str, SelectField> {
    let (input, output) = identifier(input)?;
    if input.is_empty() {
        Err(nom::Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Fail,
        )))
    } else {
        Ok((input, SelectField::new(String::from(output), None)))
    }
}

fn field_col_alias(input: &str) -> IResult<&str, SelectField> {
    let (input, (col, alias)) = separated_pair(identifier, multispace1, identifier)(input)?;
    Ok((
        input,
        SelectField::new(String::from(col), Some(String::from(alias))),
    ))
}

fn field_col_as_alias(input: &str) -> IResult<&str, SelectField> {
    let (input, (col, _, _, _, alias)) =
        tuple((identifier, multispace1, tag("as"), multispace1, identifier)).parse(input)?;
    Ok((
        input,
        SelectField::new(String::from(col), Some(String::from(alias))),
    ))
}
fn select_field(input: &str) -> IResult<&str, SelectField> {
    alt((field_col, field_col_alias, field_col_as_alias))(input)
}

fn select_fields(input: &str) -> IResult<&str, Vec<SelectField>> {
    separated_list1(ws(tag(",")), select_field)(input)
}

fn select_clause(input: &str) -> IResult<&str, Select> {
    let select = tag_no_case("select");
    let from = tag_no_case("from");
    let (input, (_, _, fields, _, _, _, tb)) = tuple((
        select,
        multispace1,
        select_fields,
        multispace1,
        from,
        multispace1,
        table,
    ))
    .parse(input)?;
    Ok((input, Select::new(fields, String::from(tb))))
}
