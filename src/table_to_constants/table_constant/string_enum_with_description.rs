use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

pub struct StringEnumWithDescription {}
impl StringEnumWithDescription {
    pub fn new(option: &TableOption, db: &Rdbms) -> Self {
        StringEnumWithDescription {}
    }
}
