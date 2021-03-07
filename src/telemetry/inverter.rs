use crate::value::{Current, Identity, Voltage};
use serde::{Serialize, Deserialize};


/// Inverter telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct InverterTelemetry {
    pub id: Identity,
    pub input: InverterInputTelemetry,
    pub output: InverterOutputTelemetry,
}

/// Inverter input telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct InverterInputTelemetry {
    pub voltage: Voltage,
    pub current: Current,
}

/// Inverter output telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct InverterOutputTelemetry {
    pub voltage: Voltage,
    pub current: Current,
}

/// State of the inverter
#[derive(Serialize, Deserialize, Debug)]
pub enum InverterMode {
    NoData,
    Off,
    On,
    Error,
}
