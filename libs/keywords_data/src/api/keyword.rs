use std::sync::Arc;

use super::Error;
use crate::DATA;

use model::{keyword, Keywords, Language};

pub fn search<I, L>(keyword: &str, languages: Option<I>) -> Result<Vec<keyword::Occurence>, Error>
where
    L: Into<Language>,
    I: IntoIterator<Item = L> + Clone,
{
    languages
        .clone()
        .map(super::language::check_supported)
        .transpose()?;

    let data = Arc::clone(&*DATA);

    let languages = languages.map(|languages| languages.into_iter().map(Into::into).collect());
    let languages = languages.unwrap_or_else(super::language::list);

    let mut occurences = vec![];
    for language in languages.iter() {
        if let Some(keywords) = data.get(language.as_ref()) {
            match keywords {
                Keywords::Versioned(keywords) => {
                    for (version, keywords) in keywords.iter() {
                        let keyword = keywords.iter().find(|item| item.as_ref() == keyword);

                        if let Some(keyword) = keyword {
                            occurences.push(
                                keyword::Occurence::builder()
                                    .language(language.to_owned())
                                    .since(version.to_owned())
                                    .keyword(keyword.to_owned())
                                    .build(),
                            );
                        }
                    }
                }
                Keywords::Flat(keywords) => {
                    let keyword = keywords.iter().find(|item| item.as_ref() == keyword);

                    if let Some(keyword) = keyword {
                        occurences.push(
                            keyword::Occurence::builder()
                                .language(language.to_owned())
                                .keyword(keyword.to_owned())
                                .build(),
                        );
                    }
                }
            }
        }
    }

    Ok(occurences)
}
