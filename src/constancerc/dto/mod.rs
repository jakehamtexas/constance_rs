pub mod key_column_type;
pub mod output_options;
pub mod query_execution_options;
pub mod table_identifier;
pub mod table_option;

use serde::Deserialize;
use table_option::TableOption;

pub mod language;
use output_options::OutputOptions;
use query_execution_options::QueryExecutionOptions;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstanceRc {
    pub table_options: Vec<TableOption>,
    pub output_options: Option<OutputOptions>,
    pub query_execution_options: QueryExecutionOptions,
}
