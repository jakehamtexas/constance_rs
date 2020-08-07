use super::table_identifier::TableIdentifier;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TableOption {
    pub identifier: TableIdentifier,
    pub key_column_name: String,
    pub value_column_names: Vec<String>,
    pub description_column_name: Option<String>,
}
