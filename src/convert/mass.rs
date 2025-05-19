use std::{collections::HashMap, fmt::Display, str::FromStr};

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::error::ConvertError;

use super::UnitConverter;

pub struct MassConverter;

static MASS_FACTORS: Lazy<HashMap<(MassUnit, MassUnit), f64>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let conversions = vec![
        (MassUnit::Kilogram, MassUnit::Gram, 1000.0),
        (MassUnit::Kilogram, MassUnit::Pound, 2.20462),
        (MassUnit::Kilogram, MassUnit::Ounce, 35.274),
        (MassUnit::Stone, MassUnit::Kilogram, 6.35029),
        (MassUnit::Stone, MassUnit::Pound, 14.0),
        (MassUnit::Pound, MassUnit::Ounce, 16.0),
        (MassUnit::Pound, MassUnit::Gram, 453.59291),
        (MassUnit::Stone, MassUnit::Ounce, 224.0),
        (MassUnit::Stone, MassUnit::Gram, 6350.29497),
        (MassUnit::Ounce, MassUnit::Gram, 28.34949),
    ];

    for (from, to, factor) in conversions {
        map.insert((from.clone(), to.clone()), factor);
        map.insert((to, from), 1.0 / factor);
    }

    map
});

impl UnitConverter for MassConverter {
    fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, crate::error::ConvertError> {
        let from_unit = MassUnit::from_str(from)?;
        let to_unit = MassUnit::from_str(to)?;

        MASS_FACTORS
            .get(&(from_unit, to_unit))
            .map(|factor| value * factor)
            .ok_or(ConvertError::UnsupportedConversion(
                from.to_string(),
                to.to_string(),
            ))
    }

    fn supported_units(&self) -> Vec<String> {
        let mut units: Vec<String> = MassUnit::iter().map(|unit| unit.to_string()).collect();
        units.sort();
        units
    }
}

impl MassConverter {
    pub fn get_unit_string(&self, unit_str: &str) -> String {
        if let Ok(unit) = MassUnit::from_str(unit_str) {
            unit.to_string()
        } else {
            unit_str.to_string()
        }
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
