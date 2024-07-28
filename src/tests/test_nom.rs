#[cfg(test)]
mod test_nom {
    use clap::builder::TypedValueParser;
    use nom::bytes::complete::{is_not, tag, tag_no_case, take_until, take_while, take_while1};
    use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, char, multispace0, multispace1, space0};
    use nom::{Compare, error, InputIter, InputLength, InputTake, IResult, Parser};
    use nom::branch::alt;
    use nom::sequence::{delimited, pair, preceded, separated_pair, Tuple, tuple};
    use nom::combinator::{not, recognize};
    use nom::Err::Error;
    use nom::error::{ErrorKind, ParseError};
    use nom::multi::{many0_count, separated_list1};

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

    // fn optional<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<Option<&'a str>, O, E>
    //     where F: Parser<&'a str, O, E>
    // {
    //     unimplemented!()
    // }

    fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
    where
        F: Parser<&'a str, O, E>,
    {
        delimited(
            multispace0,
            inner,
            multispace0,
        )
    }

    // fn better_not<T, Input, E>(t: T) -> impl FnMut(Input) -> IResult<(), Input, E>
    // where
    //     T: InputIter + InputTake + InputLength + Clone,
    //     Input: InputTake + Compare<T> + Clone,
    //     E: ParseError<Input>
    // {
    //     let new_t = t.clone();
    //     move |input| {
    //         let mut not_tag = not(tag(new_t));
    //         not_tag.parse(input).map(|(i, _)| ((), i))
    //     }
    // }

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
            Err(nom::Err::Error(ParseError::from_error_kind(input, ErrorKind::Fail)))
        } else {
            Ok((input, SelectField::new(String::from(output), None)))
        }
    }

    fn field_col_alias(input: &str) -> IResult<&str, SelectField> {
        let (input, (col, alias)) = separated_pair(identifier, multispace1, identifier)(input)?;
        Ok((input, SelectField::new(String::from(col), Some(String::from(alias)))))
    }

    fn field_col_as_alias(input: &str) -> IResult<&str, SelectField> {
        let (input, (col, _, _, _, alias)) = tuple((identifier, multispace1, tag("as"), multispace1, identifier)).parse(input)?;
        Ok((input, SelectField::new(String::from(col), Some(String::from(alias)))))
    }

    // col1
    // col1 c1
    // col1 as c1
    fn select_field(input: &str) -> IResult<&str, SelectField> {
        alt((field_col, field_col_alias, field_col_as_alias))(input)
    }


    fn select_fields(input: &str) -> IResult<&str, Vec<SelectField>> {
        separated_list1(ws(tag(",")), select_field)(input)
    }

    fn select_clause(input: &str) -> IResult<&str, Select> {
        let select = tag_no_case("select");
        let from = tag_no_case("from");
        let (input, (_, _, fields, _, _, _, tb)) = tuple((select, multispace1, select_fields, multispace1, from, multispace1, table)).parse(input)?;
        Ok((input, Select::new(fields, String::from(tb))))
    }

    #[test]
    fn test1() {
        let sql0_0 = "select sales from test";
        let sql0_1 = "select sales as s from test";
        let sql0_2 = "select sales as s, num from test";
        let sql0_3 = "select sales, num as n from test";
        let sql0_4 = "select sales as s, num as n from test";
        let sql0_4 = "select sales s, num n from test";
        let sql0_5 = "select count(*) from test";
        let sql0_6 = "select count(*) c from test";
        let sql0_7 = "select count(*),sales from test";
        let sql0_8 = "select count(*) c,sales s from test";
        let sql0_9 = "select count(*) c,sales s,num as n from test";
        let sql0_10 = "select t.sales from test t";
        let sql0_11 = "select t.num from test t";
    }

    #[test]
    fn test2() {
        let test = tag::<&str, &str, nom::error::Error<&str>>("test");
        let mut not_test = not2(test);
        println!("{:?}", not_test("testa"));
    }
}