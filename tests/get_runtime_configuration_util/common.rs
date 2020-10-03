use constance::{
    functions::get_runtime_configuration,
    testing_only::ConstanceRc,
    types::{FileSystem, RcParser},
};

use super::super::common::from_path::FromPath;

pub fn from_path(path: &str) -> ConstanceRc {
    let cli_args = FromPath { path };
    let file_system = FileSystem {};
    let rc_parser = RcParser {};
    get_runtime_configuration(cli_args, file_system, rc_parser)
}

pub fn vec_assert(assertable: &Vec<String>) {
    string_assert(assertable.first().unwrap());
}

pub fn string_assert(assertable: &str) {
    assert_eq!(assertable, "");
}

pub fn bool_assert(assertable: bool) {
    assert!(!assertable);
}

pub fn num_assert(assertable: i32) {
    assert_eq!(assertable, 0);
}

pub fn option_assert<T>(option: &Option<T>) {
    assert!(option.is_none());
}

static PATH_PREFIX: &str = "./tests/get_runtime_configuration_util/rcs";

fn get_path<'a>(extension: &'a str) -> impl Fn(&str) -> String + 'a {
    move |filename_without_extension| {
        format!(
            "{}/{}/{}.{}",
            PATH_PREFIX, extension, filename_without_extension, extension
        )
    }
}

pub fn get_json(filename: &str) -> String {
    get_path("json")(filename)
}

pub fn get_yaml(filename: &str) -> String {
    get_path("yml")(filename)
}
