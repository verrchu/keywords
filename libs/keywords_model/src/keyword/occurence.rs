use std::fmt::{self, Display};

use crate::{language, Keyword, Language};

use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Occurence {
    keyword: Keyword,
    #[builder(setter(into))]
    language: Language,
    #[builder(default, setter(strip_option, into))]
    since: Option<language::Version>,
}

impl Display for Occurence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.language)?;

        if let Some(version) = self.since.as_ref() {
            write!(f, " (since {})", version)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            "test".to_string(),
            Occurence::builder()
                .language("test")
                .keyword(Keyword::Regular("test".into()))
                .build()
                .to_string()
        );

        assert_eq!(
            "test (since test)".to_string(),
            Occurence::builder()
                .language("test")
                .keyword(Keyword::Regular("test".into()))
                .since("test")
                .build()
                .to_string()
        );
    }
}
