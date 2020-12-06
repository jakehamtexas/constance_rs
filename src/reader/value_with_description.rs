#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ValueWithDescription {
    pub value: String,
    pub description: String,
}
