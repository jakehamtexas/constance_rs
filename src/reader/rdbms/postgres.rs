use crate::{
    constancerc::dto::table_option::TableOption,
    reader::{read_db::ReadDb, value_with_description::ValueWithDescription},
};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Postgres();

#[async_trait]
impl ReadDb for Postgres {
    async fn get_records_as_simple_key_value_pairs(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, String> {
        todo!()
    }

    async fn get_records_with_meta_description_column(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, ValueWithDescription> {
        todo!()
    }

    async fn get_records_as_object_like(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, Vec<(String, String)>> {
        todo!()
    }

    async fn get_records_as_object_like_with_descriptions(
        &self,
        table_option: &TableOption,
    ) -> HashMap<String, Vec<(String, ValueWithDescription)>> {
        todo!()
    }
}
