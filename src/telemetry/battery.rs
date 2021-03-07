use crate::value::{Identity, Soc, Voltage};
use serde::{Serialize, Deserialize};


/// Battery telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryTelemetry {
    /// Identity of the module
    pub id: Identity,

    /// Battery voltage
    pub voltage: Voltage,

    /// Battery state-of-charge
    pub soc: Soc,
}
