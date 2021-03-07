use serde::{Serialize, Deserialize};
use std::fmt;
use crate::value::{Current, Identity, Voltage};
use crate::HasData;


/// Charger telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct ChargerTelemetry {
    pub id: Identity,
    pub input: ChargerInputTelemetry,
    pub output: ChargerOutputTelemetry,
    pub mode: ChargerMode,
}

/// Charger input telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct ChargerInputTelemetry {
    pub voltage: Voltage,
    pub current: Current,
}

/// Charger output telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct ChargerOutputTelemetry {
    pub voltage: Voltage,
    pub current: Current,
}

/// State of the charger
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ChargerMode {
    NoData,
    Off,
    Bulk,
    Absorption,
    Float,
    Error,
}

impl HasData for ChargerMode {
    fn has_data(&self) -> bool {
        self != &Self::NoData
    }
}

impl Default for ChargerMode {
    fn default() -> Self {
        Self::NoData
    }
}

impl fmt::Display for ChargerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoData => write!(f, "{}", crate::NO_DATA),
            Self::Off => write!(f, "off"),
            Self::Bulk => write!(f, "bulk"),
            Self::Absorption => write!(f, "absorption"),
            Self::Float => write!(f, "float"),
            Self::Error => write!(f, "error"),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn charger_mode_default() {
        assert_eq!(ChargerMode::NoData, ChargerMode::default());
    }

    #[test]
    fn charger_mode_to_string() {
        assert_eq!(crate::NO_DATA, ChargerMode::NoData.to_string());
        assert_eq!("off", ChargerMode::Off.to_string());
        assert_eq!("bulk", ChargerMode::Bulk.to_string());
        assert_eq!("absorption", ChargerMode::Absorption.to_string());
        assert_eq!("float", ChargerMode::Float.to_string());
        assert_eq!("error", ChargerMode::Error.to_string());
    }

    #[test]
    fn charger_mode_copy_equality_and_hash() {
        let mode1 = ChargerMode::Off;
        let mode2 = ChargerMode::Bulk;

        let mut set = std::collections::HashSet::new();
        set.insert(mode1);

        assert!(set.contains(&mode1));
        assert!(!set.contains(&mode2));
    }

    #[test]
    fn charger_mode_has_data() {
        let some = ChargerMode::Off;
        let none = ChargerMode::NoData;

        assert_eq!(true, some.has_data());
        assert_eq!(false, some.no_data());
        assert_eq!(false, none.has_data());
        assert_eq!(true, none.no_data());
    }
}