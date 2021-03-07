pub mod metadata;
pub mod telemetry;
pub mod value;

/// Display string for a value with data not present
pub const NO_DATA: &'static str = "no-data";

/// Predicates for has-data and no-data
pub trait HasData {
    fn has_data(&self) -> bool;

    fn no_data(&self) -> bool {
        !self.has_data()
    }
}
