use crate::{
    get_write_configurations::casing_engine,
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::object_like_with_description::ObjectLikeWithDescription,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::simple_enum_with_description::SimpleEnumWithDescription,
    table_to_constants::table_constant::string_enum::StringEnum,
    table_to_constants::table_constant::string_enum_with_description::StringEnumWithDescription,
    testing_only::ValueWithDescription,
};

use super::{
    tokens::CLOSE_BRACE, tokens::COMMA, tokens::FOUR_SPACE_TAB, tokens::NEWLINE,
    tokens::OPEN_BRACE, FileBufferEngine,
};

static API_COMMENT_SLASHES: &str = "///";
static SUMMARY_XML_OPEN: &str = "<summary>";
static SUMMARY_XML_CLOSE: &str = "</summary>";
pub struct Dotnet {}

fn get_before(name: &str) -> String {
    let namespace_statement = "namespace Constant";
    let name = format!("public enum {}", casing_engine::pascal_case(name));
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

impl FileBufferEngine for Dotnet {
    fn simple_enum(&self, constant: &SimpleEnum) -> String {
        let before = get_before(&constant.identifier.object_name);
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

    fn simple_enum_with_description(&self, constant: &SimpleEnumWithDescription) -> String {
        let before = get_before(&constant.identifier.object_name);
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
        let before = get_before(&constant.identifier.object_name);
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

    fn string_enum_with_description(&self, constant: &StringEnumWithDescription) -> String {
        let before = get_before(&constant.identifier.object_name);
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

    fn object_like(&self, _constant: &ObjectLike) -> String {
        todo!()
    }

    fn object_like_with_description(&self, _constant: &ObjectLikeWithDescription) -> String {
        todo!()
    }
}
