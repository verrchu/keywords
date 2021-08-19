use std::borrow::Borrow;

use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum Keyword {
    Regular(String),
}

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        match self {
            Self::Regular(keyword) => keyword,
        }
    }
}

impl Borrow<str> for Keyword {
    fn borrow(&self) -> &str {
        match self {
            Self::Regular(keyword) => keyword,
        }
    }
}
