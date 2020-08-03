use super::ConstanceRc;

pub fn get_runtime_configuration() -> ConstanceRc {
    ConstanceRc {
        table_options: vec![],
        language_targets: vec![],
        conn_string: "".to_string(),
        query_timeout_in_ms: 0,
        should_parallelize: true,
    }
}
