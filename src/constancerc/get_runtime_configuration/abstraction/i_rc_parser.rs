use super::super::super::dto::ConstanceRc;
pub trait IRcParser {
    fn from_yaml(&self, buf: &str) -> Option<ConstanceRc>;
    fn from_json(&self, buf: &str) -> Option<ConstanceRc>;
}
