mod flat;
mod versioned;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Keywords {
    Versioned(versioned::Keywords),
    Flat(flat::Keywords),
}
