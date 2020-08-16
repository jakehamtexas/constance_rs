use crate::reader::{read_db::ReadDb, value_with_description::ValueWithDescription};

#[derive(Debug)]
pub struct Mssql();

impl ReadDb for Mssql {
    fn get_records_as_simple_key_value_pairs(
        &self,
        table_name: String,
    ) -> std::collections::HashMap<String, String> {
        todo!()
    }
    fn get_records_with_meta_description_column(
        &self,
        table_name: String,
    ) -> std::collections::HashMap<String, ValueWithDescription> {
        todo!()
    }
}
