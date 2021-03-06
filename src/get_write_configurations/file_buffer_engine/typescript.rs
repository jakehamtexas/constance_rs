use std::collections::HashMap;

use casing_engine::pascal_case;

use crate::{
    get_write_configurations::casing_engine,
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::string_enum::StringEnum,
    testing_only::TableIdentifier,
    testing_only::{Column, ValueWithDescription, STRING_TYPE},
};

static API_COMMENT_STAR: &str = "*";

fn get_name(identifier: &TableIdentifier) -> String {
    casing_engine::pascal_case(&identifier.object_name)
}
fn get_before(identifier: &TableIdentifier) -> String {
    let name = format!("enum {}", get_name(identifier));
    [&name, SPACE, OPEN_BRACE, NEWLINE, FOUR_SPACE_TAB].join("")
}
fn get_after(identifier: &TableIdentifier) -> String {
    let export = format!("export default {}", get_name(identifier));
    [NEWLINE, CLOSE_BRACE, NEWLINE, NEWLINE, &export].join("")
}
use super::{
    get_value, tokens::CLOSE_BRACE, tokens::COMMA, tokens::COMMENT_END, tokens::COMMENT_START,
    tokens::FOUR_SPACE_TAB, tokens::NEWLINE, tokens::OPEN_BRACE, tokens::SPACE, FileBufferEngine,
};
pub struct Typescript {}

fn primitive_enum(
    map: &HashMap<String, ValueWithDescription>,
    identifier: &TableIdentifier,
    quotes: bool,
) -> String {
    let map_size = map.len();
    let members = map
        .iter()
        .enumerate()
        .map(
            |(index, (key, ValueWithDescription { value, description }))| {
                let is_last_iteration = index == map_size - 1;
                let mut without_comment = if is_last_iteration {
                    format!(
                        "{} = {}",
                        casing_engine::pascal_case(key),
                        get_value(value, quotes)
                    )
                } else {
                    format!(
                        "{} = {}{}",
                        casing_engine::pascal_case(key),
                        get_value(value, quotes),
                        COMMA
                    )
                };

                if let Some(description) = description {
                    let comment_description = ["", API_COMMENT_STAR, &description].join(" ");
                    let comment_end = ["", COMMENT_END].join(" ");
                    let comment = [COMMENT_START.to_owned(), comment_description, comment_end]
                        .join(&format!("{}{}", NEWLINE, FOUR_SPACE_TAB));
                    if !is_last_iteration {
                        without_comment.push_str(NEWLINE);
                    }
                    [&comment, NEWLINE, FOUR_SPACE_TAB, &without_comment].join("")
                } else {
                    without_comment
                }
            },
        )
        .collect::<Vec<String>>()
        .join([NEWLINE, FOUR_SPACE_TAB].join("").as_str());
    [
        get_before(identifier),
        members,
        COMMA.to_string(),
        get_after(identifier),
    ]
    .join("")
}

impl FileBufferEngine for Typescript {
    fn simple_enum(&self, constant: &SimpleEnum) -> String {
        primitive_enum(&constant.map, &constant.identifier, false)
    }

    fn string_enum(&self, constant: &StringEnum) -> String {
        primitive_enum(&constant.map, &constant.identifier, true)
    }

    fn object_like(&self, constant: &ObjectLike) -> String {
        let const_object_name = get_name(&constant.identifier);
        let first_line = [
            format!("const {} = {{", const_object_name).as_str(),
            NEWLINE,
        ]
        .join("");
        let map_size = constant.map.len();
        let next_lines = constant
            .map
            .clone()
            .into_iter()
            .enumerate()
            .map(
                |(index, (ValueWithDescription { description, value }, column_value_pairs))| {
                    let is_last_iteration = index == map_size - 1;
                    let mut without_comment = [
                        vec![
                            FOUR_SPACE_TAB,
                            format!("{}: ", pascal_case(&value)).as_str(),
                            OPEN_BRACE,
                        ]
                        .join(""),
                        vec![
                            FOUR_SPACE_TAB,
                            column_value_pairs
                                .iter()
                                .map(|(Column { name, data_type }, value)| {
                                    format!(
                                        "{}: {},",
                                        name,
                                        if data_type == STRING_TYPE {
                                            format!("\"{}\"", value)
                                        } else {
                                            value.to_string()
                                        },
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join([NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB].join("").as_str())
                                .as_str(),
                        ]
                        .join(""),
                        vec![CLOSE_BRACE, COMMA, NEWLINE].join(""),
                    ]
                    .join([NEWLINE, FOUR_SPACE_TAB].join("").as_str());
                    if let Some(description) = description {
                        let comment_description = ["", API_COMMENT_STAR, &description].join(" ");
                        let comment_end = ["", COMMENT_END].join(" ");
                        let comment = [COMMENT_START.to_owned(), comment_description, comment_end]
                            .join(&format!("{}{}", NEWLINE, FOUR_SPACE_TAB));
                        if !is_last_iteration {
                            without_comment.push_str(NEWLINE);
                        }
                        [FOUR_SPACE_TAB, &comment, NEWLINE, &without_comment].join("")
                    } else {
                        without_comment
                    }
                },
            )
            .collect::<Vec<_>>()
            .join("");
        let last_lines = format!(
            "{} as const;{}{}export default {};{}",
            CLOSE_BRACE, NEWLINE, NEWLINE, const_object_name, NEWLINE
        );
        [first_line, next_lines, last_lines].join("")
    }
}
