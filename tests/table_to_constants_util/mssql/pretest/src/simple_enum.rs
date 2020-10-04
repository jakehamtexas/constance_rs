use crate::{
    column::{Column, ID_COLUMN, NAME_COLUMN},
    sql_util::get_create_table_statement,
    sql_util::get_raw_insert_statement,
    sql_util::without_first_char,
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
        .fold(raw, |simple_enum_insert, (index, name)| {
            let first_two_value_indices = simple_enum_insert
                .match_indices("?")
                .take(COLUMNS.len())
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            let (first_index, second_index) = (
                first_two_value_indices.first().unwrap(),
                first_two_value_indices.get(1).unwrap(),
            );
            let (before, after_first) = simple_enum_insert.split_at(*first_index);
            let (between, after) = after_first.split_at(*second_index - before.len());

            let between_with_value = format!("{}{}", index, without_first_char(between));
            let after_with_value = format!("'{}'{}", name, without_first_char(after));

            [before.to_owned(), between_with_value, after_with_value].join("")
        }))
}
