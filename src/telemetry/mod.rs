use chrono::{DateTime, Utc};

pub mod fault;
pub mod computer;
pub mod battery;
pub mod battery_bank;
pub mod inverter;
pub mod charger;
pub mod mppt;

/// A moment-in-time snapshot of telemetry data.
pub struct Telemetry {
    /// The system time in UTC
    pub time: DateTime<Utc>,

    /// Fault telemetry
    pub fault: Vec<fault::FaultTelemetry>,

    /// Computer telemetry
    pub computer: Vec<computer::ComputerTelemetry>,

    /// Battery telemetry
    pub battery: Vec<battery::BatteryTelemetry>,

    /// Battery bank telemetry
    pub battery_bank: Vec<battery_bank::BatteryBankTelemetry>,

    /// Inverter telemetry
    pub inverter: Vec<inverter::InverterTelemetry>,

    /// Charger telemetry
    pub charger: Vec<charger::ChargerTelemetry>,

    /// Mppt telemetry
    pub mppt: Vec<mppt::MpptTelemetry>,
}
