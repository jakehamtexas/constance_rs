use super::super::super::abstraction::i_cli_args::ICliArgs;

pub struct GivesNoArg {}

impl ICliArgs for GivesNoArg {
    fn _get_args(&self) -> Vec<String> {
        vec![]
    }
}
