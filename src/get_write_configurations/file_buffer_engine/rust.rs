use std::collections::HashMap;

use crate::{
    get_write_configurations::casing_engine,
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::object_like_with_description::ObjectLikeWithDescription,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::simple_enum_with_description::SimpleEnumWithDescription,
    table_to_constants::table_constant::string_enum::StringEnum,
    table_to_constants::table_constant::string_enum_with_description::StringEnumWithDescription,
    testing_only::TableIdentifier, testing_only::ValueWithDescription,
};

use super::{
    get_value, tokens::CLOSE_BRACE, tokens::COMMA, tokens::COMMENT_END, tokens::COMMENT_START,
    tokens::FOUR_SPACE_TAB, tokens::NEWLINE, tokens::OPEN_BRACE, tokens::SPACE, FileBufferEngine,
};

fn get_before(identifier: &TableIdentifier) -> String {
    let name = format!(
        "pub enum {}",
        casing_engine::pascal_case(&identifier.object_name)
    );
    [&name, SPACE, OPEN_BRACE, NEWLINE, FOUR_SPACE_TAB].join("")
}

fn get_after() -> String {
    [NEWLINE, CLOSE_BRACE, NEWLINE].join("")
}

pub struct Rust {}

fn primitive_enum(
    map: &HashMap<String, ValueWithDescription>,
    identifier: &TableIdentifier,
    quotes: bool,
) -> String {
    let members = map
        .iter()
        .map(|(key, ValueWithDescription { value, .. })| {
            format!(
                "{} = {}",
                casing_engine::pascal_case(key),
                get_value(value, quotes)
            )
        })
        .collect::<Vec<String>>()
        .join([COMMA, NEWLINE, FOUR_SPACE_TAB].join("").as_str());
    [
        get_before(identifier),
        members,
        COMMA.to_string(),
        get_after(),
    ]
    .join("")
}

fn primitive_enum_with_description(
    map: &HashMap<String, ValueWithDescription>,
    identifier: &TableIdentifier,
    quotes: bool,
) -> String {
    let before = get_before(identifier);
    let members = map
        .clone()
        .into_iter()
        .map(|(key, ValueWithDescription { value, description })| {
            let comment = [COMMENT_START, &description.unwrap(), COMMENT_END].join(" ");
            let member = format!(
                "{} = {}",
                casing_engine::pascal_case(&key),
                get_value(&value, quotes)
            );
            [comment, member].join(&format!("{}{}", NEWLINE, FOUR_SPACE_TAB))
        })
        .collect::<Vec<String>>()
        .join([COMMA, NEWLINE, NEWLINE, FOUR_SPACE_TAB].join("").as_str());
    let after = get_after();
    [before, members, COMMA.to_owned(), after].join("")
}

impl FileBufferEngine for Rust {
    fn simple_enum(&self, constant: &SimpleEnum) -> String {
        primitive_enum(&constant.map, &constant.identifier, false)
    }

    fn simple_enum_with_description(&self, constant: &SimpleEnumWithDescription) -> String {
        primitive_enum_with_description(&constant.map, &constant.identifier, false)
    }

    fn string_enum(&self, constant: &StringEnum) -> String {
        primitive_enum(&constant.map, &constant.identifier, true)
    }

    fn string_enum_with_description(&self, constant: &StringEnumWithDescription) -> String {
        primitive_enum_with_description(&constant.map, &constant.identifier, true)
    }

    fn object_like(&self, _constant: &ObjectLike) -> String {
        todo!()
    }

    fn object_like_with_description(&self, _constant: &ObjectLikeWithDescription) -> String {
        todo!()
    }
}
