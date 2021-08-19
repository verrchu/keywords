use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;

use super::{Keyword, Version};

#[derive(Debug, Clone, Deserialize)]
pub enum Keywords {
    Versioned(IndexMap<Version, IndexSet<Keyword>>),
}
