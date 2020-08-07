use std::io::Error;

pub trait IFileSystem {
    fn get_file(&self, path: &str) -> Result<String, Error>;
    fn get_extension(&self, path: &str) -> RcFileExtension;
}

pub enum RcFileExtension {
    Json,
    Yaml,
}
