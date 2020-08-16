use super::{super::constancerc::dto::ConstanceRc, rdbms::Rdbms};
pub fn get_database(rc: &ConstanceRc) -> Rdbms {
    Rdbms::from_string(&rc.query_execution_options.rdbms)
}
