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
