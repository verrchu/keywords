use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum Keyword {
    Introduced(String, Annotation),
    Removed(String),
}

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        match self {
            Self::Removed(keyword) => keyword,
            Self::Introduced(keyword, _annotation) => keyword,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum Annotation {
    Unused,
    Regular,
}

impl Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unused => write!(f, "unused"),
            Self::Regular => write!(f, "regular"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("unused".to_string(), Annotation::Unused.to_string());
        assert_eq!("regular".to_string(), Annotation::Regular.to_string());
    }
}
