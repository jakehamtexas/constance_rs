use super::language::Language;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputOptions {
    pub language_targets: Option<Vec<String>>,
    pub path: Option<String>,
}

pub static DEFAULT_PATH: &str = "./";

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            language_targets: Some(vec![Language::default().to_string()]),
            // TODO: Change this to something meaningful at some point. Home directory?
            path: Some(String::from(DEFAULT_PATH)),
        }
    }
}
