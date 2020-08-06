use constancerc::get_runtime_configuration::file_system::FileSystem;

mod constancerc;
mod reader;
mod table_to_constants;
mod write_files_for_targets;
fn main() {
    let rc = constancerc::get_runtime_configuration::get_runtime_configuration();

    let table_options = &rc.table_options;
    let output_options = &rc.output_options;

    let db = reader::manager::get_database(&rc);

    let table_constants = table_to_constants::manager::get_table_constants(db, table_options);

    write_files_for_targets::manager::write_files_for_targets(&table_constants, &output_options);
}
