use super::language::Language;
#[derive(Debug)]
pub struct OutputOptions<'a> {
    pub language_targets: Option<Vec<Language>>,
    pub path: Option<&'a str>,
}

impl Default for OutputOptions<'_> {
    fn default() -> Self {
        Self {
            language_targets: Some(vec![Language::default()]),
            // TODO: Change this to something meaningful at some point. Home directory?
            path: Some("./"),
        }
    }
}
