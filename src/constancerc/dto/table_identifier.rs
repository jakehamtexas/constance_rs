use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct TableIdentifier {
    pub instance_name: String,
    pub database_name: String,
    pub schema_name: String,
    pub object_name: String,
}
