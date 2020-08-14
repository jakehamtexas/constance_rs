use super::super::super::abstraction::i_cli_args::ICliArgs;

pub static FIRST_ARG: &str = "my/path/to/file.json";

pub struct GivesOneArg {}

impl ICliArgs for GivesOneArg {
    fn _get_args(&self) -> Vec<String> {
        vec![FIRST_ARG.to_string()]
    }
}
