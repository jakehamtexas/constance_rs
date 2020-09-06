pub enum KeyColumnType {
    String,
    Number,
}

const STRING_TYPE: &str = "string";
const NUMBER_TYPE: &str = "number";

impl KeyColumnType {
    pub fn from_option(opt: &Option<String>) -> Self {
        let key_column_type = match opt {
            Some(ref key_column) => &key_column,
            None => STRING_TYPE,
        };
        match key_column_type {
            STRING_TYPE => KeyColumnType::String,
            NUMBER_TYPE => KeyColumnType::Number,
            _ => panic!("Key column type {} not recognized", key_column_type),
        }
    }
}
