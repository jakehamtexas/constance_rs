pub mod output_options;
pub mod query_execution_options;
pub mod table_identifier;
pub mod table_option;

use table_option::TableOption;

pub mod language;
use output_options::OutputOptions;
use query_execution_options::QueryExecutionOptions;
#[derive(Debug)]
pub struct ConstanceRc<'a> {
    pub table_options: Vec<TableOption<'a>>,
    pub output_options: OutputOptions<'a>,
    pub query_execution_options: QueryExecutionOptions<'a>,
}
