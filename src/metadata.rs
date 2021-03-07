use crate::value::Identity;
use serde::{Serialize, Deserialize};


/// Metadata regarding the system and structure of the telemetry
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    /// Metadata related to the overall system
    pub system: SystemMetadata,

    /// Metadata for each module in the system.
    pub module: Vec<ModuleMetadata>,
}

/// Metadata related to the overall system
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemMetadata {
    /// Name of the system
    pub name: String,
}

/// Metadata related to a specific module
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleMetadata {
    /// Name of the module
    pub name: String,

    /// Module identity
    pub identity: Identity,

    /// Module type
    pub kind: ModuleKind,

    /// Nominal interval between telemetry updates, in milliseconds
    pub interval: u32,
}

/// Type of module
#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleKind {
    /// An unknown, unsupported module
    Unknown,

    /// A computer or controller
    Computer,

    /// An individual battery
    Battery,

    /// A bank of batteries
    BatteryBank,

    /// A device which produces AC power from a DC source
    Inverter,

    /// A device capable of charging batteries from an AC source
    Charger,

    /// A device capable of charging batteries from a PV or Wind source
    Mppt,
}
