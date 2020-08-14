use super::super::abstraction::i_cli_args::ICliArgs;
use std::env;

pub struct CliArgs {}

impl ICliArgs for CliArgs {
    fn get_path(&self) -> String {
        let args = env::args().collect::<Vec<String>>();
        args.first()
            .expect("No configuration path specified!")
            .to_owned()
    }
}
