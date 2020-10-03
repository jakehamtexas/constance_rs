use crate::{
    constancerc::dto::table_option::TableOption,
    reader::{rdbms::Rdbms, read_db::ReadDb, value_with_description::ValueWithDescription},
};
use std::collections::HashMap;
#[derive(Debug)]
pub struct StringEnumWithDescription {
    pub map: HashMap<String, ValueWithDescription>,
}
impl StringEnumWithDescription {
    pub async fn new(option: &TableOption, db: &Rdbms) -> Self {
        let map = match db {
            Rdbms::Mssql(db) => db.get_records_with_meta_description_column(option).await,
        };

        Self { map }
    }
}
