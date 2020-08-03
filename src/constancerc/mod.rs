pub mod manager;
pub mod table_identifier;
pub mod table_option;

use table_option::TableOption;

pub mod language;
use language::Language;
#[derive(Debug)]
pub struct ConstanceRc {
    pub table_options: Vec<TableOption>,
    pub language_targets: Vec<Language>,
    pub conn_string: String,
    pub query_timeout_in_ms: i32,
    pub should_parallelize: bool,
}
