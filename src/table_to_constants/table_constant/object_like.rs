use std::{collections::HashMap, fmt::Debug};

use crate::{
    constancerc::dto::table_option::TableOption,
    reader::{rdbms::Rdbms, read_db::ReadDb},
    testing_only::TableIdentifier,
};

#[derive(Debug)]
pub struct ObjectLike {
    pub map: HashMap<String, Vec<(String, String)>>,
    pub identifier: TableIdentifier,
}
impl ObjectLike {
    pub async fn new(option: &TableOption, db: &Rdbms) -> Self {
        let map = match db {
            Rdbms::Mssql(db) => db.get_records_as_object_like(option).await,
        };
        let identifier = option.identifier.clone();
        ObjectLike { map, identifier }
    }
}
