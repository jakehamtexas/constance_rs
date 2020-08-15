use constance::{
    get_database, get_runtime_configuration, get_table_constants, write_files_for_targets, CliArgs,
    FileSystem, RcParser,
};

fn main() {
    let cli_args = CliArgs {};
    let file_system = FileSystem {};
    let rc_parser = RcParser {};
    let rc = get_runtime_configuration(cli_args, file_system, rc_parser);

    let table_options = &rc.table_options;
    let output_options = &rc.output_options;

    let db = get_database(&rc);

    let table_constants = get_table_constants(db, table_options);

    write_files_for_targets(&table_constants, &output_options);
}
