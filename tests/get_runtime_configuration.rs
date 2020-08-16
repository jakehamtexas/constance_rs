use constance::{
    functions::get_runtime_configuration,
    types::{CliArgs, ConstanceRc, FileSystem, ICliArgs, RcParser},
};
use std::path::Path;

struct FromPath<'a> {
    pub path: &'a str,
}

impl ICliArgs for FromPath<'_> {
    fn _get_args(&self) -> Vec<String> {
        vec![self.path.to_owned()]
    }
}

fn from_path(path: &str) -> ConstanceRc {
    let cli_args = FromPath { path };
    let file_system = FileSystem {};
    let rc_parser = RcParser {};
    get_runtime_configuration(cli_args, file_system, rc_parser)
}

fn vec_assert(assertable: &Vec<String>) {
    string_assert(assertable.first().unwrap());
}

fn string_assert(assertable: &str) {
    assert_eq!(assertable, "");
}

fn bool_assert(assertable: bool) {
    assert!(!assertable);
}

fn num_assert(assertable: i32) {
    assert_eq!(assertable, 0);
}

fn assert_entire_rc(rc: ConstanceRc) {
    let output_options = rc.output_options.unwrap();

    let output_options_path = &output_options.path.unwrap();
    let language_targets = &output_options.language_targets.unwrap();

    string_assert(output_options_path);
    vec_assert(language_targets);

    let table_option = rc.table_options.first().unwrap();

    let identifier = &table_option.identifier;
    let instance_name = &identifier.instance_name;
    let database_name = &identifier.database_name;
    let schema_name = &identifier.schema_name;
    let object_name = &identifier.object_name;

    let key_column_name = &table_option.key_column_name;
    let value_column_names = &table_option.value_column_names;
    let description_column_name = &table_option.description_column_name.as_ref().unwrap();

    string_assert(instance_name);
    string_assert(database_name);
    string_assert(schema_name);
    string_assert(object_name);

    string_assert(key_column_name);
    vec_assert(value_column_names);
    string_assert(description_column_name);

    let query_execution_options = rc.query_execution_options;
    let conn_string = &query_execution_options.conn_string;
    let query_timeout_in_ms = query_execution_options.query_timeout_in_ms.unwrap();
    let should_parallelize = query_execution_options.should_parallelize.unwrap();

    string_assert(conn_string);
    num_assert(query_timeout_in_ms);
    bool_assert(should_parallelize);
}
#[test]
pub fn every_field_from_json() {
    // arrange
    let path = "./tests/rcs/json/every-field.json";

    // act
    let rc = from_path(path);

    // assert
    assert_entire_rc(rc);
}

#[test]
pub fn every_field_from_yaml() {
    // arrange
    let path = "./tests/rcs/yaml/every-field.yml";

    // act
    let rc = from_path(path);

    // assert
    assert_entire_rc(rc);
}
