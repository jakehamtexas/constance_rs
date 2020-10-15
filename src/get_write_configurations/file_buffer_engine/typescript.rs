use crate::{
    get_write_configurations::casing_engine,
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::object_like_with_description::ObjectLikeWithDescription,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::simple_enum_with_description::SimpleEnumWithDescription,
    table_to_constants::table_constant::string_enum::StringEnum,
    table_to_constants::table_constant::string_enum_with_description::StringEnumWithDescription,
};
fn get_before(name: &str) -> String {
    let name = format!("enum {}", name);
    [&name, SPACE, OPEN_BRACE, NEWLINE, FOUR_SPACE_TAB].join("")
}
fn get_after(name: &str) -> String {
    let export = format!("export default {}", name);
    [NEWLINE, CLOSE_BRACE, NEWLINE, NEWLINE, &export].join("")
}
use super::{
    tokens::CLOSE_BRACE, tokens::FOUR_SPACE_TAB, tokens::NEWLINE, tokens::OPEN_BRACE,
    tokens::SPACE, FileBufferEngine,
};
pub struct Typescript {}

impl FileBufferEngine for Typescript {
    fn simple_enum(&self, constant: &SimpleEnum) -> String {
        let members = constant
            .map
            .iter()
            .map(|(key, value)| format!("{} = {}", casing_engine::pascal_case(key), value))
            .collect::<Vec<String>>()
            .join([NEWLINE, FOUR_SPACE_TAB].join("").as_str());
        let name = casing_engine::pascal_case(&constant.identifier.object_name);
        [get_before(&name), members, get_after(&name)].join("")
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
