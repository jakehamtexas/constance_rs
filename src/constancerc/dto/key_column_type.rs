pub enum KeyColumnType {
    String,
    Number,
}

impl KeyColumnType {
    pub fn from_string(from: &str) -> Self {
        match from {
            "string" => KeyColumnType::String,
            "number" => KeyColumnType::Number,
            _ => panic!("Key column type {} not recognized", from),
        }
    }
}
