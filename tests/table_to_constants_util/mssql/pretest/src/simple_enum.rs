use crate::{
    column::{Column, ID_COLUMN, NAME_COLUMN},
    insert_utils::to_substituted,
    sql_util::get_create_table_statement,
    sql_util::get_raw_insert_statement,
};

const TABLE_NAME: &'static str = "simple_enum";
static COLUMNS: &[&Column] = &[&ID_COLUMN, &NAME_COLUMN];

pub fn create_table_statement() -> Result<String, sql::Error> {
    get_create_table_statement(TABLE_NAME, &COLUMNS)
}

pub fn insert_statement(json: &Vec<&str>) -> Result<String, sql::Error> {
    let raw = get_raw_insert_statement(TABLE_NAME, &COLUMNS, json.len())?;
    Ok(json
        .iter()
        .enumerate()
        .map(|(index, arg)| vec![index.to_string(), arg.to_string()])
        .fold(raw, |statement, args| to_substituted(&statement, &args)))
}
