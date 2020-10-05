use serde::Deserialize;

use crate::{
    column::{Column, DESCRIPTION_COLUMN, ID_COLUMN, NAME_COLUMN},
    insert_utils::to_substituted,
    sql_util::get_create_table_statement,
    sql_util::get_raw_insert_statement,
    string_enum::STRING_ID_COLUMN,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringEnumWithDescription {
    name: String,
    description: String,
    string_id: String,
}

const TABLE_NAME: &'static str = "string_enum_with_description";
static COLUMNS: &[&Column] = &[
    &ID_COLUMN,
    &NAME_COLUMN,
    &STRING_ID_COLUMN,
    &DESCRIPTION_COLUMN,
];

pub fn create_table_statement() -> Result<String, sql::Error> {
    get_create_table_statement(TABLE_NAME, &COLUMNS)
}

pub fn insert_statement<'a>(
    json: &'a Vec<StringEnumWithDescription>,
) -> Result<String, sql::Error> {
    let raw = get_raw_insert_statement(TABLE_NAME, &COLUMNS, json.len())?;
    Ok(json
        .iter()
        .enumerate()
        .map(
            |(
                index,
                StringEnumWithDescription {
                    name,
                    string_id,
                    description,
                },
            )| {
                vec![
                    index.to_string(),
                    name.to_owned(),
                    string_id.to_owned(),
                    description.to_owned(),
                ]
            },
        )
        .fold(raw, |statement, args| to_substituted(&statement, &args)))
}
