#[derive(Debug)]
pub struct TableIdentifier {
    instance_name: String,
    database_name: String,
    schema_name: String,
    object_name: String,
}
