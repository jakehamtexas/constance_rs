use super::super::i_rc_parser::IRcParser;
use crate::constancerc::dto::ConstanceRc;
use serde_json::Result as JsonResult;
use serde_yaml::Result as YamlResult;
pub struct RcParser {}

impl IRcParser for RcParser {
    fn from_yaml<'a>(&self, buf: &str) -> Option<ConstanceRc> {
        let rc: YamlResult<ConstanceRc> = serde_yaml::from_str(buf);
        rc.ok()
    }
    fn from_json<'a>(&self, buf: &str) -> Option<ConstanceRc> {
        let rc: JsonResult<ConstanceRc> = serde_json::from_str(buf);
        rc.ok()
    }
}
