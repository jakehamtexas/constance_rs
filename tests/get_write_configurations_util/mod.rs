use std::collections::HashMap;
pub mod dotnet_simple_enum_buffer;
pub mod dotnet_simple_enum_with_description_buffer;
pub mod dotnet_string_enum_buffer;
pub mod dotnet_string_enum_with_description_buffer;
pub mod rust_simple_enum_buffer;
pub mod rust_simple_enum_with_description_buffer;
pub mod rust_string_enum_buffer;
pub mod rust_string_enum_with_description_buffer;
pub mod typescript_simple_enum_buffer;
pub mod typescript_simple_enum_with_description_buffer;
pub mod typescript_string_enum_buffer;
pub mod typescript_string_enum_with_description_buffer;

use constance::{
    testing_only::{
        Language, SimpleEnum, StringEnum, StringEnumWithDescription, TableConstant,
        TableIdentifier, ValueWithDescription,
    },
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
    map.insert(
        "test1".to_string(),
        ValueWithDescription {
            value: "5".to_string(),
            description: None,
        },
    );
    map.insert(
        "test2".to_string(),
        ValueWithDescription {
            value: "7".to_string(),
            description: None,
        },
    );
    vec![TableConstant::SimpleEnum(SimpleEnum {
        identifier: TableIdentifier {
            object_name: "test_enum".to_string(),
            ..TableIdentifier::default()
        },
        map,
    })]
}

pub fn get_table_constants_for_string_enum_buffer_test() -> Vec<TableConstant> {
    let mut map = HashMap::new();
    map.insert(
        "test1".to_string(),
        ValueWithDescription {
            value: "test1".to_string(),
            description: None,
        },
    );
    map.insert(
        "test2".to_string(),
        ValueWithDescription {
            value: "test2".to_string(),
            description: None,
        },
    );
    vec![TableConstant::StringEnum(StringEnum {
        identifier: TableIdentifier {
            object_name: "test_enum".to_string(),
            ..TableIdentifier::default()
        },
        map,
    })]
}

pub fn get_table_constants_for_simple_enum_with_description_buffer_test() -> Vec<TableConstant> {
    let mut map = HashMap::new();
    map.insert(
        "test1".to_string(),
        ValueWithDescription {
            value: "5".to_string(),
            description: Some("description5".to_string()),
        },
    );
    map.insert(
        "test2".to_string(),
        ValueWithDescription {
            value: "7".to_string(),
            description: Some("description7".to_string()),
        },
    );
    vec![TableConstant::SimpleEnum(SimpleEnum {
        identifier: TableIdentifier {
            object_name: "test_enum".to_string(),
            ..TableIdentifier::default()
        },
        map,
    })]
}

pub fn get_table_constants_for_string_enum_with_description_buffer_test() -> Vec<TableConstant> {
    let mut map = HashMap::new();
    map.insert(
        "test1".to_string(),
        ValueWithDescription {
            value: "test1".to_string(),
            description: Some("description5".to_string()),
        },
    );
    map.insert(
        "test2".to_string(),
        ValueWithDescription {
            value: "test2".to_string(),
            description: Some("description7".to_string()),
        },
    );
    vec![TableConstant::StringEnumWithDescription(
        StringEnumWithDescription {
            identifier: TableIdentifier {
                object_name: "test_enum".to_string(),
                ..TableIdentifier::default()
            },
            map,
        },
    )]
}

pub fn get_output_options_for_filename_test(lang: Language) -> OutputOptions {
    OutputOptions {
        language_targets: Some(vec![lang.to_string()]),
        ..OutputOptions::default()
    }
}
