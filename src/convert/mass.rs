use std::{fmt::Display, str::FromStr};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::error::ConvertError;

use super::UnitConverter;

pub struct MassConverter;

#[derive(Debug, Clone, Copy)]
struct Gram(f64);

#[derive(Debug, Clone, Copy)]
struct Kilogram(f64);

#[derive(Debug, Clone, Copy)]
struct Ounce(f64);

#[derive(Debug, Clone, Copy)]
struct Pound(f64);

#[derive(Debug, Clone, Copy)]
struct Stone(f64);

impl MassConverter {
    pub fn get_unit_string(&self, unit_str: &str) -> String {
        if let Ok(unit) = MassUnit::from_str(unit_str) {
            unit.to_string()
        } else {
            unit_str.to_string()
        }
    }

    fn to_grams(value: f64, unit: &MassUnit) -> Gram {
        // Use grams as the base unit
        match unit {
            MassUnit::Gram => Gram(value),
            MassUnit::Kilogram => Kilogram(value).into(),
            MassUnit::Ounce => Ounce(value).into(),
            MassUnit::Pound => Pound(value).into(),
            MassUnit::Stone => Stone(value).into(),
        }
    }

    fn from_grams(grams: Gram, unit: &MassUnit) -> f64 {
        match unit {
            MassUnit::Gram => grams.0,
            MassUnit::Kilogram => Kilogram::from(grams).0,
            MassUnit::Ounce => Ounce::from(grams).0,
            MassUnit::Pound => Pound::from(grams).0,
            MassUnit::Stone => Stone::from(grams).0,
        }
    }
}

impl UnitConverter for MassConverter {
    fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        let from_unit = MassUnit::from_str(from)?;
        let to_unit = MassUnit::from_str(to)?;

        let grams = Self::to_grams(value, &from_unit);
        Ok(Self::from_grams(grams, &to_unit))
    }

    fn supported_units(&self) -> Vec<String> {
        let mut units: Vec<String> = MassUnit::iter().map(|unit| unit.to_string()).collect();
        units.sort();
        units
    }
}

// Convert to Gram
impl From<Kilogram> for Gram {
    fn from(value: Kilogram) -> Self {
        Gram(value.0 * 1000.0)
    }
}

impl From<Pound> for Gram {
    fn from(value: Pound) -> Self {
        Gram(value.0 * 453.59291)
    }
}

impl From<Ounce> for Gram {
    fn from(value: Ounce) -> Self {
        Gram(value.0 * 28.34949)
    }
}

impl From<Stone> for Gram {
    fn from(value: Stone) -> Self {
        Gram(value.0 * 6350.29497)
    }
}

// Convert from Gram
impl From<Gram> for Kilogram {
    fn from(value: Gram) -> Self {
        Kilogram(value.0 / 1000.0)
    }
}

impl From<Gram> for Pound {
    fn from(value: Gram) -> Self {
        Pound(value.0 / 453.59291)
    }
}

impl From<Gram> for Ounce {
    fn from(value: Gram) -> Self {
        Ounce(value.0 / 28.34949)
    }
}

impl From<Gram> for Stone {
    fn from(value: Gram) -> Self {
        Stone(value.0 / 6350.29497)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum MassUnit {
    Kilogram,
    Pound,
    Stone,
    Ounce,
    Gram,
}

impl FromStr for MassUnit {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kg" | "kilogram" | "kilograms" => Ok(MassUnit::Kilogram),
            "lb" | "pound" | "pounds" => Ok(MassUnit::Pound),
            "st" | "stone" | "stones" => Ok(MassUnit::Stone),
            "oz" | "ounce" | "ounces" => Ok(MassUnit::Ounce),
            "g" | "gram" | "grams" => Ok(MassUnit::Gram),
            _ => Err(ConvertError::InvalidUnit(s.to_string())),
        }
    }
}

impl Display for MassUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MassUnit::Kilogram => write!(f, "kg"),
            MassUnit::Pound => write!(f, "lb"),
            MassUnit::Stone => write!(f, "st"),
            MassUnit::Ounce => write!(f, "oz"),
            MassUnit::Gram => write!(f, "g"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert::UnitConverter, test_utils::assert_approx_eq};

    use super::MassConverter;

    #[test]
    fn test_kg_to_lb() {
        let converter = MassConverter;
        let result = converter.convert(2.5, "kg", "lb").unwrap();
        assert_approx_eq(5.51155, result, 1e-4);
    }
}
