use super::super::abstraction::i_cli_args::ICliArgs;
use std::env;

pub struct CliArgs {}

impl ICliArgs for CliArgs {
    fn _get_args(&self) -> Vec<String> {
        env::args().collect::<Vec<String>>()
    }
}
