use super::{connection_options::ConnectionOptions, language::Language};
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub connection_string: Option<String>,
    pub connection_options: Option<ConnectionOptions>,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            connection_string: None,
            connection_options: None,
        }
    }
}
