use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ConvertError {
    #[error("Invalid unit: '{0}'")]
    InvalidUnit(String),

    #[error("Conversion from '{0}' to '{1}' not supported")]
    UnsupportedConversion(String, String),

    #[error("Error parsing an expression")]
    ParseError,
}
