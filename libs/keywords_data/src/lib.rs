pub mod api;
mod error;
pub use error::Error;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use model::Keywords;

use indexmap::{indexmap, IndexMap};
use once_cell::sync::Lazy;
use ron::de::from_reader;

static DATA: Lazy<Arc<IndexMap<&str, Keywords>>> = Lazy::new(|| {
    let data = indexmap! {
        "rust" => read_keywords_file("rust").unwrap()
    };

    Arc::new(data)
});

fn read_keywords_file(lang: &str) -> eyre::Result<Keywords> {
    let path = format!("{}/data/{}.ron", env!("CARGO_MANIFEST_DIR"), lang);
    let path = Path::new(&path);

    let file = File::open(path).map_err(eyre::Report::from)?;

    from_reader(BufReader::new(file)).map_err(eyre::Report::from)
}
