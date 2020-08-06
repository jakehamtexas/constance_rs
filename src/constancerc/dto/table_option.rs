use super::table_identifier::TableIdentifier;

#[derive(Debug)]
pub struct TableOption {
    identifier: TableIdentifier,
    key_column_name: String,
    value_column_names: Vec<String>,
}
