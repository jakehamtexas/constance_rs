use super::dto::ConstanceRc;
use i_file_system::{IFileSystem, RcFileExtension};
use i_rc_parser::IRcParser;

pub mod concrete;
pub mod i_file_system;
pub mod i_rc_parser;

pub fn get_runtime_configuration<'a>(
    path: &'a str,
    file_system: impl IFileSystem,
    rc_parser: impl IRcParser,
) -> ConstanceRc {
    let buf = file_system
        .get_file(path)
        .expect(&format!("No configuration file found at path: {}", path));
    let result = match file_system.get_extension(path) {
        RcFileExtension::Json => rc_parser.from_json(&buf),
        RcFileExtension::Yaml => rc_parser.from_yaml(&buf),
        _ => panic!("Unrecognized file type!"),
    };
    result.expect(&format!("Unable to parse {}", path))
}
