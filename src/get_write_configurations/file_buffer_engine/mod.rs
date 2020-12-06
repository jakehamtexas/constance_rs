pub mod dotnet;
pub mod rust;
mod tokens;
pub mod typescript;

use crate::{
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::string_enum::StringEnum,
};

use self::tokens::QUOTATION_MARK;

pub trait FileBufferEngine {
    fn simple_enum(&self, _constant: &SimpleEnum) -> String;
    fn simple_enum_with_description(&self, _constant: &SimpleEnum) -> String;
    fn string_enum(&self, _constant: &StringEnum) -> String;
    fn string_enum_with_description(&self, _constant: &StringEnum) -> String;
    fn object_like(&self, _constant: &ObjectLike) -> String;
    fn object_like_with_description(&self, _constant: &ObjectLike) -> String;
}

pub enum FileBufferEngineType {
    Typesript(typescript::Typescript),
    Dotnet(dotnet::Dotnet),
    Rust(rust::Rust),
}

pub fn get_value(value: &str, quotes: bool) -> String {
    if quotes {
        [QUOTATION_MARK, value, QUOTATION_MARK].join("")
    } else {
        value.to_string()
    }
}
