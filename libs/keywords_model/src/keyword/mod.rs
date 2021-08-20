mod occurence;
pub use occurence::Occurence;

mod annotation;
pub use annotation::Annotation;

use std::fmt::{self, Display};

use crate::language;

use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Keywords {
    Versioned(IndexMap<language::Version, IndexSet<Keyword>>),
    Flat(IndexSet<Keyword>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum Keyword {
    Introduced(String, Annotation),
    Removed(String),
}

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        match self {
            Self::Introduced(keyword, _annotation) => keyword,
            Self::Removed(keyword) => keyword,
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Introduced(keyword, annotation) => write!(f, "{} ({})", keyword, annotation),
            Self::Removed(keyword) => write!(f, "{}", keyword),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            "test".to_string(),
            Keyword::Introduced("test".into(), Annotation::Regular).to_string()
        );

        assert_eq!(
            "test (unused)".to_string(),
            Keyword::Removed("test".into()).to_string()
        );
    }
}
