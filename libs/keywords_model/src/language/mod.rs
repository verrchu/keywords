mod version;
pub use version::Version;

use std::borrow::Borrow;
use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Language(String);

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Language {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<S: Into<String>> From<S> for Language {
    fn from(input: S) -> Self {
        Self(input.into())
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("test".to_string(), Language("test".into()).to_string());
    }
}
