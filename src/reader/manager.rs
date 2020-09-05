use super::{super::constancerc::dto::ConstanceRc, rdbms::Rdbms};
pub fn get_database(rc: &ConstanceRc) -> Rdbms {
    Rdbms::from_options(&rc.query_execution_options)
}
