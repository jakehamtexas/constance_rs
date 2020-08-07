use super::super::dto::ConstanceRc;
pub trait IRcParser {
    fn from_yaml<'a>(&self, buf: &str) -> Option<ConstanceRc>;
    fn from_json<'a>(&self, buf: &str) -> Option<ConstanceRc>;
}
