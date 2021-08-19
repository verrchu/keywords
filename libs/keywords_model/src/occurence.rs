use typed_builder::TypedBuilder;

use crate::{language, Keyword, Language};

#[derive(Debug, Clone, TypedBuilder)]
pub struct Occurence {
    #[builder(setter(into))]
    keyword: Keyword,
    #[builder(setter(into))]
    language: Language,
    #[builder(default, setter(strip_option, into))]
    since: Option<language::Version>,
}
