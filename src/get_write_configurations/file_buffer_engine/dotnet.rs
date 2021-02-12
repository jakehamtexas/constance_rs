use casing_engine::pascal_case;

use crate::{
    constancerc::dto::column_type::ColumnType,
    get_write_configurations::casing_engine,
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::string_enum::StringEnum,
    testing_only::Column,
    testing_only::{ValueWithDescription, STRING_TYPE},
};

use super::{
    tokens::CLOSE_BRACE, tokens::COMMA, tokens::FOUR_SPACE_TAB, tokens::NEWLINE,
    tokens::OPEN_BRACE, FileBufferEngine,
};

static API_COMMENT_SLASHES: &str = "///";
static SUMMARY_XML_OPEN: &str = "<summary>";
static SUMMARY_XML_CLOSE: &str = "</summary>";
pub struct Dotnet {}

enum EntityType {
    Enum,
    SealedClass,
}

fn get_before_for_enum(name: &str) -> String {
    get_before_for_entity_type(name, EntityType::Enum)
}

fn get_before_for_sealed_class(name: &str) -> String {
    get_before_for_entity_type(name, EntityType::SealedClass)
}

fn get_before_for_entity_type(name: &str, entity_type: EntityType) -> String {
    let namespace_statement = "namespace Constant";
    let entity = match entity_type {
        EntityType::Enum => "enum",
        EntityType::SealedClass => "sealed class",
    };
    let name = format!("public {} {}", entity, casing_engine::pascal_case(name));
    [
        namespace_statement,
        NEWLINE,
        OPEN_BRACE,
        NEWLINE,
        FOUR_SPACE_TAB,
        &name,
        NEWLINE,
        FOUR_SPACE_TAB,
        OPEN_BRACE,
        NEWLINE,
        FOUR_SPACE_TAB,
        FOUR_SPACE_TAB,
    ]
    .join("")
}

fn get_after() -> String {
    [
        NEWLINE,
        FOUR_SPACE_TAB,
        CLOSE_BRACE,
        NEWLINE,
        CLOSE_BRACE,
        NEWLINE,
    ]
    .join("")
}

fn get_type_name(data_type: &str) -> String {
    match ColumnType::from_string(data_type) {
        ColumnType::String => "string",
        ColumnType::Number => "int",
    }
    .to_string()
}

