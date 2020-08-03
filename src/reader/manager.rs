use super::super::constancerc::ConstanceRc;
use super::mssql::MSSQL;
use super::read_db::ReadDb;
pub fn get_database(rc: &ConstanceRc) -> impl ReadDb {
    MSSQL {}
}
