use std::{fmt::Display, str::FromStr};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::error::ConvertError;

use super::UnitConverter;

pub struct TemperatureConverter;

#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

#[derive(Debug, Clone, Copy)]
struct Kelvin(f64);

impl UnitConverter for TemperatureConverter {
    fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        let from_unit = TemperatureUnit::from_str(from)?;
        let to_unit = TemperatureUnit::from_str(to)?;

        let celsius = Self::to_celsius(value, &from_unit);
        Ok(Self::from_celsius(celsius, &to_unit))
    }

    fn supported_units(&self) -> Vec<String> {
        TemperatureUnit::iter()
            .map(|unit| unit.to_string())
            .collect()
    }
}

impl TemperatureConverter {
    pub fn get_unit_string(&self, unit_str: &str) -> String {
        if let Ok(unit) = TemperatureUnit::from_str(unit_str) {
            unit.to_string()
        } else {
            unit_str.to_string()
        }
    }

    fn to_celsius(value: f64, unit: &TemperatureUnit) -> Celsius {
        match unit {
            TemperatureUnit::Celsius => Celsius(value),
            TemperatureUnit::Fahrenheit => Fahrenheit(value).into(),
            TemperatureUnit::Kelvin => Kelvin(value).into(),
        }
    }

    fn from_celsius(celsius: Celsius, unit: &TemperatureUnit) -> f64 {
        match unit {
            TemperatureUnit::Celsius => celsius.0,
            TemperatureUnit::Fahrenheit => Fahrenheit::from(celsius).0,
            TemperatureUnit::Kelvin => Kelvin::from(celsius).0,
        }
    }
}

// Convert to Celsius
impl From<Fahrenheit> for Celsius {
    fn from(value: Fahrenheit) -> Self {
        Celsius((value.0 - 32.0) * 5.0 / 9.0)
    }
}

impl From<Kelvin> for Celsius {
    fn from(value: Kelvin) -> Self {
        Celsius(value.0 - 273.15)
    }
}

// Convert from Celsius
impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        Fahrenheit((value.0 * 9.0 / 5.0) + 32.0)
    }
}

impl From<Celsius> for Kelvin {
    fn from(value: Celsius) -> Self {
        Kelvin(value.0 + 273.15)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl FromStr for TemperatureUnit {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" | "celsius" => Ok(TemperatureUnit::Celsius),
            "f" | "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            "k" | "kelvin" => Ok(TemperatureUnit::Kelvin),
            _ => Err(ConvertError::InvalidUnit(s.to_string())),
        }
    }
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureUnit::Celsius => write!(f, "°C"),
            TemperatureUnit::Fahrenheit => write!(f, "°F"),
            TemperatureUnit::Kelvin => write!(f, "°K"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert::UnitConverter, test_utils::assert_approx_eq};

    use super::TemperatureConverter;

    #[test]
    fn basic_temperature_conversions() {
        let tests = [
            (0.0, "c", "f", 32.0),
            (0.0, "f", "c", -17.7778),
            (0.0, "c", "k", 273.15),
        ];
        let converter = TemperatureConverter;
        for (value, from, to, expected) in tests {
            let result = converter.convert(value, from, to).unwrap();
            assert_approx_eq(result, expected, 1e-4);
        }
    }
}
