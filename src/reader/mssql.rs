use super::super::abstraction::read_db::ReadDb;
pub struct MSSQL();

impl ReadDb for MSSQL {
    fn get_records_as_simple_key_value_pairs(
        &self,
        table_name: String,
    ) -> std::collections::HashMap<String, String> {
        todo!()
    }
    fn get_records_with_meta_description_column(
        &self,
        table_name: String,
    ) -> std::collections::HashMap<String, crate::dto::value_with_description::ValueWithDescription>
    {
        todo!()
    }
}
