use super::error::SelectError;
use super::expression::Expression;
use crate::sql::parser::{select_fields, table};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::Parser;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum FieldId<'a> {
    Identifier(&'a str),
    GeneratedIdentifier(String),
}

impl<'a> FieldId<'a> {
    pub fn get_identifier(self) -> String {
        match self {
            FieldId::Identifier(ident) => String::from(ident),
            FieldId::GeneratedIdentifier(ident) => ident,
        }
    }
}

impl<'a> Hash for FieldId<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FieldId::Identifier(identifier) => {
                identifier.hash(state);
            }
            FieldId::GeneratedIdentifier(id_string) => {
                id_string.hash(state);
            }
        }
    }
}

impl<'a> PartialEq for FieldId<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldId::Identifier(id1), FieldId::Identifier(id2)) => id1 == id2,
            (FieldId::GeneratedIdentifier(id1), FieldId::GeneratedIdentifier(id2)) => id1 == id2,
            (FieldId::Identifier(id1), FieldId::GeneratedIdentifier(id2)) => id1 == id2,
            (FieldId::GeneratedIdentifier(id1), FieldId::Identifier(id2)) => id1 == id2,
        }
    }
}

impl<'a> Eq for FieldId<'a> {}

#[derive(Debug)]
pub struct SelectClause<'a> {
    fields: HashMap<FieldId<'a>, Expression<'a>>,
    table: &'a str,
}

impl<'a> SelectClause<'a> {
    pub fn new<T>(
        mut initial_fields: Vec<(Expression<'a>, Option<FieldId<'a>>)>,
        table: &'a str,
    ) -> Result<SelectClause<'a>, SelectError<T>> {
        let mut fields: HashMap<FieldId<'a>, Expression<'a>> =
            HashMap::with_capacity(initial_fields.len());
        // generate field identifier for unaliased field
        let mut idx: usize = 0;
        // deal with aliased fields first
        initial_fields.sort_by_key(|f| f.1.is_none());
        for field in initial_fields {
            match field.1 {
                Some(field_id) => {
                    if fields.contains_key(&field_id) {
                        let (dup_field_id, dup_expr) = fields
                            .remove_entry(&field_id)
                            .expect("get field by field_id failed");
                        return Err(SelectError::DuplicatedFieldId {
                            field_id: dup_field_id.get_identifier(),
                            expression: dup_expr.get_org(),
                        });
                    }
                    fields.insert(field_id, field.0);
                }
                None => {
                    loop {
                        let generated_identifier =
                            FieldId::GeneratedIdentifier(format!("_field{}_", idx));
                        idx += 1;
                        if !fields.contains_key(&generated_identifier) {
                            fields.insert(generated_identifier, field.0);
                            break;
                        }
                    }
                }
            }
        }
        Ok(SelectClause { fields, table })
    }
}

// impl<'a> TryFrom<&'a str> for SelectClause<'a> {
//     type Error = SelectError<'a>;
//     fn try_from(value: &'a str) -> Result<Self, Self::Error> {
//         let select = tag_no_case("select");
//         let from = tag_no_case("from");
//         let (input, (_, _, fields, _, _, _, tb)) = (
//             select,
//             multispace1,
//             select_fields,
//             multispace1,
//             from,
//             multispace1,
//             table,
//         )
//             .parse(value)?;
//         Ok(SelectClause::new(fields, tb)?)
//     }
// }
