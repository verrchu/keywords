mod version;
pub use version::Version;

use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Language(String);

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<S: Into<String>> From<S> for Language {
    fn from(input: S) -> Self {
        Self(input.into())
    }
}
