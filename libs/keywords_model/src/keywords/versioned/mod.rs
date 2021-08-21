mod keyword;
use keyword::Keyword;

use crate::language;

use indexmap::{IndexMap, IndexSet};

pub type Keywords = IndexMap<language::Version, IndexSet<Keyword>>;
