use crate::{
    table_to_constants::table_constant::object_like::ObjectLike,
    table_to_constants::table_constant::object_like_with_description::ObjectLikeWithDescription,
    table_to_constants::table_constant::simple_enum::SimpleEnum,
    table_to_constants::table_constant::simple_enum_with_description::SimpleEnumWithDescription,
    table_to_constants::table_constant::string_enum::StringEnum,
    table_to_constants::table_constant::string_enum_with_description::StringEnumWithDescription,
};

use super::FileBufferEngine;
pub struct Dotnet {}

impl FileBufferEngine for Dotnet {
    fn simple_enum(&self, _constant: &SimpleEnum) -> String {
        todo!()
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
