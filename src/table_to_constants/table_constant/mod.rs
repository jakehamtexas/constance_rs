use super::table_constant::{
    object_like::ObjectLike, object_like_with_description::ObjectLikeWithDescription,
    simple_enum::SimpleEnum, simple_enum_with_description::SimpleEnumWithDescription,
    string_enum::StringEnum, string_enum_with_description::StringEnumWithDescription,
};
use crate::{
    constancerc::dto::{column_type::ColumnType, table_option::TableOption},
    reader::rdbms::Rdbms,
};

pub mod object_like;
pub mod object_like_with_description;
pub mod simple_enum;
pub mod simple_enum_with_description;
pub mod string_enum;
pub mod string_enum_with_description;

#[derive(Debug)]
pub enum TableConstant {
    SimpleEnum(SimpleEnum),
    SimpleEnumWithDescription(SimpleEnumWithDescription),
    StringEnum(StringEnum),
    StringEnumWithDescription(StringEnumWithDescription),
    ObjectLike(ObjectLike),
    ObjectLikeWithDescription(ObjectLikeWithDescription),
}

impl TableConstant {
    pub async fn from_option(option: &TableOption, db: &Rdbms) -> Self {
        let has_description = option.description_column_name.is_some();
        let has_multiple_values = option.value_columns.len() > 1;
        let first_value_column_type =
            ColumnType::from_string(&option.value_columns.first().unwrap().data_type);
        if !has_multiple_values {
            match (first_value_column_type, has_description) {
                (ColumnType::Number, false) => {
                    TableConstant::SimpleEnum(SimpleEnum::new(option, db).await)
                }
                (ColumnType::Number, true) => TableConstant::SimpleEnumWithDescription(
                    SimpleEnumWithDescription::new(option, db).await,
                ),
                (ColumnType::String, false) => {
                    TableConstant::StringEnum(StringEnum::new(option, db).await)
                }
                (ColumnType::String, true) => TableConstant::StringEnumWithDescription(
                    StringEnumWithDescription::new(option, db).await,
                ),
            }
        } else if has_description {
            TableConstant::ObjectLikeWithDescription(
                ObjectLikeWithDescription::new(option, db).await,
            )
        } else {
            TableConstant::ObjectLike(ObjectLike::new(option, db).await)
        }
    }
}
