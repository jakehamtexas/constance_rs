use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TableIdentifier {
    pub database_name: String,
    pub schema_name: String,
    pub object_name: String,
}
