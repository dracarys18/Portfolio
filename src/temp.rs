use chrono::{Datelike, Local};
use serde::Serialize;

#[derive(Serialize)]
pub struct Index {
    name: String,
    year: String,
    version: String,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            name: "Karthikey Hegde".to_string(),
            year: Local::now().date().year().to_string(),
            version: rustc_version_runtime::version().to_string(),
        }
    }
}
