pub enum ColumnType {
    String,
    Number,
}

pub const STRING_TYPE: &str = "string";
pub const NUMBER_TYPE: &str = "number";

impl ColumnType {
    pub fn from_string(column_type: &str) -> Self {
        match column_type {
            STRING_TYPE => ColumnType::String,
            NUMBER_TYPE => ColumnType::Number,
            _ => panic!("Key column type {} not recognized", column_type),
        }
    }
}
