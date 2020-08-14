use super::{super::super::abstraction::i_cli_args::ICliArgs, gives_one_arg::FIRST_ARG};
pub struct GivesTwoArgs {}

impl ICliArgs for GivesTwoArgs {
    fn _get_args(&self) -> Vec<String> {
        vec![
            FIRST_ARG.to_string(),
            "my/path/to/another/file.json".to_string(),
        ]
    }
}
