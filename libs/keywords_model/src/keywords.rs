use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Keywords {
    versions: IndexMap<String, Vec<String>>,
}
