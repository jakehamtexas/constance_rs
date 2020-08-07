use super::language::Language;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct OutputOptions {
    pub language_targets: Option<Vec<Language>>,
    pub path: Option<String>,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            language_targets: Some(vec![Language::default()]),
            // TODO: Change this to something meaningful at some point. Home directory?
            path: Some(String::from("./")),
        }
    }
}
