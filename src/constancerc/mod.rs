pub mod manager;
pub mod table_option;

use table_option::TableOption;

pub mod language;
use language::Language;
#[derive(Debug)]
pub struct ConstanceRc {
    pub table_options: Vec<TableOption>,
    pub language_targets: Vec<Language>,
}
