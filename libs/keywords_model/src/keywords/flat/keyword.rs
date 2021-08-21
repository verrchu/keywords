use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Keyword(String, Annotation);

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum Annotation {
    Unused,
    Regular,
}
