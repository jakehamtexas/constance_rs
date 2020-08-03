use super::super::dto::value_with_description::ValueWithDescription;
use std::collections::HashMap;

pub trait ReadDb {
    fn get_records_as_simple_key_value_pairs(&self, table_name: String) -> HashMap<String, String>;
    fn get_records_with_meta_description_column(
        &self,
        table_name: String,
    ) -> HashMap<String, ValueWithDescription>;
}
