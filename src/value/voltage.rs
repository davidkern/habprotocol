use serde::{Serialize, Deserialize};
use std::fmt;
use crate::HasData;


/// Voltage in mV
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Voltage(Option<u32>);

impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(voltage) => write!(f, "{:.3}V", voltage as f32 / 1000.0),
            None => write!(f, "{}", crate::NO_DATA),
        }
    }
}

impl HasData for Voltage {
    fn has_data(&self) -> bool {
        self.0.is_some()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Voltage(None), Voltage::default());
    }

    #[test]
    fn to_string() {
        let null = Voltage(None);
        assert_eq!(crate::NO_DATA, null.to_string());

        let current = Voltage(Some(1234));
        assert_eq!("1.234V", current.to_string());

        let zero = Voltage(Some(0));
        assert_eq!("0.000V", zero.to_string());
    }

    #[test]
    fn copy_equality_and_hash() {
        let current1 = Voltage(Some(1234));
        let current2 = Voltage(Some(2345));

        let mut set = std::collections::HashSet::new();
        set.insert(current1);

        assert!(set.contains(&current1));
        assert!(!set.contains(&current2));
    }

    #[test]
    fn has_data() {
        let some = Voltage(Some(1234));
        let none = Voltage(None);

        assert_eq!(true, some.has_data());
        assert_eq!(false, some.no_data());
        assert_eq!(false, none.has_data());
        assert_eq!(true, none.no_data());
    }
}