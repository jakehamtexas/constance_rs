use serde::Deserialize;
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableIdentifier {
    pub database_name: String,
    pub schema_name: String,
    pub object_name: String,
}
