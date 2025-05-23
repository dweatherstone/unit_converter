use strsim::levenshtein;

use crate::{
    convert::{distance::DistanceUnit, mass::MassUnit, temperature::TemperatureUnit},
    error::ConvertError,
};

/// Represents a parsed conversion expression.
#[derive(Debug)]
pub struct ParsedExpression {
    pub value: f64,
    pub from: String,
    pub to: String,
}

pub fn parse_expression(expr: &str) -> Result<ParsedExpression, ConvertError> {
    // Normalize `to` -> `->`
    let normalized = expr
        .replace(" to ", " -> ")
        .replace("TO", "->")
        .replace("To", "->");
    let parts: Vec<&str> = normalized.split("->").map(str::trim).collect();

    if parts.len() != 2 {
        return Err(ConvertError::ParseError(
            "Invalid expression. Use format like: 10C -> F".to_string(),
        ));
    }

    // Extract value+from and to
    let (left, right) = (parts[0], parts[1]);

    let (value, from_unit) = parse_value_and_unit(left)?;
    let to_unit = right.to_string();
    let from_unit = from_unit.to_lowercase();
    let to_unit = to_unit.to_lowercase();

    // Try converting units (this is where fuzziness can help)...
    let valid_units = get_all_unit_strings();

    let from_suggestion = if valid_units.contains(&from_unit.as_str()) {
        None
    } else {
        suggest_unit(&from_unit, &valid_units)
    };
    let to_suggestion = if valid_units.contains(&to_unit.as_str()) {
        None
    } else {
        suggest_unit(&to_unit, &valid_units)
    };

    if from_suggestion.is_some() || to_suggestion.is_some() {
        let mut msg = String::new();
        if let Some(s) = from_suggestion {
            msg += &format!("Unknown unit '{}'. Did you mean '{}'?\n", from_unit, s);
        }
        if let Some(s) = to_suggestion {
            msg += &format!("Unknown unit '{}'. Did you mean '{}'?\n", to_unit, s);
        }
        return Err(ConvertError::ParseError(msg.trim_end().to_string()));
    }
    Ok(ParsedExpression {
        value,
        from: from_unit,
        to: to_unit,
    })
}

fn parse_value_and_unit(input: &str) -> Result<(f64, String), ConvertError> {
    let (value_str, unit_str) =
        input
            .trim()
            .split_at(
                input
                    .find(|c: char| c.is_alphabetic())
                    .ok_or(ConvertError::ParseError(
                        "Missing unit in expression".to_string(),
                    ))?,
            );

    let value: f64 = value_str
        .trim()
        .parse()
        .map_err(|_| ConvertError::ParseError("Invalid number".to_string()))?;
    Ok((value, unit_str.trim().to_string()))
}

fn get_all_unit_strings() -> Vec<&'static str> {
    let mut units = vec![];
    units.extend(DistanceUnit::accepted_string());
    units.extend(MassUnit::accepted_string());
    units.extend(TemperatureUnit::accepted_string());
    units
}

fn suggest_unit(input: &str, valid_units: &[&str]) -> Option<String> {
    let threshold = 2;
    valid_units
        .iter()
        .min_by_key(|unit| levenshtein(&input.to_lowercase(), &unit.to_lowercase()))
        .filter(|unit| levenshtein(&input.to_lowercase(), &unit.to_lowercase()) <= threshold)
        .map(|unit| unit.to_string())
}

#[cfg(test)]
mod tests {
    use crate::error::ConvertError;

    use super::parse_expression;

    #[test]
    fn test_parse_expression_valid() {
        let expr = "10C -> F";
        let parsed = parse_expression(expr).unwrap();
        assert_eq!(parsed.value, 10.0);
        assert_eq!(parsed.from, "c".to_string());
        assert_eq!(parsed.to, "f".to_string());
    }

    #[test]
    fn test_parse_expression_invalid_format() {
        let result = parse_expression("10C into F");
        assert!(result.is_err(), "Expected error, got: {:?}", result);
        let err = result.unwrap_err();
        assert!(matches!(err, ConvertError::ParseError(_)));
    }

    #[test]
    fn test_parse_expression_non_numberic_value() {
        let result = parse_expression("abcC -> F");
        assert!(result.is_err(), "Expected error, got: {:?}", result);
        let err = result.unwrap_err();
        assert!(matches!(err, ConvertError::ParseError(_)));
    }
}
