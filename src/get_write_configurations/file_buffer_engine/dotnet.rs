use std::path::Path;

use crate::{
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::object_like_with_description::ObjectLikeWithDescription,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::simple_enum_with_description::SimpleEnumWithDescription,
    table_to_constants::table_constant::string_enum::StringEnum,
    table_to_constants::table_constant::string_enum_with_description::StringEnumWithDescription,
};

use super::{
    tokens::CLOSE_BRACE, tokens::FOUR_SPACE_TAB, tokens::NEWLINE, tokens::OPEN_BRACE,
    FileBufferEngine,
};
pub struct Dotnet {}

fn get_before(name: &str) -> String {
    let namespace_statement = "namespace Constant";
    let name = format!("enum {}", name);
    [
        namespace_statement,
        NEWLINE,
        OPEN_BRACE,
        NEWLINE,
        FOUR_SPACE_TAB,
        &name,
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
            .map(|(key, value)| format!("{} = {}", key, value))
            .collect::<Vec<String>>()
            .join([NEWLINE, FOUR_SPACE_TAB, FOUR_SPACE_TAB].join("").as_str());
        let after = get_after();
        [before, members, after].join("")
    }

    fn simple_enum_with_description(&self, _constant: &SimpleEnumWithDescription) -> String {
        todo!()
    }

    fn string_enum(&self, _constant: &StringEnum) -> String {
        todo!()
    }

    fn string_enum_with_description(&self, _constant: &StringEnumWithDescription) -> String {
        todo!()
    }

    fn object_like(&self, _constant: &ObjectLike) -> String {
        todo!()
    }

    fn object_like_with_description(&self, _constant: &ObjectLikeWithDescription) -> String {
        todo!()
    }
}
