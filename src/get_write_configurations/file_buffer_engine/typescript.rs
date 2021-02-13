use std::collections::HashMap;

use crate::{
    get_write_configurations::casing_engine,
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::string_enum::StringEnum, testing_only::TableIdentifier,
    testing_only::ValueWithDescription,
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

    fn simple_enum_with_description(&self, constant: &SimpleEnum) -> String {
        self.simple_enum(constant)
    }

    fn string_enum(&self, constant: &StringEnum) -> String {
        primitive_enum(&constant.map, &constant.identifier, true)
    }

    fn string_enum_with_description(&self, constant: &StringEnum) -> String {
        self.string_enum(constant)
    }

    fn object_like(&self, _constant: &ObjectLike) -> String {
        todo!()
    }

    fn object_like_with_description(&self, _constant: &ObjectLike) -> String {
        todo!()
    }
}
