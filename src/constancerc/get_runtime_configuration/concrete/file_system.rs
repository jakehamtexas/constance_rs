use super::super::abstraction::i_file_system::{IFileSystem, RcFileExtension};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{prelude::*, Error};
use std::path::Path;

fn get_extension_raw(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
pub struct FileSystem {}

impl IFileSystem for FileSystem {
    fn get_file(&self, path: &str) -> Result<String, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    fn get_extension(&self, path: &str) -> RcFileExtension {
        let extension = get_extension_raw(path)
            .expect(&format!("This file doesn't have an extension: {}", path));
        match extension {
            "json" => RcFileExtension::Json,
            "yml" => RcFileExtension::Yaml,
            _ => panic!(format!("This isn't a known file extension: {}", extension)),
        }
    }
}
