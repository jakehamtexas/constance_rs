use super::value_with_description::ValueWithDescription;
use crate::{constancerc::dto::table_option::TableOption, testing_only::Column};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait ReadDb {
    async fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription>;
    async fn get_records_as_object_like(
        &self,
        table_option: &TableOption,
    ) -> HashMap<ValueWithDescription, Vec<(Column, String)>>;
}
