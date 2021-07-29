#[derive(Clone, Debug)]
pub enum Error {
    UnsupportedLanguages {
        unsupported: Vec<String>,
        supported: Vec<String>,
    },
}
