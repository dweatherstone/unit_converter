use unitconvert::error::ConvertError;
use unitconvert::expression::parse_expression;

#[test]
fn parses_valid_expression() {
    let result = parse_expression("10C -> F").unwrap();
    assert_eq!(result.value, 10.0);
    assert_eq!(result.from, "c".to_string());
    assert_eq!(result.to, "f".to_string());
}

#[test]
fn handles_invalid_format() {
    let result = parse_expression("10C into F");
    assert!(result.is_err(), "Expected error, got {:?}", result);
    let err = result.unwrap_err();
    assert!(matches!(err, ConvertError::ParseError(_)));
}

#[test]
fn handles_non_numeric_input() {
    let result = parse_expression("abcC -> F");
    assert!(result.is_err(), "Expected error, got {:?}", result);
    let err = result.unwrap_err();
    assert!(matches!(err, ConvertError::ParseError(_)));
}
