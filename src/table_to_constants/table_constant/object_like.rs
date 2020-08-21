use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

pub struct ObjectLike {}
impl ObjectLike {
    pub fn new(option: &TableOption, db: &Rdbms) -> Self {
        ObjectLike {}
    }
}
