use crate::sql;
use crate::sql::expression::Expression;
use crate::sql::select::FieldId;
use nom::error::Error;
use thiserror::Error;

// parse error
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SelectError<T> {
    #[error("not select clause")]
    NotSelectClause,
    #[error("nom::error {0:?}")]
    ParseError(#[from] nom::Err<nom::error::Error<T>>),
    #[error("duplicated field identifier {field_id:?} in expression {expression:?}")]
    DuplicatedFieldId {
        field_id: String,
        expression: String,
    },
}

// tokenize error
#[derive(Error, Debug)]
pub enum SqlParseError<T> {
    #[error(transparent)]
    SelectError(#[from] SelectError<T>),
}

impl<T> From<SelectError<T>> for nom::Err<SqlParseError<T>> {
    fn from(value: SelectError<T>) -> Self {
        match value {
            SelectError::NotSelectClause
            | SelectError::DuplicatedFieldId { .. }
            | SelectError::ParseError(..) => nom::Err::Error(value.into()),
        }
    }
}
