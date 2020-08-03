use super::super::constancerc::table_option::TableOption;
use super::super::reader::read_db::ReadDb;
use super::table_constant::TableConstant;

pub fn get_table_constants(
    db: impl ReadDb,
    table_options: &Vec<TableOption>,
) -> Vec<TableConstant> {
    vec![]
}
