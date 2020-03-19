//
// error.rs
//
use self::Error::*;
use std::fmt::{Display, Formatter, Result};
use validator::ValidationErrors;

pub enum Error {
    ValidationError,

    // Access
    AccessDenied,
}

pub struct ValidationError {
    field: String,
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AccessDenied => write!(f, "Access denied"),
            ValidationError => write!(f, "Error"),
        }
    }
}

pub fn from_validation_errors(e: ValidationErrors) -> Vec<ValidationError> {
    let field_errors = e.field_errors();
    field_errors
        .iter()
        .map(|(k, v)| {
            let messages = v
                .into_iter()
                .filter(|f| f.message.is_some())
                .map(|f| f.clone().message.unwrap().to_string())
                .collect::<Vec<String>>()
                .join(", ");
            ValidationError {
                field: k.to_string(),
                message: messages,
            }
        })
        .collect::<Vec<ValidationError>>()
}
