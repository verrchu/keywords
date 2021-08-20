use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Version(String);

impl<S: Into<String>> From<S> for Version {
    fn from(input: S) -> Self {
        Self(input.into())
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("test".to_string(), Version("test".into()).to_string());
    }
}
