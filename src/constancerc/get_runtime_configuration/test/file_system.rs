#![allow(unused_imports, dead_code)]
use crate::constancerc::get_runtime_configuration::abstraction::i_file_system::{
    IFileSystem, RcFileExtension,
};
use crate::constancerc::get_runtime_configuration::concrete::file_system::FileSystem;

fn get_path(extension: &str) -> String {
    format!("my/path/with/filename.{}", extension)
}

fn get_path_from_enum(extension: &RcFileExtension) -> String {
    let ext = match extension {
        RcFileExtension::Yaml => "yml",
        RcFileExtension::Json => "json",
    };
    get_path(ext)
}

#[test]
#[should_panic]
pub fn get_extension_foreign_extension_given_should_panic() {
    // arrange
    let file_system = FileSystem {};
    let expected = "txt";

    let path = get_path(expected);

    // act
    file_system.get_extension(&path);
}

#[test]
#[should_panic]
pub fn get_extension_no_extension_given_should_panic() {
    // arrange
    let file_system = FileSystem {};
    let expected = "";

    let path = get_path(expected);

    // act
    file_system.get_extension(&path);
}

#[test]
pub fn get_extension_json_extension_given_returns_json() {
    // arrange
    let file_system = FileSystem {};
    let expected = RcFileExtension::Json;

    let path = get_path_from_enum(&expected);

    // act
    let actual = file_system.get_extension(&path);

    // assert
    assert_eq!(actual, expected);
}

#[test]
pub fn get_extension_yml_extension_given_returns_yml() {
    // arrange
    let file_system = FileSystem {};
    let expected = RcFileExtension::Yaml;

    let path = get_path_from_enum(&expected);

    // act
    let actual = file_system.get_extension(&path);

    // assert
    assert_eq!(actual, expected);
}
