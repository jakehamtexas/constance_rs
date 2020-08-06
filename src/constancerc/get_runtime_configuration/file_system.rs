pub trait FileSystem {
    fn get_file(&self, path: String) -> Option<String>;
    fn get_extension(&self, path: String) -> RcFileExtension;
}

pub enum RcFileExtension {
    Json,
    Yaml,
}
