use crate::convert::UnitConverter;
use crate::error::ConvertError;
use std::{fmt, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct DistanceConverter;

#[derive(Debug, Clone, Copy)]
struct Meter(f64);
#[derive(Debug, Clone, Copy)]
struct Kilometer(f64);
#[derive(Debug, Clone, Copy)]
struct Foot(f64);
#[derive(Debug, Clone, Copy)]
struct Mile(f64);
#[derive(Debug, Clone, Copy)]
struct Inch(f64);
#[derive(Debug, Clone, Copy)]
struct Centimeter(f64);
#[derive(Debug, Clone, Copy)]
struct Millimeter(f64);

impl DistanceConverter {
    pub fn get_unit_string(&self, unit_str: &str) -> String {
        if let Ok(unit) = DistanceUnit::from_str(unit_str) {
            unit.to_string()
        } else {
            unit_str.to_string()
        }
    }

    fn to_meter(value: f64, unit: &DistanceUnit) -> Meter {
        match unit {
            DistanceUnit::Meter => Meter(value),
            DistanceUnit::Kilometer => Kilometer(value).into(),
            DistanceUnit::Foot => Foot(value).into(),
            DistanceUnit::Mile => Mile(value).into(),
            DistanceUnit::Inch => Inch(value).into(),
            DistanceUnit::Centimeter => Centimeter(value).into(),
            DistanceUnit::Millimeter => Millimeter(value).into(),
        }
    }

    fn from_meter(meters: Meter, unit: &DistanceUnit) -> f64 {
        match unit {
            DistanceUnit::Meter => meters.0,
            DistanceUnit::Kilometer => Kilometer::from(meters).0,
            DistanceUnit::Foot => Foot::from(meters).0,
            DistanceUnit::Mile => Mile::from(meters).0,
            DistanceUnit::Inch => Inch::from(meters).0,
            DistanceUnit::Centimeter => Centimeter::from(meters).0,
            DistanceUnit::Millimeter => Millimeter::from(meters).0,
        }
    }
}

impl UnitConverter for DistanceConverter {
    fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        let from_unit = DistanceUnit::from_str(from)?;
        let to_unit = DistanceUnit::from_str(to)?;

        let meters = Self::to_meter(value, &from_unit);
        Ok(Self::from_meter(meters, &to_unit))
    }

    fn supported_units(&self) -> Vec<String> {
        let mut units: Vec<String> = DistanceUnit::iter().map(|unit| unit.to_string()).collect();
        units.sort();
        units
    }
}

// Convert to Meter
impl From<Kilometer> for Meter {
    fn from(value: Kilometer) -> Self {
        Meter(value.0 * 1000.0)
    }
}

impl From<Foot> for Meter {
    fn from(value: Foot) -> Self {
        Meter(value.0 * 0.3048)
    }
}

impl From<Mile> for Meter {
    fn from(value: Mile) -> Self {
        Meter(value.0 * 1609.34)
    }
}

impl From<Inch> for Meter {
    fn from(value: Inch) -> Self {
        Meter(value.0 * 0.0254)
    }
}

impl From<Centimeter> for Meter {
    fn from(value: Centimeter) -> Self {
        Meter(value.0 * 0.01)
    }
}

impl From<Millimeter> for Meter {
    fn from(value: Millimeter) -> Self {
        Meter(value.0 * 0.001)
    }
}

// Convert from Meter
impl From<Meter> for Kilometer {
    fn from(value: Meter) -> Self {
        Kilometer(value.0 / 1000.0)
    }
}

impl From<Meter> for Foot {
    fn from(value: Meter) -> Self {
        Foot(value.0 / 0.3048)
    }
}

impl From<Meter> for Mile {
    fn from(value: Meter) -> Self {
        Mile(value.0 / 1609.34)
    }
}

impl From<Meter> for Inch {
    fn from(value: Meter) -> Self {
        Inch(value.0 / 0.0254)
    }
}

impl From<Meter> for Centimeter {
    fn from(value: Meter) -> Self {
        Centimeter(value.0 / 0.01)
    }
}

impl From<Meter> for Millimeter {
    fn from(value: Meter) -> Self {
        Millimeter(value.0 / 0.001)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum DistanceUnit {
    Meter,
    Kilometer,
    Foot,
    Mile,
    Inch,
    Centimeter,
    Millimeter,
}

impl FromStr for DistanceUnit {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "m" | "meter" | "metre" | "meters" | "metres" => Ok(DistanceUnit::Meter),
            "km" | "kilometer" | "kilometre" | "kilometers" | "kilometres" => {
                Ok(DistanceUnit::Kilometer)
            }
            "ft" | "foot" | "feet" => Ok(DistanceUnit::Foot),
            "mi" | "mile" | "miles" => Ok(DistanceUnit::Mile),
            "in" | "inch" | "inches" => Ok(DistanceUnit::Inch),
            "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => {
                Ok(DistanceUnit::Centimeter)
            }
            "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => {
                Ok(DistanceUnit::Millimeter)
            }
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
            DistanceUnit::Mile => write!(f, "mi"),
            DistanceUnit::Inch => write!(f, "in"),
            DistanceUnit::Centimeter => write!(f, "cm"),
            DistanceUnit::Millimeter => write!(f, "mm"),
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
