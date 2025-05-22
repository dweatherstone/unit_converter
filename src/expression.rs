use regex::Regex;

use crate::error::ConvertError;

pub fn parse_expression(expr: &str) -> Result<(f64, String, String), ConvertError> {
    let re = Regex::new(r"(?i)^\s*([0-9]*\.?[0-9]+)\s*([a-zA-Z]+)\s*->\s*([a-zA-Z]+)\s*$")
        .map_err(|_| ConvertError::ParseError)?;

    if let Some(caps) = re.captures(expr) {
        let value: f64 = caps[1].parse().map_err(|_| ConvertError::ParseError)?;
        let from = caps[2].trim().to_string();
        let to = caps[3].trim().to_string();

        Ok((value, from, to))
    } else {
        Err(ConvertError::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ConvertError;

    use super::parse_expression;

    #[test]
    fn test_parse_expression_valid() {
        let expr = "10C -> F";
        let parsed = parse_expression(expr).unwrap();
        assert_eq!(parsed, (10.0, "C".to_string(), "F".to_string()));
    }

    #[test]
    fn test_parse_expression_invalid_format() {
        let result = parse_expression("10C to F");
        assert!(matches!(result, Err(ConvertError::ParseError)));
    }

    #[test]
    fn test_parse_expression_non_numberic_value() {
        let result = parse_expression("abcC -> F");
        assert!(matches!(result, Err(ConvertError::ParseError)));
    }
}
