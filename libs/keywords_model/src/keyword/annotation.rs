use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum Annotation {
    Unused,
}

impl Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unused => write!(f, "unused"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("unused".to_string(), Annotation::Unused.to_string());
    }
}
