use std::collections::HashMap;
pub mod dotnet_simple_enum_buffer;

use constance::{
    testing_only::{Language, SimpleEnum, TableConstant, TableIdentifier},
    types::OutputOptions,
};

pub fn get_table_constants_for_filename_test() -> Vec<TableConstant> {
    vec![TableConstant::SimpleEnum(SimpleEnum {
        identifier: TableIdentifier {
            object_name: "test_enum".to_string(),
            ..TableIdentifier::default()
        },
        ..SimpleEnum::default()
    })]
}

pub fn get_table_constants_for_simple_enum_buffer_test() -> Vec<TableConstant> {
    let mut map = HashMap::new();
    map.insert("test1".to_string(), "5".to_string());
    map.insert("test2".to_string(), "7".to_string());
    vec![TableConstant::SimpleEnum(SimpleEnum {
        identifier: TableIdentifier {
            object_name: "test_enum".to_string(),
            ..TableIdentifier::default()
        },
        map,
    })]
}

pub fn get_output_options_for_filename_test(lang: Language) -> OutputOptions {
    OutputOptions {
        language_targets: Some(vec![lang.to_string()]),
        ..OutputOptions::default()
    }
}
