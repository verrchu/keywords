use serde::Deserialize;

use crate::{keyword, Keyword};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum ListedKeyword {
    Regular(Keyword),
    Annotated(Keyword, keyword::Annotation),
}

impl AsRef<str> for ListedKeyword {
    fn as_ref(&self) -> &str {
        match self {
            Self::Regular(keyword) => keyword.as_ref(),
            Self::Annotated(keyword, _annotation) => keyword.as_ref(),
        }
    }
}

impl AsRef<Keyword> for ListedKeyword {
    fn as_ref(&self) -> &Keyword {
        match self {
            Self::Regular(keyword) => keyword,
            Self::Annotated(keyword, _annotation) => keyword,
        }
    }
}
