use super::value_with_description::ValueWithDescription;
use crate::constancerc::dto::table_option::TableOption;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait ReadDb {
    async fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String>;
    async fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription>;
    async fn get_records_as_object_like(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, Vec<(String, String)>>;
    // TODO: Modify ConstanceRc schema so that there is a possibility
    // for a description column per each property column.
    async fn get_records_as_object_like_with_descriptions(
        &self,
        table_option: &TableOption,
    ) -> HashMap<ValueWithDescription, Vec<(String, String)>>;
}
