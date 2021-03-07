use serde::{Serialize, Deserialize};
use std::fmt;
use crate::HasData;


/// State of charge in 1/100th of a percent
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Soc(Option<u16>);

impl fmt::Display for Soc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(soc) => write!(f, "{:.2}%", soc as f32 / 100.0),
            None => write!(f, "{}", crate::NO_DATA),
        }
    }
}


impl HasData for Soc {
    fn has_data(&self) -> bool {
        self.0.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Soc(None), Soc::default());
    }

    #[test]
    fn to_string() {
        let null = Soc(None);
        assert_eq!(crate::NO_DATA, null.to_string());

        let current = Soc(Some(123));
        assert_eq!("1.23%", current.to_string());

        let zero = Soc(Some(0));
        assert_eq!("0.00%", zero.to_string());
    }

    #[test]
    fn copy_equality_and_hash() {
        let current1 = Soc(Some(1234));
        let current2 = Soc(Some(2345));

        let mut set = std::collections::HashSet::new();
        set.insert(current1);

        assert!(set.contains(&current1));
        assert!(!set.contains(&current2));
    }

    #[test]
    fn has_data() {
        let some = Soc(Some(1234));
        let none = Soc(None);

        assert_eq!(true, some.has_data());
        assert_eq!(false, some.no_data());
        assert_eq!(false, none.has_data());
        assert_eq!(true, none.no_data());
    }
}