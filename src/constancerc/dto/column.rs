use super::column_type::{NUMBER_TYPE, STRING_TYPE};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub name: String,
    pub data_type: String,
}

impl Column {
    pub fn string(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data_type: STRING_TYPE.to_string(),
        }
    }
    pub fn number(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data_type: NUMBER_TYPE.to_string(),
        }
    }
}
