use serde::Deserialize;

use crate::{
    column::{Column, DESCRIPTION_COLUMN, ID_COLUMN, NAME_COLUMN},
    insert_utils::to_substituted,
    object_like::{ObjectLikeEnumInner, FIRST_COLUMN, SECOND_COLUMN},
    sql_util::get_create_table_statement,
    sql_util::get_raw_insert_statement,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectLikeEnumWithDescription {
    name: String,
    description: String,
    value: ObjectLikeEnumInner,
}

const TABLE_NAME: &'static str = "object_like_enum_with_description";
static COLUMNS: &[&Column] = &[
    &ID_COLUMN,
    &NAME_COLUMN,
    &DESCRIPTION_COLUMN,
    &FIRST_COLUMN,
    &SECOND_COLUMN,
];

pub fn create_table_statement() -> Result<String, sql::Error> {
    get_create_table_statement(TABLE_NAME, &COLUMNS)
}

pub fn insert_statement<'a>(
    json: &'a Vec<ObjectLikeEnumWithDescription>,
) -> Result<String, sql::Error> {
    let raw = get_raw_insert_statement(TABLE_NAME, &COLUMNS, json.len())?;
    Ok(json
        .iter()
        .enumerate()
        .map(
            |(
                index,
                ObjectLikeEnumWithDescription {
                    name,
                    description,
                    value,
                },
            )| {
                vec![
                    index.to_string(),
                    name.to_string(),
                    description.to_string(),
                    value.first.to_string(),
                    value.second.to_string(),
                ]
            },
        )
        .fold(raw, |statement, args| to_substituted(&statement, &args)))
}
