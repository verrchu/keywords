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
