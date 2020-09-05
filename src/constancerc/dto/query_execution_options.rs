use super::connection::Connection;
use crate::reader::rdbms::Rdbms;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryExecutionOptions {
    pub connection: Connection,
    pub rdbms: String,
    pub query_timeout_in_ms: Option<i32>,
    pub should_parallelize: Option<bool>,
}

impl Default for QueryExecutionOptions {
    fn default() -> Self {
        Self {
            connection: Connection::default(),
            rdbms: Rdbms::default().to_rc_string(),
            query_timeout_in_ms: Some(1000),
            should_parallelize: Some(false),
        }
    }
}
