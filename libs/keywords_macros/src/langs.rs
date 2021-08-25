#[macro_export]
macro_rules! langs {
    ($($lang:expr),+) => {{
        let mut langs = HashMap::new();

        $(
        langs.insert($lang, read_keywords_file($lang).unwrap());
        )+

        langs
    }};
}
