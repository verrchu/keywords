use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;

use crate::ListedKeyword;

use super::language;

#[derive(Debug, Clone, Deserialize)]
pub enum Keywords {
    Versioned(IndexMap<language::Version, IndexSet<ListedKeyword>>),
}
