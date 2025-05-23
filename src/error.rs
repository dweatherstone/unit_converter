use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("Invalid unit: '{0}'")]
    InvalidUnit(String),

    #[error("Conversion from '{0}' to '{1}' not supported")]
    UnsupportedConversion(String, String),

    #[error("Error parsing an expression: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl PartialEq for ConvertError {
    fn eq(&self, other: &Self) -> bool {
        use ConvertError::*;
        match (self, other) {
            (InvalidUnit(a), InvalidUnit(b)) => a == b,
            (UnsupportedConversion(a1, a2), UnsupportedConversion(b1, b2)) => a1 == b1 && a2 == b2,
            (ParseError(a), ParseError(b)) => a == b,
            (IoError(_), IoError(_)) => false,
            _ => false,
        }
    }
}
