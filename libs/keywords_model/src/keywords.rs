use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Keywords {
    versions: IndexMap<String, IndexSet<String>>,
}

impl Keywords {
    pub fn versions(&self) -> impl IntoIterator<Item = (&String, &IndexSet<String>)> {
        &self.versions
    }
}
