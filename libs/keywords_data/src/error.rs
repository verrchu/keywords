use std::fmt::{self, Display};

use model::Language;

use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error("unsupported languages: {unsupported}. these are supported: {supported}")]
    UnsupportedLanguages {
        unsupported: Languages,
        supported: Languages,
    },
}

#[derive(Clone, Debug)]
pub struct Languages(Vec<Language>);

impl<I, L> From<I> for Languages
where
    L: Into<Language>,
    I: IntoIterator<Item = L>,
{
    fn from(input: I) -> Self {
        Self(input.into_iter().map(Into::into).collect())
    }
}

impl Display for Languages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = &self.0;

        if inner.is_empty() {
            write!(f, "<none>")?;
        }

        write!(f, "({})", inner.join(", "))
    }
}
