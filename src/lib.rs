mod constancerc;
mod reader;
mod table_to_constants;
mod write_files_for_targets;

pub mod functions {
    pub use super::constancerc::get_runtime_configuration::get_runtime_configuration;
    pub use super::reader::manager::get_database;
    pub use super::table_to_constants::manager::get_table_constants;
    pub use super::write_files_for_targets::manager::write_files_for_targets;
}

pub mod types {
    pub use super::constancerc::dto::output_options::OutputOptions;
    pub use super::constancerc::get_runtime_configuration::concrete::{
        cli_args::CliArgs, file_system::FileSystem, rc_parser::RcParser,
    };
}

pub mod testing_only {
    pub use super::constancerc::dto::{
        column::Column,
        column_type::{NUMBER_TYPE, STRING_TYPE},
        connection::Connection,
        connection_options::ConnectionOptions,
        query_execution_options::QueryExecutionOptions,
        table_identifier::TableIdentifier,
        table_option::TableOption,
        ConstanceRc,
    };
    pub use super::constancerc::get_runtime_configuration::abstraction::{
        i_cli_args::ICliArgs, i_file_system::IFileSystem, i_rc_parser::IRcParser,
    };
    pub use super::reader::rdbms::MSSQL;
    pub use super::table_to_constants::table_constant::TableConstant;
}
