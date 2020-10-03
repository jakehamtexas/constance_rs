use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

#[derive(Debug)]
pub struct ObjectLike {}
impl ObjectLike {
    pub async fn new(_option: &TableOption, _db: &Rdbms) -> Self {
        ObjectLike {}
    }
}
