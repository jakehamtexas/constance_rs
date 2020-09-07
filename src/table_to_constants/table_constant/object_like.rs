use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

pub struct ObjectLike {}
impl ObjectLike {
    pub async fn new(_option: &TableOption, _db: &Rdbms) -> Self {
        ObjectLike {}
    }
}
