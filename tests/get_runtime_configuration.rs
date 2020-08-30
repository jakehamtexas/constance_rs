use get_runtime_configuration_util::{
    common::{from_path, get_json, get_yaml},
    every_field, only_required_fields,
};

mod get_runtime_configuration_util;

#[test]
pub fn every_field_from_json() {
    // arrange
    let path = get_json("every-field");

    // act
    let rc = from_path(&path);

    // assert
    every_field::assert(rc);
}

#[test]
pub fn every_field_from_yaml() {
    // arrange
    let path = get_yaml("every-field");

    // act
    let rc = from_path(&path);

    // assert
    every_field::assert(rc);
}

#[test]
pub fn only_required_fields_from_json() {
    // arrange
    let path = get_json("only-required-fields");

    // act
    let rc = from_path(&path);

    // assert
    only_required_fields::assert(rc);
}

#[test]
pub fn only_required_fields_from_yaml() {
    // arrange
    let path = get_yaml("only-required-fields");

    // act
    let rc = from_path(&path);

    // assert
    only_required_fields::assert(rc);
}

#[test]
#[should_panic]
pub fn incomplete_fields_from_json() {
    // arrange
    let path = get_json("incomplete-fields");

    // act
    from_path(&path);
}

#[test]
#[should_panic]
pub fn incomplete_fields_from_yaml() {
    // arrange
    let path = get_yaml("incomplete-fields");

    // act
    from_path(&path);
}
