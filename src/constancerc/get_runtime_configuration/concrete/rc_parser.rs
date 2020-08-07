use super::super::i_rc_parser::IRcParser;
pub struct RcParser {}

impl IRcParser for RcParser {
    fn from_yaml<'a>(&self, buf: &str) -> crate::constancerc::dto::ConstanceRc<'a> {
        todo!()
    }
    fn from_json<'a>(&self, buf: &str) -> crate::constancerc::dto::ConstanceRc<'a> {
        todo!()
    }
}
