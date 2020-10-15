use crate::{
    constancerc::dto::output_options::OutputOptions,
    constancerc::dto::output_options::DEFAULT_PATH,
    get_write_configurations::write_configuration::WriteConfiguration,
};

fn write(write_configuration: &WriteConfiguration, dir_path: &str) -> std::io::Result<()> {
    let path =
        relative_path::RelativePath::new(dir_path).with_file_name(&write_configuration.filename);
    std::fs::write(path.to_string(), &write_configuration.buffer)
}

pub fn write_all(write_configurations: &[WriteConfiguration], output_options: &OutputOptions) {
    let path = match &output_options.path {
        Some(p) => p,
        None => DEFAULT_PATH,
    };

    for config in write_configurations.iter() {
        let res = write(config, path);
        if res.is_err() {
            panic!("File write unsuccessful!")
        }
    }
}
