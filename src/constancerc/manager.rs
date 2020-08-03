use super::ConstanceRc;

pub fn get_runtime_configuration() -> ConstanceRc {
    ConstanceRc {
        table_options: vec![],
        language_targets: vec![],
    }
}
