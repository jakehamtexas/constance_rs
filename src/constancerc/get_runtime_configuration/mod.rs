use super::dto::ConstanceRc;
use abstraction::{
    i_cli_args::ICliArgs,
    i_file_system::{IFileSystem, RcFileExtension},
    i_rc_parser::IRcParser,
};

pub mod abstraction;
pub mod concrete;
pub mod test;

pub fn get_runtime_configuration<'a>(
    cli_args: impl ICliArgs,
    file_system: impl IFileSystem,
    rc_parser: impl IRcParser,
) -> ConstanceRc {
    let path = cli_args.get_path();
    let buf = file_system
        .get_file(&path)
        .expect(&format!("No configuration file found at path: {}", path));
    let result = match file_system.get_extension(&path) {
        RcFileExtension::Json => rc_parser.from_json(&buf),
        RcFileExtension::Yaml => rc_parser.from_yaml(&buf),
    };
    result.expect(&format!("Unable to parse {}", path))
}
