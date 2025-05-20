use crate::convert::UnitConverter;
use crate::error::ConvertError;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct DistanceConverter;

static DISTANCE_FACTORS: Lazy<HashMap<(DistanceUnit, DistanceUnit), f64>> = Lazy::new(|| {
    let mut map = HashMap::new();

    let conversions = vec![
        (DistanceUnit::Kilometer, DistanceUnit::Meter, 1000.0),
        (DistanceUnit::Meter, DistanceUnit::Foot, 3.28084),
        (
            DistanceUnit::Foot,
            DistanceUnit::Kilometer,
            1000.0 * 3.28084,
        ),
    ];

    for (from, to, factor) in conversions {
        map.insert((from.clone(), to.clone()), factor);
        map.insert((to, from), 1.0 / factor);
    }
    map
});

impl UnitConverter for DistanceConverter {
    fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        let from_unit = DistanceUnit::from_str(from)?;
        let to_unit = DistanceUnit::from_str(to)?;

        DISTANCE_FACTORS
            .get(&(from_unit, to_unit))
            .map(|factor| value * factor)
            .ok_or(ConvertError::UnsupportedConversion(
                from.to_string(),
                to.to_string(),
            ))
    }

    fn supported_units(&self) -> Vec<String> {
        let mut units: Vec<String> = DistanceUnit::iter().map(|unit| unit.to_string()).collect();
        units.sort();
        units
    }
}

impl DistanceConverter {
    pub fn get_unit_string(&self, unit_str: &str) -> String {
        if let Ok(unit) = DistanceUnit::from_str(unit_str) {
            unit.to_string()
        } else {
            unit_str.to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum DistanceUnit {
    Meter,
    Kilometer,
    Foot,
}

impl FromStr for DistanceUnit {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "m" | "meter" | "metre" => Ok(DistanceUnit::Meter),
            "km" | "kilometer" | "kilometre" => Ok(DistanceUnit::Kilometer),
            "ft" | "foot" | "feet" => Ok(DistanceUnit::Foot),
            _ => Err(ConvertError::InvalidUnit(s.to_string())),
        }
    }
}

impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistanceUnit::Foot => write!(f, "ft"),
            DistanceUnit::Meter => write!(f, "m"),
            DistanceUnit::Kilometer => write!(f, "km"),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::{DistanceConverter, DistanceUnit};
    use crate::{
        convert::UnitConverter,
        error::ConvertError,
        test_utils::{assert_approx_eq, assert_convert_error},
    };

    #[test]
    fn test_m_to_ft() {
        let converter = DistanceConverter;
        let result = converter.convert(1.0, "m", "ft").unwrap();
        assert_approx_eq(result, 3.28084, 1e-5);
    }

    #[test]
    fn test_ft_to_m() {
        let converter = DistanceConverter;
        let result = converter.convert(3.28084, "ft", "m").unwrap();
        assert_approx_eq(result, 1.0, 1e-5);
    }

    #[test]
    fn test_invalid_unit() {
        assert_convert_error(DistanceConverter, "banana", "ft", 1.0, |e| {
            matches!(e, ConvertError::InvalidUnit(_))
        });
    }

    #[test]
    fn test_unit_parsing() {
        assert_eq!(DistanceUnit::from_str("m"), Ok(DistanceUnit::Meter));
        assert_eq!(DistanceUnit::from_str("ft"), Ok(DistanceUnit::Foot));
        let result = DistanceUnit::from_str("unknown");
        assert!(result.is_err(), "Expected error, got {:?}", result);
        let err = result.unwrap_err();
        assert!(matches!(err, ConvertError::InvalidUnit(_)));
    }
}
