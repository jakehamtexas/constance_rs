use super::table_constant::{
    object_like::ObjectLike, object_like_with_description::ObjectLikeWithDescription,
    simple_enum::SimpleEnum, simple_enum_with_description::SimpleEnumWithDescription,
    string_enum::StringEnum, string_enum_with_description::StringEnumWithDescription,
};
use crate::{
    constancerc::dto::{key_column_type::KeyColumnType, table_option::TableOption},
    reader::rdbms::Rdbms,
};

pub mod object_like;
pub mod object_like_with_description;
pub mod simple_enum;
pub mod simple_enum_with_description;
pub mod string_enum;
pub mod string_enum_with_description;

pub enum TableConstant {
    SimpleEnum(SimpleEnum),
    SimpleEnumWithDescription(SimpleEnumWithDescription),
    StringEnum(StringEnum),
    StringEnumWithDescription(StringEnumWithDescription),
    ObjectLike(ObjectLike),
    ObjectLikeWithDescription(ObjectLikeWithDescription),
}

fn get_key_column_type(option: &TableOption) -> KeyColumnType {
    let key_column_type = match option.key_column_type {
        Some(ref key_column) => &key_column,
        None => "string",
    };
    KeyColumnType::from_string(key_column_type)
}

impl TableConstant {
    pub async fn from_option(option: &TableOption, db: &Rdbms) -> Self {
        let has_description = option.description_column_name.is_some();
        let has_multiple_values = option.value_column_names.len() > 1;
        let key_column_type = get_key_column_type(option);
        match (key_column_type, has_description, has_multiple_values) {
            (KeyColumnType::Number, false, false) => {
                TableConstant::SimpleEnum(SimpleEnum::new(option, db).await)
            }
            (KeyColumnType::Number, true, false) => {
                TableConstant::SimpleEnumWithDescription(SimpleEnumWithDescription::new(option, db))
            }
            (KeyColumnType::String, false, false) => {
                TableConstant::StringEnum(StringEnum::new(option, db))
            }
            (KeyColumnType::String, true, false) => {
                TableConstant::StringEnumWithDescription(StringEnumWithDescription::new(option, db))
            }
            (KeyColumnType::String, false, true) => {
                TableConstant::ObjectLike(ObjectLike::new(option, db))
            }
            (KeyColumnType::String, true, true) => {
                TableConstant::ObjectLikeWithDescription(ObjectLikeWithDescription::new(option, db))
            }
            _ => panic!("Unimplemented table option configuration"),
        }
    }
}
