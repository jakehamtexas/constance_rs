use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

#[derive(Debug)]
pub struct ObjectLikeWithDescription {}
impl ObjectLikeWithDescription {
    pub async fn new(_option: &TableOption, _db: &Rdbms) -> Self {
        ObjectLikeWithDescription {}
    }
}
