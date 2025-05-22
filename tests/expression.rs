use unitconvert::error::ConvertError;
use unitconvert::expression::parse_expression;

#[test]
fn parses_valid_expression() {
    let result = parse_expression("10C -> F").unwrap();
    assert_eq!(result, (10.0, "C".to_string(), "F".to_string()));
}

#[test]
fn handles_invalid_format() {
    let result = parse_expression("10C to F");
    assert!(matches!(result, Err(ConvertError::ParseError)));
}

#[test]
fn handles_non_numeric_input() {
    let result = parse_expression("abcC -> F");
    assert!(matches!(result, Err(ConvertError::ParseError)));
}
