use super::table_identifier::TableIdentifier;

#[derive(Debug)]
pub struct TableOption<'a> {
    identifier: TableIdentifier<'a>,
    key_column_name: &'a str,
    value_column_names: Vec<&'a str>,
}
