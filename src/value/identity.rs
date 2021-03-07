use serde::{Serialize, Deserialize};
use std::fmt;


/// Stable identity of a component in the system
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Identity(u64);

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "№{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_string() {
        let id = Identity(1234);
        assert_eq!("№1234", id.to_string());
    }

    #[test]
    fn copy_equality_and_hash() {
        let id1 = Identity(1234);
        let id2 = Identity(2345);

        let mut set = std::collections::HashSet::new();
        set.insert(id1);

        assert!(set.contains(&id1));
        assert!(!set.contains(&id2));
    }
}