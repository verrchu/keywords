use std::sync::Arc;

use super::Error;
use crate::DATA;

use model::Occurence;

pub fn search(word: &str, languages: Option<Vec<String>>) -> Result<Vec<Occurence>, Error> {
    languages
        .as_ref()
        .map(super::languages::check_supported)
        .transpose()?;

    let data = Arc::clone(&*DATA);

    let languages =
        languages.unwrap_or_else(|| data.keys().map(ToString::to_string).collect::<Vec<_>>());

    let mut occurences = vec![];
    for language in languages.iter() {
        if let Some(keywords) = data.get(language.as_str()) {
            for (version, keywords) in keywords.versions() {
                if keywords.contains(word) {
                    occurences.push(
                        Occurence::builder()
                            .language(language)
                            .since(version)
                            .word(word)
                            .build(),
                    );
                }
            }
        }
    }

    Ok(occurences)
}
