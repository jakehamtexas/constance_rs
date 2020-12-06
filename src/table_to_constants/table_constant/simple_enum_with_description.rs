use crate::{
    constancerc::dto::{table_identifier::TableIdentifier, table_option::TableOption},
    reader::{rdbms::Rdbms, read_db::ReadDb, value_with_description::ValueWithDescription},
};
use std::collections::HashMap;
#[derive(Debug)]
pub struct SimpleEnumWithDescription {
    pub map: HashMap<String, ValueWithDescription>,
    pub identifier: TableIdentifier,
}
impl SimpleEnumWithDescription {
    pub async fn new(option: &TableOption, db: &Rdbms) -> Self {
        let map = match db {
            Rdbms::Mssql(db) => db.get_records_as_simple_key_value_pairs(option).await,
        };
        let identifier = option.identifier.clone();
        Self { map, identifier }
    }
}
