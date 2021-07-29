use std::sync::Arc;

use crate::DATA;

use itertools::Itertools;

pub fn list() -> Vec<String> {
    Arc::clone(&*DATA)
        .keys()
        .map(ToString::to_string)
        .sorted()
        .collect()
}
