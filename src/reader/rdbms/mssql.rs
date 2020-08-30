use crate::{
    constancerc::dto::table_option::TableOption,
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Mssql();

impl ReadDb for Mssql {
    fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String> {
        let key_column_name = &table_option.key_column_name;
        let value_column_name = table_option.value_column_names.first().unwrap();
        todo!()
    }

    fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription> {
        todo!()
    }
}
