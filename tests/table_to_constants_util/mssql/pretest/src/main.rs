#![feature(str_split_once)]
use serde::Deserialize;
use std::fs::File;
use std::io::{prelude::*, Error};
use string_enum::StringEnum;

mod column;
mod insert_utils;
mod simple_enum;
mod sql_util;
mod string_enum;

fn get_file_contents(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_sql(statements: &[&str]) -> String {
    let create_database_staement = "CREATE DATABASE `test`";
    let use_statement = "USE test";
    let go_statement = "GO";
    let whitespace = "";

    let padded_statements = statements
        .into_iter()
        .flat_map(|statement| vec![use_statement, statement, go_statement, whitespace])
        .collect::<Vec<_>>();

    vec![create_database_staement, go_statement, whitespace]
        .into_iter()
        .chain(padded_statements)
        .collect::<Vec<&str>>()
        .join("\n")
        .replace("`", "")
}

fn get_buf(dir_path: &str, file_name_no_ext: &str) -> Result<String, std::io::Error> {
    let simple_enum_path = format!("{}/{}.json", dir_path, file_name_no_ext);
    get_file_contents(&simple_enum_path)
}

fn get_json<'a, T>(buf: &'a str) -> Result<Vec<T>, serde_json::Error>
where
    T: Deserialize<'a>,
{
    serde_json::from_str::<Vec<T>>(buf)
}

use std::env;
fn main() {
    let args = env::args().collect::<Vec<_>>();
    let dir_path = args
        .get(1)
        .expect("No directory path specified! It should be the first argument.");

    let simple_enum_buf = get_buf(dir_path, "simple_enum").unwrap();
    let simple_enum_json = get_json::<&str>(&simple_enum_buf).unwrap();

    let string_enum_buf = get_buf(dir_path, "string_enum").unwrap();
    let string_enum_json = get_json::<StringEnum>(&string_enum_buf).unwrap();

    let simple_enum_create_table = simple_enum::create_table_statement().unwrap();
    let simple_enum_insert = simple_enum::insert_statement(&simple_enum_json).unwrap();

    let string_enum_create_table = string_enum::create_table_statement().unwrap();
    let string_enum_insert = string_enum::insert_statement(&string_enum_json).unwrap();

    let sql = get_sql(&[
        &simple_enum_create_table,
        &simple_enum_insert,
        &string_enum_create_table,
        &string_enum_insert,
    ]);
    println!("{}", sql);
    std::fs::write("../init.sql", sql).expect("Unable to write to file.");
}
