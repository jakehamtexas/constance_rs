use sql::prelude::*;

use crate::column::Column;

pub fn get_create_table_statement(
    table_name: &str,
    columns: &[&Column],
) -> Result<String, sql::Error> {
    columns
        .iter()
        .fold(create_table(table_name), |create_table, column| {
            create_table.column(match column {
                Column::Pkey => "id".integer().not_null(),
                Column::Text(name) => name.string(),
                Column::Number(name) => name.integer(),
            })
        })
        .compile()
}

pub fn get_raw_insert_statement(
    table_name: &str,
    columns: &[&Column],
    batch_size: usize,
) -> Result<String, sql::Error> {
    insert_into(table_name)
        .columns(
            &columns
                .iter()
                .map(|column| match column {
                    Column::Pkey => "id",
                    Column::Text(name) => name,
                    Column::Number(name) => name,
                })
                .collect::<Vec<&str>>(),
        )
        .batch(batch_size)
        .compile()
}

pub fn without_first_char(s: &str) -> String {
    s.char_indices()
        .next()
        .and_then(|(i, _)| s.get(i + 1..))
        .unwrap_or("")
        .to_owned()
}
