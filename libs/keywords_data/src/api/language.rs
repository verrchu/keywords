use std::collections::HashSet;
use std::sync::Arc;

use super::Error;
use crate::DATA;

use model::Language;

use itertools::Itertools;

pub fn list() -> Vec<Language> {
    Arc::clone(&*DATA)
        .keys()
        .map(ToOwned::to_owned)
        .map(Into::into)
        .collect()
}

pub fn check_supported<I, L>(languages: I) -> Result<(), Error>
where
    L: Into<Language>,
    I: IntoIterator<Item = L>,
{
    let provided_languages: HashSet<Language> = languages.into_iter().map(Into::into).collect();
    let supported_languages: HashSet<Language> = list().into_iter().collect();

    let diff: Vec<_> = provided_languages
        .difference(&supported_languages)
        .map(ToOwned::to_owned)
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
