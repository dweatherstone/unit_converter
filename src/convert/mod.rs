use std::str::FromStr;

use distance::{DisctanceConverter, DistanceUnit};
use mass::{MassConverter, MassUnit};
use temperature::{TemperatureConverter, TemperatureUnit};

use crate::error::ConvertError;

pub mod distance;
pub mod mass;
pub mod temperature;

pub trait UnitConverter {
    fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError>;

    fn supported_units(&self) -> Vec<String>;
}

pub fn get_converter(from: &str, to: &str) -> Result<AnyConverter, ConvertError> {
    if DistanceUnit::from_str(from).is_ok() && DistanceUnit::from_str(to).is_ok() {
        Ok(AnyConverter::Distance(DisctanceConverter))
    } else if MassUnit::from_str(from).is_ok() && MassUnit::from_str(to).is_ok() {
        Ok(AnyConverter::Mass(MassConverter))
    } else if TemperatureUnit::from_str(from).is_ok() && TemperatureUnit::from_str(to).is_ok() {
        Ok(AnyConverter::Temperature(TemperatureConverter))
    } else {
        Err(ConvertError::UnsupportedConversion(
            from.to_string(),
            to.to_string(),
        ))
    }
}

pub enum AnyConverter {
    Distance(DisctanceConverter),
    Mass(MassConverter),
    Temperature(TemperatureConverter),
}

impl AnyConverter {
    pub fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        match self {
            AnyConverter::Distance(c) => c.convert(value, from, to),
            AnyConverter::Mass(c) => c.convert(value, from, to),
            AnyConverter::Temperature(c) => c.convert(value, from, to),
        }
    }

    pub fn supported_units(&self) -> Vec<String> {
        match self {
            AnyConverter::Distance(c) => c.supported_units(),
            AnyConverter::Mass(c) => c.supported_units(),
            AnyConverter::Temperature(c) => c.supported_units(),
        }
    }

    pub fn get_unit_string(&self, unit_str: &str) -> String {
        match self {
            AnyConverter::Distance(c) => c.get_unit_string(unit_str),
            AnyConverter::Mass(c) => c.get_unit_string(unit_str),
            AnyConverter::Temperature(c) => c.get_unit_string(unit_str),
        }
    }
}
