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
    Regular(String),
    Annotated(String, Annotation),
}

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        match self {
            Self::Regular(keyword) => keyword,
            Self::Annotated(keyword, _annotation) => keyword,
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Regular(keyword) => write!(f, "{}", keyword),
            Self::Annotated(keyword, annotation) => write!(f, "{} ({})", keyword, annotation),
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
            Keyword::Regular("test".into()).to_string()
        );

        assert_eq!(
            "test (unused)".to_string(),
            Keyword::Annotated("test".into(), Annotation::Unused).to_string()
        );
    }
}
