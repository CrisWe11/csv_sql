
#[cfg(test)]
mod test;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, tag_no_case, take_until, take_while, take_while1};
use nom::character::complete::{
    alpha1, alphanumeric0, alphanumeric1, char, multispace0, multispace1, space0,
};
use nom::combinator::{not, recognize};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0_count, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple, Tuple};
use nom::Err::Error;
use nom::{error, Compare, IResult, InputIter, InputLength, InputTake, Parser};

#[derive(Debug)]
struct Select {
    fields: Vec<SelectField>,
    table: String,
}

impl Select {
    fn new(fields: Vec<SelectField>, table: String) -> Self {
        Select { fields, table }
    }
}

#[derive(Debug)]
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

pub fn not2<I, O, E: ParseError<I>, F>(mut parser: F) -> impl FnMut(I) -> IResult<(), I, E>
where
    F: Parser<I, O, E>,
    I: Clone,
{
    move |input: I| {
        let i = input.clone();
        match parser.parse(input) {
            Ok(_) => Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Not))),
            Err(_) => Ok(((), i)),
        }
    }
}

macro_rules! identifier {
        ($($x: literal),*) => (
            pub fn identifier<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
                let (new_input, output) = recognize(
                    pair(
                        alt((alpha1, tag("_"))),
                        many0_count(alt((alphanumeric1, tag("_")))),
                    )
                ).parse(input)?;
                match output {
                    $($x)|* => Err(Error(E::from_error_kind(input, ErrorKind::Tag))),
                    _ => Ok((new_input, output))
                }
            }
        );
    }

identifier!("from", "as", "select", "into", "delete", "alter");

fn table(input: &str) -> IResult<&str, &str> {
    identifier(input)
}

fn field_col(input: &str) -> IResult<&str, SelectField> {
    let (input, output) = identifier(input)?;
    if !input.is_empty() {
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
