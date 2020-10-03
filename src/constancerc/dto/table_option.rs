use super::{column::Column, table_identifier::TableIdentifier};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableOption {
    pub identifier: TableIdentifier,
    pub key_column_name: String,
    pub value_columns: Vec<Column>,
    pub description_column_name: Option<String>,
}
