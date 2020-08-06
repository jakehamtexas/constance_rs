#[derive(Debug)]
pub struct QueryExecutionOptions {
    pub conn_string: String,
    pub query_timeout_in_ms: Option<i32>,
    pub should_parallelize: Option<bool>,
}

impl Default for QueryExecutionOptions {
    fn default() -> Self {
        Self {
            conn_string: "".to_string(),
            query_timeout_in_ms: Some(1000),
            should_parallelize: Some(false),
        }
    }
}

impl QueryExecutionOptions {
    pub fn default_with_conn_string(conn_string: String) -> QueryExecutionOptions {
        QueryExecutionOptions {
            conn_string,
            ..Default::default()
        }
    }
}
