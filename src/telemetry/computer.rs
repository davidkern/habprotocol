use crate::value::Identity;
use serde::{Serialize, Deserialize};


/// Computer telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct ComputerTelemetry {
    /// Identity of the module
    pub id: Identity,

    /// Load average, if available
    pub load_average: LoadAverage,
}

/// Unix load average
#[derive(Serialize, Deserialize, Debug)]
pub struct LoadAverage(Option<u16>);

