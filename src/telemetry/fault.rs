use crate::value::Identity;
use serde::{Serialize, Deserialize};


/// A fault condition
#[derive(Serialize, Deserialize, Debug)]
pub struct FaultTelemetry {
    /// Identity of the module
    pub id: Identity,

    /// Description of the fault
    pub description: String,

    /// Severity of the fault
    pub severity: FaultSeverity,
}

/// Severity of the fault
#[derive(Serialize, Deserialize, Debug)]
pub enum FaultSeverity {
    /// Unimportant, for troubleshooting purposes
    Informational,

    /// Should be displayed but no damage or unsafe condition is present
    Warning,

    /// A condition in which damage could occur but safety is not
    /// impared. Should be displayed and alerted via audio and
    /// visual means.
    /// 
    /// The corresponding system should be disabled automatically
    /// if possible.
    Critical,

    /// An unsafe condition exists which compromizes safety and
    /// is likely to lead to injury or severe damage. Should be
    /// displayed and occupants alerted to evacuate immediately.
    /// 
    /// All systems not critical to evacuation of occupants should be
    /// immediately disabled if possible.
    Emergency,
}
