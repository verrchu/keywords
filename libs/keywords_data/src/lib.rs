pub mod api;
mod error;
pub use error::Error;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use macros::langs;
use model::Keywords;

use once_cell::sync::Lazy;
use ron::de::from_reader;

static DATA: Lazy<Arc<HashMap<&str, Keywords>>> = Lazy::new(|| {
    let data = langs!("elixir", "erlang", "go", "java", "python", "ruby", "rust");

    Arc::new(data)
});

fn read_keywords_file(lang: &str) -> eyre::Result<Keywords> {
    let path = format!("{}/data/{}.ron", env!("CARGO_MANIFEST_DIR"), lang);
    let path = Path::new(&path);

    let file = File::open(path).map_err(eyre::Report::from)?;

    from_reader(BufReader::new(file)).map_err(eyre::Report::from)
}
