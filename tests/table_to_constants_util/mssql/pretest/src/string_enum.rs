use crate::{
    column::{Column, ID_COLUMN, NAME_COLUMN},
    sql_util::get_create_table_statement,
    sql_util::get_raw_insert_statement,
    sql_util::without_first_char,
};

const TABLE_NAME: &'static str = "string_enum";
static STRING_ID_COLUMN: Column = Column::Text("string_id");
static COLUMNS: &[&Column] = &[&ID_COLUMN, &NAME_COLUMN, &STRING_ID_COLUMN];

pub fn create_table_statement() -> Result<String, sql::Error> {
    get_create_table_statement(TABLE_NAME, &COLUMNS)
}

pub fn insert_statement(json: &Vec<&str>) -> Result<String, sql::Error> {
    let _raw = get_raw_insert_statement(TABLE_NAME, &COLUMNS, json.len())?;
    todo!();
}
