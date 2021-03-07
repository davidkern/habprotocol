use crate::value::{Identity, Soc, Voltage};
use serde::{Serialize, Deserialize};


/// Battery bank telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryBankTelemetry {
    /// Identity of the module
    pub id: Identity,

    /// Battery bank voltage, if available
    pub voltage: Voltage,

    /// Battery bank state-of-charge, if available
    pub soc: Soc,
}

