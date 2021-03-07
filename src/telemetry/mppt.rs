use crate::value::{Current, Identity, Voltage};
use serde::{Serialize, Deserialize};


/// Mppt telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct MpptTelemetry {
    pub id: Identity,
    pub input_voltage: MpptInputTelemetry,
    pub output_voltage: MpptOutputTelemetry,
    pub mode: MpptMode,
}

/// Charger input telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct MpptInputTelemetry {
    pub voltage: Voltage,
    pub current: Current,
}

/// Charger output telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct MpptOutputTelemetry {
    pub voltage: Voltage,
    pub current: Current,
}

/// State of the mppt
#[derive(Serialize, Deserialize, Debug)]
pub enum MpptMode {
    NoData,
    Off,
    Bulk,
    Absorption,
    Float,
    Error,
}
