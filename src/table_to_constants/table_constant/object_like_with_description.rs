use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

pub struct ObjectLikeWithDescription {}
impl ObjectLikeWithDescription {
    pub fn new(option: &TableOption, db: &Rdbms) -> Self {
        ObjectLikeWithDescription {}
    }
}
