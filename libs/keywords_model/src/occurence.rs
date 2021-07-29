use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Occurence {
    #[builder(setter(into))]
    word: String,
    #[builder(setter(into))]
    language: String,
    #[builder(default, setter(strip_option, into))]
    since: Option<String>,
}
