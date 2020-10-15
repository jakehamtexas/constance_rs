use constance::{
    functions::{
        get_database, get_runtime_configuration, get_table_constants, get_write_configurations,
        write_all,
    },
    types::{CliArgs, FileSystem, OutputOptions, RcParser},
};

#[tokio::main]
async fn main() {
    let cli_args = CliArgs {};
    let file_system = FileSystem {};
    let rc_parser = RcParser {};
    let rc = get_runtime_configuration(cli_args, file_system, rc_parser);

    let db = get_database(&rc);

    let table_options = &rc.table_options;
    let table_constants = get_table_constants(db, table_options).await;

    let output_options = &rc.output_options.unwrap_or_else(OutputOptions::default);
    let write_configurations = get_write_configurations(&table_constants, &output_options);
    write_all(&write_configurations, &output_options);
}
