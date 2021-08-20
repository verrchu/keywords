mod listed;
pub use listed::ListedKeyword;

mod occurence;
pub use occurence::Occurence;

mod annotation;
pub use annotation::Annotation;

use crate::language;

use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Keyword(String);

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<S: ToString> From<S> for Keyword {
    fn from(input: S) -> Self {
        Self(input.to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Keywords {
    Versioned(IndexMap<language::Version, IndexSet<ListedKeyword>>),
    Flat(IndexSet<ListedKeyword>),
}
