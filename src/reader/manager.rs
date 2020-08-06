use super::super::constancerc::dto::ConstanceRc;
use super::mssql::MSSQL;
use super::read_db::ReadDb;
pub fn get_database(rc: &ConstanceRc) -> impl ReadDb {
    MSSQL {}
}
