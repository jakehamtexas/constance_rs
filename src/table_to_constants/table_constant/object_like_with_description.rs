use std::collections::HashMap;

use crate::{
    constancerc::dto::table_option::TableOption,
    reader::{rdbms::Rdbms, read_db::ReadDb},
    testing_only::{TableIdentifier, ValueWithDescription},
};

#[derive(Debug)]
pub struct ObjectLikeWithDescription {
    pub map: HashMap<ValueWithDescription, Vec<(String, String)>>,
    pub identifier: TableIdentifier,
}
impl ObjectLikeWithDescription {
    pub async fn new(option: &TableOption, db: &Rdbms) -> Self {
        let map = match db {
            Rdbms::Mssql(db) => {
                db.get_records_as_object_like_with_descriptions(option)
                    .await
            }
        };
        let identifier = option.identifier.clone();
        ObjectLikeWithDescription { map, identifier }
    }
}
