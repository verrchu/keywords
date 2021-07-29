use std::collections::HashSet;
use std::sync::Arc;

use super::Error;
use crate::DATA;

use itertools::Itertools;

pub fn list() -> Vec<String> {
    Arc::clone(&*DATA)
        .keys()
        .map(ToString::to_string)
        .sorted()
        .collect()
}

pub fn check_supported(languages: &Vec<String>) -> Result<(), Error> {
    let provided_languages: HashSet<_> = languages.iter().map(ToString::to_string).collect();
    let supported_languages: HashSet<_> =
        Arc::clone(&*DATA).keys().map(ToString::to_string).collect();

    let diff: Vec<_> = provided_languages
        .difference(&supported_languages)
        .map(ToString::to_string)
        .collect();

    if !diff.is_empty() {
        Err(Error::UnsupportedLanguages {
            unsupported: diff.into_iter().sorted().collect(),
            supported: supported_languages.into_iter().sorted().collect(),
        })
    } else {
        Ok(())
    }
}
