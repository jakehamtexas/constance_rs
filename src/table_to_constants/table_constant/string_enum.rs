use crate::{
    constancerc::dto::table_option::TableOption,
    reader::{rdbms::Rdbms, read_db::ReadDb},
};
use std::collections::HashMap;
pub struct StringEnum {
    pub map: HashMap<String, String>,
}
impl StringEnum {
    pub async fn new(option: &TableOption, db: &Rdbms) -> Self {
        let map = match db {
            Rdbms::Mssql(db) => db.get_records_as_simple_key_value_pairs(option),
            _ => panic!("Unimplemented string enum query!"),
        }
        .await;
        Self { map }
    }
}
