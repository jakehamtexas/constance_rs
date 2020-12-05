use inflector::cases::{camelcase, pascalcase, snakecase};
#[allow(dead_code)]
pub fn camel_case(name: &str) -> String {
    camelcase::to_camel_case(name)
}

pub fn snake_case(name: &str) -> String {
    snakecase::to_snake_case(name)
}

pub fn pascal_case(name: &str) -> String {
    pascalcase::to_pascal_case(name)
}
