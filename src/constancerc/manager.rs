use super::ConstanceRc;

pub fn get_runtime_configuration() -> ConstanceRc {
    ConstanceRc {
        table_options: Default::default(),
        output_options: Default::default(),
        query_execution_options: Default::default(),
    }
}
