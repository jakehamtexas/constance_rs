use column::Column;
use sql::prelude::*;
use std::fs::File;
use std::io::{prelude::*, Error};

mod column;
fn get_file_contents(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn without_first_char(s: &str) -> String {
    s.char_indices()
        .next()
        .and_then(|(i, _)| s.get(i + 1..))
        .unwrap_or("")
        .to_owned()
}

fn get_create_table_statement(table_name: &str, columns: &[Column]) -> Result<String, sql::Error> {
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

fn get_raw_insert_statement(
    table_name: &str,
    columns: &[Column],
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

use std::env;
fn main() {
    let args = env::args().collect::<Vec<_>>();
    let dir_path = args
        .get(1)
        .expect("No directory path specified! It should be the first argument.");

    let simple_enum_path = format!("{}/{}.json", dir_path, "simple_enum");
    let simple_enum_buf = get_file_contents(&simple_enum_path).expect("Invalid directory path!");
    let simple_enum_json =
        serde_json::from_str::<Vec<&str>>(&simple_enum_buf).expect("Unable to deserialize!");

    let create_database_statement = "CREATE DATABASE `test`".to_string();

    let id_column = Column::Pkey;
    let name_column = Column::Text("name");
    let table_name = "simple_enum";

    let columns = vec![id_column, name_column];
    let simple_enum_create_table = get_create_table_statement(table_name, &columns).unwrap();
    let simple_enum_raw_insert =
        get_raw_insert_statement(table_name, &columns, simple_enum_json.len()).unwrap();

    let simple_enum_insert = simple_enum_json.iter().enumerate().fold(
        simple_enum_raw_insert,
        |simple_enum_insert, (index, name)| {
            let first_two_value_indices = simple_enum_insert
                .match_indices("?")
                .take(columns.len())
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
        },
    );
    let sql = vec![
        create_database_statement,
        simple_enum_create_table,
        simple_enum_insert,
    ]
    .join("\n")
    .replace("`", "");

    std::fs::write("../init.sql", sql).expect("Unable to write to file.");
}