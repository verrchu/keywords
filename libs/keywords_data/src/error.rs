use model::Language;

#[derive(Clone, Debug)]
pub enum Error {
    UnsupportedLanguages {
        unsupported: Vec<Language>,
        supported: Vec<Language>,
    },
}
