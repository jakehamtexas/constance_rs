use super::super::dto::ConstanceRc;
pub trait RcParser {
    fn from_yaml(&self, buf: String) -> ConstanceRc;
    fn from_json(&self, buf: String) -> ConstanceRc;
}
