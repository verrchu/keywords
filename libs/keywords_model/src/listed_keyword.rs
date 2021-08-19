use std::borrow::Borrow;

use serde::Deserialize;

use crate::Keyword;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum ListedKeyword {
    Regular(Keyword),
}

impl AsRef<str> for ListedKeyword {
    fn as_ref(&self) -> &str {
        match self {
            Self::Regular(keyword) => keyword.as_ref(),
        }
    }
}

impl Borrow<Keyword> for ListedKeyword {
    fn borrow(&self) -> &Keyword {
        match self {
            Self::Regular(keyword) => keyword,
        }
    }
}
