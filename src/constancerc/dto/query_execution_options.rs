#[derive(Debug)]
pub struct QueryExecutionOptions<'a> {
    pub conn_string: &'a str,
    pub query_timeout_in_ms: Option<i32>,
    pub should_parallelize: Option<bool>,
}

impl Default for QueryExecutionOptions<'_> {
    fn default() -> Self {
        Self {
            conn_string: "",
            query_timeout_in_ms: Some(1000),
            should_parallelize: Some(false),
        }
    }
}

impl QueryExecutionOptions<'_> {
    pub fn default_with_conn_string(conn_string: &str) -> QueryExecutionOptions {
        QueryExecutionOptions {
            conn_string,
            ..Default::default()
        }
    }
}
