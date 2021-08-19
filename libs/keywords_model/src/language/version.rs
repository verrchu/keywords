use serde::Deserialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Version(String);

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
