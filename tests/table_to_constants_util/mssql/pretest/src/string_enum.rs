use crate::{
    column::{Column, ID_COLUMN, NAME_COLUMN},
    insert_utils::to_substituted,
    sql_util::get_create_table_statement,
    sql_util::get_raw_insert_statement,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringEnum {
    name: String,
    string_id: String,
}

const TABLE_NAME: &'static str = "string_enum";
pub static STRING_ID_COLUMN: Column = Column::Text("string_id");
static COLUMNS: &[&Column] = &[&ID_COLUMN, &NAME_COLUMN, &STRING_ID_COLUMN];

pub fn create_table_statement() -> Result<String, sql::Error> {
    get_create_table_statement(TABLE_NAME, &COLUMNS)
}

pub fn insert_statement<'a>(json: &'a Vec<StringEnum>) -> Result<String, sql::Error> {
    let raw = get_raw_insert_statement(TABLE_NAME, &COLUMNS, json.len())?;
    Ok(json
        .iter()
        .enumerate()
        .map(|(index, StringEnum { name, string_id })| {
            vec![index.to_string(), name.to_string(), string_id.to_string()]
        })
        .fold(raw, |statement, args| to_substituted(&statement, &args)))
}
