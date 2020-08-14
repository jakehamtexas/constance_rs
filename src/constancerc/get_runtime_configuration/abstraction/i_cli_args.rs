pub trait ICliArgs {
    fn get_path(&self) -> String {
        let args = self._get_args();
        args.first()
            .expect("No configuration path specified!")
            .to_owned()
    }
    fn _get_args(&self) -> Vec<String>;
}
