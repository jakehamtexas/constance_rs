use super::{
    super::dto::ConstanceRc,
    file_system::{FileSystem, RcFileExtension},
    rc_parser::RcParser,
};

pub fn get_runtime_configuration(
    path: String,
    file_system: impl FileSystem,
    rc_parser: impl RcParser,
) -> ConstanceRc {
    let buf = file_system
        .get_file(path)
        .expect(&format!("No configuration file found at path: {}", path));
    match file_system.get_extension(path) {
        RcFileExtension::Json => rc_parser.from_json(buf),
        RcFileExtension::Yaml => rc_parser.from_yaml(buf),
        _ => panic!("Unrecognized file type!"),
    }
}
