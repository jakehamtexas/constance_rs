use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub enum Language {
    Typescript,
    Rust,
    Dotnet,
}

impl Default for Language {
    fn default() -> Self {
        Language::Dotnet
    }
}
