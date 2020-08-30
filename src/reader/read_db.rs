use super::value_with_description::ValueWithDescription;
use crate::constancerc::dto::table_option::TableOption;
use std::collections::HashMap;

pub trait ReadDb {
    fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String>;
    fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription>;
}
