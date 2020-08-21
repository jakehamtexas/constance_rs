use super::super::constancerc::dto::table_option::TableOption;
use super::table_constant::TableConstant;
use crate::reader::rdbms::Rdbms;

pub fn get_table_constants(db: Rdbms, table_options: &Vec<TableOption>) -> Vec<TableConstant> {
    table_options
        .iter()
        .map(|option| TableConstant::from_option(option, &db))
        .collect()
}
