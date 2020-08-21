use crate::{constancerc::dto::table_option::TableOption, reader::rdbms::Rdbms};

pub struct SimpleEnum {}
impl SimpleEnum {
    pub fn new(option: &TableOption, db: &Rdbms) -> Self {
        SimpleEnum {}
    }
}
