#[derive(Debug)]
pub struct TableIdentifier<'a> {
    instance_name: &'a str,
    database_name: &'a str,
    schema_name: &'a str,
    object_name: &'a str,
}
