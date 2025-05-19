use std::{fmt::Display, str::FromStr};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::error::ConvertError;

use super::UnitConverter;

pub struct TemperatureConverter;

impl UnitConverter for TemperatureConverter {
    fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        let from_unit = TemperatureUnit::from_str(from)?;
        let to_unit = TemperatureUnit::from_str(to)?;

        match (&from_unit, &to_unit) {
            (&TemperatureUnit::Celsius, &TemperatureUnit::Fahrenheit) => {
                Ok(Self::convert_celsius_fahrenheit(value, true))
            }
            (&TemperatureUnit::Fahrenheit, &TemperatureUnit::Celsius) => {
                Ok(Self::convert_celsius_fahrenheit(value, false))
            }
            (&TemperatureUnit::Celsius, &TemperatureUnit::Kelvin) => {
                Ok(Self::convert_celsius_kelvin(value, true))
            }
            (&TemperatureUnit::Kelvin, &TemperatureUnit::Celsius) => {
                Ok(Self::convert_celsius_kelvin(value, false))
            }
            (&TemperatureUnit::Fahrenheit, &TemperatureUnit::Kelvin) => {
                let celsius = Self::convert_celsius_fahrenheit(value, false);
                Ok(Self::convert_celsius_kelvin(celsius, true))
            }
            (&TemperatureUnit::Kelvin, &TemperatureUnit::Fahrenheit) => {
                let celsius = Self::convert_celsius_kelvin(value, false);
                Ok(Self::convert_celsius_fahrenheit(celsius, true))
            }
            _ => Err(ConvertError::UnsupportedConversion(
                from_unit.to_string(),
                to_unit.to_string(),
            )),
        }
    }

    fn supported_units(&self) -> Vec<String> {
        TemperatureUnit::iter()
            .map(|unit| unit.to_string())
            .collect()
    }
}

impl TemperatureConverter {
    fn convert_celsius_fahrenheit(value: f64, from_celsius: bool) -> f64 {
        if from_celsius {
            (value * 9.0 / 5.0) + 32.0
        } else {
            (value - 32.0) * 5.0 / 9.0
        }
    }

    fn convert_celsius_kelvin(value: f64, from_celsius: bool) -> f64 {
        if from_celsius {
            value + 273.15
        } else {
            value - 273.15
        }
    }

    pub fn get_unit_string(&self, unit_str: &str) -> String {
        if let Ok(unit) = TemperatureUnit::from_str(unit_str) {
            unit.to_string()
        } else {
            unit_str.to_string()
        }
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
