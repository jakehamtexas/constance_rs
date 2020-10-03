use constance::testing_only::ICliArgs;

pub struct FromPath<'a> {
    pub path: &'a str,
}

impl ICliArgs for FromPath<'_> {
    fn _get_args(&self) -> Vec<String> {
        vec![self.path.to_owned()]
    }
}