impl FileBufferEngine for Dotnet {
    fn simple_enum(&self, constant: &SimpleEnum) -> String {
        let before = get_before_for_enum(&constant.identifier.object_name);
        let members = constant
            .map
            .iter()
            .map(|(key, ValueWithDescription { value, .. })| {
                format!("{} = {}", casing_engine::pascal_case(key), value)
            })
            .collect::<Vec<String>>()
            .join(
                [COMMA, NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                    .join("")
                    .as_str(),
            );
        let after = get_after();
        [before, members, after].join("")
    }

    fn simple_enum_with_description(&self, constant: &SimpleEnum) -> String {
        let before = get_before_for_enum(&constant.identifier.object_name);
        let members = constant
            .map
            .clone()
            .into_iter()
            .map(|(key, ValueWithDescription { value, description })| {
                let comment_start = [API_COMMENT_SLASHES, SUMMARY_XML_OPEN].join(" ");
                let comment_description = [API_COMMENT_SLASHES, &description.unwrap()].join(" ");
                let comment_end = [API_COMMENT_SLASHES, SUMMARY_XML_CLOSE].join(" ");

                let member = format!("{} = {}", casing_engine::pascal_case(&key), value);
                [comment_start, comment_description, comment_end, member]
                    .join(&format!("{}{}{}", NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB))
            })
            .collect::<Vec<String>>()
            .join(
                [COMMA, NEWLINE, NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                    .join("")
                    .as_str(),
            );
        let after = get_after();
        [before, members, after].join("")
    }

    fn string_enum(&self, constant: &StringEnum) -> String {
        let before = get_before_for_enum(&constant.identifier.object_name);
        let members = constant
            .map
            .iter()
            .map(|(key, ValueWithDescription { value, .. })| {
                [
                    format!("[System.ComponentModel.Description(\"{}\")]", value),
                    casing_engine::pascal_case(key),
                ]
                .join([NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB].join("").as_str())
            })
            .collect::<Vec<String>>()
            .join(
                [COMMA, NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                    .join("")
                    .as_str(),
            );
        let after = get_after();
        [before, members, after].join("")
    }

    fn string_enum_with_description(&self, constant: &StringEnum) -> String {
        let before = get_before_for_enum(&constant.identifier.object_name);
        let members = constant
            .map
            .clone()
            .into_iter()
            .map(|(key, ValueWithDescription { value, description })| {
                let comment_start = [API_COMMENT_SLASHES, SUMMARY_XML_OPEN].join(" ");
                let comment_description = [API_COMMENT_SLASHES, &description.unwrap()].join(" ");
                let comment_end = [API_COMMENT_SLASHES, SUMMARY_XML_CLOSE].join(" ");

                let value = format!("[System.ComponentModel.Description(\"{}\")]", value);
                let key = casing_engine::pascal_case(&key);
                [comment_start, comment_description, comment_end, value, key]
                    .join(&format!("{}{}{}", NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB))
            })
            .collect::<Vec<String>>()
            .join(
                [COMMA, NEWLINE, NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                    .join("")
                    .as_str(),
            );
        let after = get_after();
        [before, members, after].join("")
    }

    fn object_like(&self, constant: &ObjectLike) -> String {
        let class_name = &constant.identifier.object_name;
        let before = get_before_for_sealed_class(class_name);
        let members = {
            let map_clone = constant.map.clone();
            let columns = map_clone
                .values()
                .nth(0)
                .unwrap()
                .into_iter()
                .map(|(column, ..)| column)
                .collect::<Vec<&Column>>();
            let properties = columns
                .iter()
                .map(|Column { data_type, name }| {
                    format!(
                        "public readonly {} {};",
                        get_type_name(data_type),
                        pascal_case(name)
                    )
                })
                .collect::<Vec<String>>()
                .join(
                    vec![NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                        .join("")
                        .as_str(),
                );
            let constructor = {
                let constructor_first_line = format!("private {}(", pascal_case(class_name));
                let args = columns
                    .iter()
                    .map(|Column { data_type, name }| {
                        format!(
                            "{} {}",
                            get_type_name(&data_type),
                            casing_engine::camel_case(&name)
                        )
                    })
                    .collect::<Vec<String>>();
                let constructor_assignments = columns
                    .iter()
                    .map(|Column { data_type: _, name }| {
                        vec![
                            format!(
                                "{} = {};",
                                casing_engine::pascal_case(name),
                                casing_engine::camel_case(name)
                            ),
                            NEWLINE.to_string(),
                            FOUR_SPACE_TAB.to_string(),
                            FOUR_SPACE_TAB.to_string(),
                        ]
                        .join("")
                    })
                    .collect::<Vec<String>>();
                vec![
                    vec![
                        FOUR_SPACE_TAB.to_string(),
                        FOUR_SPACE_TAB.to_string(),
                        constructor_first_line.to_owned(),
                        NEWLINE.to_string(),
                    ],
                    vec![vec![
                        args.iter()
                            .map(|arg| {
                                [FOUR_SPACE_TAB, FOUR_SPACE_TAB, FOUR_SPACE_TAB, arg].join("")
                            })
                            .collect::<Vec<String>>()
                            .join(vec![COMMA, NEWLINE].join("").as_str()),
                        ")".to_string(),
                        OPEN_BRACE.to_string(),
                        constructor_assignments
                            .iter()
                            .map(|assignment| format!("{}{}", FOUR_SPACE_TAB, assignment))
                            .collect::<Vec<String>>()
                            .join(""),
                    ]
                    .join(
                        vec![NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                            .join("")
                            .as_str(),
                    )],
                    vec![CLOSE_BRACE.to_string()],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<String>>()
                .join("")
            };

            let static_instances = vec![
                FOUR_SPACE_TAB.to_string(),
                FOUR_SPACE_TAB.to_string(),
                constant
                    .map
                    .clone()
                    .into_iter()
                    .map(
                        |(
                            ValueWithDescription {
                                value,
                                description: _,
                            },
                            columns,
                        )| {
                            vec![
                                format!(
                                    "public static {} {} = new {}(",
                                    pascal_case(class_name),
                                    pascal_case(&value),
                                    pascal_case(class_name)
                                ),
                                columns
                                    .iter()
                                    .map(|(Column { data_type, name: _ }, value)| {
                                        if data_type == STRING_TYPE {
                                            format!("\"{}\"", value)
                                        } else {
                                            value.to_string()
                                        }
                                    })
                                    .collect::<Vec<String>>()
                                    .join(", "),
                                ");".to_string(),
                            ]
                            .join("")
                        },
                    )
                    .collect::<Vec<String>>()
                    .join(&vec![NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB].join("")),
            ]
            .join("");
            [properties, constructor, static_instances]
                .join(vec![NEWLINE, NEWLINE].join("").as_str())
        };

        let after = get_after();
        [before, members, after].join("")
    }

    fn object_like_with_description(&self, constant: &ObjectLike) -> String {
        let class_name = &constant.identifier.object_name;
        let before = get_before_for_sealed_class(class_name);
        let members = {
            let map_clone = constant.map.clone();
            let columns = map_clone
                .values()
                .nth(0)
                .unwrap()
                .into_iter()
                .map(|(column, ..)| column)
                .collect::<Vec<&Column>>();
            let properties = columns
                .iter()
                .map(|Column { data_type, name }| {
                    format!(
                        "public readonly {} {};",
                        get_type_name(data_type),
                        pascal_case(name)
                    )
                })
                .collect::<Vec<String>>()
                .join(
                    vec![NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                        .join("")
                        .as_str(),
                );
            let constructor = {
                let constructor_first_line = format!("private {}(", pascal_case(class_name));
                let args = columns
                    .iter()
                    .map(|Column { data_type, name }| {
                        format!(
                            "{} {}",
                            get_type_name(&data_type),
                            casing_engine::camel_case(&name)
                        )
                    })
                    .collect::<Vec<String>>();
                let constructor_assignments = columns
                    .iter()
                    .map(|Column { data_type: _, name }| {
                        vec![
                            format!(
                                "{} = {};",
                                casing_engine::pascal_case(name),
                                casing_engine::camel_case(name)
                            ),
                            NEWLINE.to_string(),
                            FOUR_SPACE_TAB.to_string(),
                            FOUR_SPACE_TAB.to_string(),
                        ]
                        .join("")
                    })
                    .collect::<Vec<String>>();
                vec![
                    vec![
                        FOUR_SPACE_TAB.to_string(),
                        FOUR_SPACE_TAB.to_string(),
                        constructor_first_line.to_owned(),
                        NEWLINE.to_string(),
                    ],
                    vec![vec![
                        args.iter()
                            .map(|arg| {
                                [FOUR_SPACE_TAB, FOUR_SPACE_TAB, FOUR_SPACE_TAB, arg].join("")
                            })
                            .collect::<Vec<String>>()
                            .join(vec![COMMA, NEWLINE].join("").as_str()),
                        ")".to_string(),
                        OPEN_BRACE.to_string(),
                        constructor_assignments
                            .iter()
                            .map(|assignment| format!("{}{}", FOUR_SPACE_TAB, assignment))
                            .collect::<Vec<String>>()
                            .join(""),
                    ]
                    .join(
                        vec![NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                            .join("")
                            .as_str(),
                    )],
                    vec![CLOSE_BRACE.to_string()],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<String>>()
                .join("")
            };

            let static_instances = vec![
                FOUR_SPACE_TAB.to_string(),
                FOUR_SPACE_TAB.to_string(),
                constant
                    .map
                    .clone()
                    .into_iter()
                    .map(|(ValueWithDescription { value, description }, columns)| {
                        let comment_start = [API_COMMENT_SLASHES, SUMMARY_XML_OPEN].join(" ");
                        let comment_description =
                            [API_COMMENT_SLASHES, &description.unwrap()].join(" ");
                        let comment_end = [API_COMMENT_SLASHES, SUMMARY_XML_CLOSE].join(" ");

                        vec![
                            vec![comment_start, comment_description, comment_end].join(
                                vec![NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB]
                                    .join("")
                                    .as_str(),
                            ),
                            format!(
                                "{}public static {} {} = new {}(",
                                vec![NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB].join(""),
                                pascal_case(class_name),
                                pascal_case(&value),
                                pascal_case(class_name)
                            ),
                            columns
                                .iter()
                                .map(|(Column { data_type, name: _ }, value)| {
                                    if data_type == STRING_TYPE {
                                        format!("\"{}\"", value)
                                    } else {
                                        value.to_string()
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(", "),
                            ");".to_string(),
                        ]
                        .join("")
                    })
                    .collect::<Vec<String>>()
                    .join(&vec![NEWLINE, NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB].join("")),
            ]
            .join("");
            [properties, constructor, static_instances]
                .join(vec![NEWLINE, NEWLINE].join("").as_str())
        };

        let after = get_after();
        [before, members, after].join("")
    }
}
