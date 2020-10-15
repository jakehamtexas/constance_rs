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

impl Language {
    pub fn to_string(&self) -> String {
        let val = match self {
            Language::Typescript => "typescript",
            Language::Rust => "rust",
            Language::Dotnet => "dotnet",
        };
        val.to_owned()
    }

    pub fn from_string(from: &str) -> Self {
        match from {
            "typescript" => Language::Typescript,
            "rust" => Language::Rust,
            "dotnet" => Language::Dotnet,
            _ => panic!("Unimplemented language found in runtime configuration!"),
        }
    }
}
