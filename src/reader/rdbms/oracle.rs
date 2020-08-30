use crate::{
    constancerc::dto::table_option::TableOption,
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Oracle();

impl ReadDb for Oracle {
    fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String> {
        todo!()
    }

    fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription> {
        todo!()
    }
}
