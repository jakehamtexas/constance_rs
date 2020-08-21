use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

pub struct SimpleEnumWithDescription {}
impl SimpleEnumWithDescription {
    pub fn new(option: &TableOption, db: &Rdbms) -> Self {
        SimpleEnumWithDescription {}
    }
}
