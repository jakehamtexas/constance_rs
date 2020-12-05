use crate::{
    column::{Column, ID_COLUMN, NAME_COLUMN},
    insert_utils::to_substituted,
    sql_util::get_create_table_statement,
    sql_util::get_raw_insert_statement,
};
use serde::Deserialize;

const TABLE_NAME: &'static str = "object_like_enum";

pub static FIRST_COLUMN: Column = Column::Text("first");
pub static SECOND_COLUMN: Column = Column::Number("second");
static COLUMNS: &[&Column] = &[&ID_COLUMN, &NAME_COLUMN, &FIRST_COLUMN, &SECOND_COLUMN];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectLikeEnum {
    name: String,
    value: ObjectLikeEnumInner,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectLikeEnumInner {
    pub first: String,
    pub second: i32,
}

pub fn create_table_statement() -> Result<String, sql::Error> {
    get_create_table_statement(TABLE_NAME, &COLUMNS)
}

pub fn insert_statement(json: &Vec<ObjectLikeEnum>) -> Result<String, sql::Error> {
    let raw = get_raw_insert_statement(TABLE_NAME, &COLUMNS, json.len())?;
    Ok(json
        .iter()
        .enumerate()
        .map(|(index, ObjectLikeEnum { name, value })| {
            vec![
                index.to_string(),
                name.to_string(),
                value.first.to_string(),
                value.second.to_string(),
            ]
        })
        .fold(raw, |statement, args| to_substituted(&statement, &args)))
}
