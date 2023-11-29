#[cfg(test)]
mod test_nom {
    use nom::bytes::complete::{tag, tag_no_case, take_until, take_while, take_while1};
    use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, char, multispace0, space0};
    use nom::{IResult, Parser};
    use nom::branch::alt;
    use nom::sequence::{delimited, pair, Tuple};
    use nom::character::{is_alphanumeric, is_space};
    use nom::combinator::recognize;
    use nom::error::ParseError;
    use nom::multi::{many0_count, separated_list1};

    #[derive(Debug)]
    struct Select {
        fields: Vec<SelectField>,
        table: String,
    }

    #[derive(Debug)]
    struct SelectField {
        fields: String,
    }

    fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
        where F: Parser<&'a str, O, E> {
        delimited(
            multispace0,
            inner,
            multispace0,
        )
    }

    fn identifier(input: &str) -> IResult<&str, &str> {
        recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0_count(alt((alphanumeric1, tag("_")))),
            )
        ).parse(input)
    }

    fn select_field(input: &str) -> IResult<&str, SelectField> {
        let (input, f) = identifier(input)?;
        Ok((
            input, SelectField { fields: String::from(f) }
        ))
    }

    fn table(input: &str) -> IResult<&str, &str> {
        identifier(input)
    }

    fn select_clause(input: &str) -> IResult<&str, Select> {
        let select = tag_no_case("select");
        let from = tag_no_case("from");
        let sep = char(',');
        let fields = separated_list1(sep, select_field);

        let (input, (_, f, _, t)) = (ws(select), ws(fields), ws(from), ws(table)).parse(input)?;
        Ok((input,
            Select {
                fields: f,
                table: String::from(t),
            }))
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
        println!("{:?}", select_clause(sql0_0).unwrap());
    }
}