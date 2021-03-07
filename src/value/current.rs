use serde::{Serialize, Deserialize};
use std::fmt;
use crate::HasData;

/// Current stored in mA
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Current(Option<u32>);

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(current) => write!(f, "{:.3}A", current as f32 / 1000.0),
            None => write!(f, "{}", crate::NO_DATA),
        }
    }
}

impl HasData for Current {
    fn has_data(&self) -> bool {
        self.0.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Current(None), Current::default());
    }

    #[test]
    fn to_string() {
        let null = Current(None);
        assert_eq!(crate::NO_DATA, null.to_string());

        let current = Current(Some(1234));
        assert_eq!("1.234A", current.to_string());

        let zero = Current(Some(0));
        assert_eq!("0.000A", zero.to_string());
    }

    #[test]
    fn copy_equality_and_hash() {
        let current1 = Current(Some(1234));
        let current2 = Current(Some(2345));

        let mut set = std::collections::HashSet::new();
        set.insert(current1);

        assert_eq!(true, set.contains(&current1));
        assert_eq!(false, set.contains(&current2));
    }

    #[test]
    fn has_data() {
        let some = Current(Some(1234));
        let none = Current(None);

        assert_eq!(true, some.has_data());
        assert_eq!(false, some.no_data());
        assert_eq!(false, none.has_data());
        assert_eq!(true, none.no_data());
    }
}