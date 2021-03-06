use constance::{
    functions::get_write_configurations, testing_only::Language, testing_only::TableConstant,
    types::OutputOptions,
};
use get_write_configurations_util::{
    dotnet_object_like_enum_buffer::{
        DOTNET_OBJECT_LIKE_ENUM_BUFFER1, DOTNET_OBJECT_LIKE_ENUM_BUFFER2,
    },
    dotnet_object_like_enum_with_description_buffer::{
        DOTNET_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER1,
        DOTNET_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER2,
    },
    dotnet_simple_enum_buffer::DOTNET_SIMPLE_ENUM_BUFFER1,
    dotnet_simple_enum_buffer::DOTNET_SIMPLE_ENUM_BUFFER2,
    dotnet_simple_enum_with_description_buffer::DOTNET_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER1,
    dotnet_simple_enum_with_description_buffer::DOTNET_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER2,
    dotnet_string_enum_buffer::DOTNET_STRING_ENUM_BUFFER1,
    dotnet_string_enum_buffer::DOTNET_STRING_ENUM_BUFFER2,
    dotnet_string_enum_with_description_buffer::{
        DOTNET_STRING_ENUM_WITH_DESCRIPTION_BUFFER1, DOTNET_STRING_ENUM_WITH_DESCRIPTION_BUFFER2,
    },
    get_output_options_for_filename_test, get_table_constants_for_filename_test,
    get_table_constants_for_object_like_buffer_test,
    get_table_constants_for_object_like_with_description_buffer_test,
    get_table_constants_for_simple_enum_buffer_test,
    get_table_constants_for_simple_enum_with_description_buffer_test,
    get_table_constants_for_string_enum_buffer_test,
    get_table_constants_for_string_enum_with_description_buffer_test,
    rust_simple_enum_buffer::RUST_SIMPLE_ENUM_BUFFER1,
    rust_simple_enum_buffer::RUST_SIMPLE_ENUM_BUFFER2,
    rust_simple_enum_with_description_buffer::RUST_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER1,
    rust_simple_enum_with_description_buffer::RUST_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER2,
    rust_string_enum_buffer::RUST_STRING_ENUM_BUFFER1,
    rust_string_enum_buffer::RUST_STRING_ENUM_BUFFER2,
    rust_string_enum_with_description_buffer::RUST_STRING_ENUM_WITH_DESCRIPTION_BUFFER1,
    rust_string_enum_with_description_buffer::RUST_STRING_ENUM_WITH_DESCRIPTION_BUFFER2,
    typescript_object_like_enum_buffer::{
        TYPESCRIPT_OBJECT_LIKE_ENUM_BUFFER1, TYPESCRIPT_OBJECT_LIKE_ENUM_BUFFER2,
    },
    typescript_object_like_enum_with_description_buffer::{
        TYPESCRIPT_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER1,
        TYPESCRIPT_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER2,
    },
    typescript_simple_enum_buffer::TYPESCRIPT_SIMPLE_ENUM_BUFFER1,
    typescript_simple_enum_buffer::TYPESCRIPT_SIMPLE_ENUM_BUFFER2,
    typescript_simple_enum_with_description_buffer::TYPESCRIPT_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER1,
    typescript_simple_enum_with_description_buffer::TYPESCRIPT_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER2,
    typescript_string_enum_buffer::TYPESCRIPT_STRING_ENUM_BUFFER1,
    typescript_string_enum_buffer::TYPESCRIPT_STRING_ENUM_BUFFER2,
    typescript_string_enum_with_description_buffer::TYPESCRIPT_STRING_ENUM_WITH_DESCRIPTION_BUFFER1,
    typescript_string_enum_with_description_buffer::TYPESCRIPT_STRING_ENUM_WITH_DESCRIPTION_BUFFER2,
};
mod get_write_configurations_util;

fn do_filename_test(lang: Language, expected: &str) {
    // arrange
    let table_constants = get_table_constants_for_filename_test();
    let output_options = get_output_options_for_filename_test(lang);

    // act
    let configurations = get_write_configurations(&table_constants, &output_options);
    let first = configurations.first();
    let actual = &first.unwrap().filename;

    // assert
    assert_eq!(actual, expected);
}
#[test]
pub fn dotnet_filename() {
    do_filename_test(Language::Dotnet, "TestEnum.cs");
}

#[test]
pub fn rust_filename() {
    do_filename_test(Language::Rust, "test_enum.rs");
}

#[test]
pub fn typescript_filename() {
    do_filename_test(Language::Typescript, "TestEnum.ts");
}

fn do_buffer_assertion(
    table_constants: &[TableConstant],
    output_options: &OutputOptions,
    expecteds: &[&str],
) {
    // act
    let configurations = get_write_configurations(table_constants, output_options);
    let first = configurations.first();
    let actual = &first.unwrap().buffer;

    println!(
        "{:?}\r\n\r\n\r\n{:?}",
        [actual],
        [expecteds.first().unwrap()]
    );
    // assert
    assert!(expecteds.iter().any(|expected| expected == actual));
}

fn do_simple_enum_buffer_test(lang: Language, expecteds: &[&str]) {
    // arrange
    let table_constants = get_table_constants_for_simple_enum_buffer_test();
    let output_options = get_output_options_for_filename_test(lang);

    do_buffer_assertion(&table_constants, &output_options, expecteds);
}
#[test]
pub fn dotnet_simple_enum_buffer() {
    do_simple_enum_buffer_test(
        Language::Dotnet,
        &[DOTNET_SIMPLE_ENUM_BUFFER1, DOTNET_SIMPLE_ENUM_BUFFER2],
    );
}

#[test]
pub fn rust_simple_enum_buffer() {
    do_simple_enum_buffer_test(
        Language::Rust,
        &[RUST_SIMPLE_ENUM_BUFFER1, RUST_SIMPLE_ENUM_BUFFER2],
    );
}

#[test]
pub fn typescript_simple_enum_buffer() {
    do_simple_enum_buffer_test(
        Language::Typescript,
        &[
            TYPESCRIPT_SIMPLE_ENUM_BUFFER1,
            TYPESCRIPT_SIMPLE_ENUM_BUFFER2,
        ],
    );
}

fn do_simple_enum_with_description_buffer_test(lang: Language, expecteds: &[&str]) {
    // arrange
    let table_constants = get_table_constants_for_simple_enum_with_description_buffer_test();
    let output_options = get_output_options_for_filename_test(lang);

    do_buffer_assertion(&table_constants, &output_options, expecteds);
}

#[test]
pub fn dotnet_simple_enum_with_description_buffer() {
    do_simple_enum_with_description_buffer_test(
        Language::Dotnet,
        &[
            DOTNET_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER1,
            DOTNET_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}

#[test]
pub fn rust_simple_enum_with_description_buffer() {
    do_simple_enum_with_description_buffer_test(
        Language::Rust,
        &[
            RUST_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER1,
            RUST_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}

#[test]
pub fn typescript_simple_enum_with_description_buffer() {
    do_simple_enum_with_description_buffer_test(
        Language::Typescript,
        &[
            TYPESCRIPT_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER1,
            TYPESCRIPT_SIMPLE_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}

fn do_string_enum_buffer_test(lang: Language, expecteds: &[&str]) {
    // arrange
    let table_constants = get_table_constants_for_string_enum_buffer_test();
    let output_options = get_output_options_for_filename_test(lang);

    do_buffer_assertion(&table_constants, &output_options, expecteds);
}

#[test]
pub fn dotnet_string_enum_buffer() {
    do_string_enum_buffer_test(
        Language::Dotnet,
        &[DOTNET_STRING_ENUM_BUFFER1, DOTNET_STRING_ENUM_BUFFER2],
    )
}

#[test]
pub fn rust_string_enum_buffer() {
    do_string_enum_buffer_test(
        Language::Rust,
        &[RUST_STRING_ENUM_BUFFER1, RUST_STRING_ENUM_BUFFER2],
    )
}

#[test]
pub fn typescript_string_enum_buffer() {
    do_string_enum_buffer_test(
        Language::Typescript,
        &[
            TYPESCRIPT_STRING_ENUM_BUFFER1,
            TYPESCRIPT_STRING_ENUM_BUFFER2,
        ],
    )
}

fn do_string_enum_with_description_buffer_test(lang: Language, expecteds: &[&str]) {
    // arrange
    let table_constants = get_table_constants_for_string_enum_with_description_buffer_test();
    let output_options = get_output_options_for_filename_test(lang);

    do_buffer_assertion(&table_constants, &output_options, expecteds);
}

#[test]
pub fn dotnet_string_enum_with_description_buffer() {
    do_string_enum_with_description_buffer_test(
        Language::Dotnet,
        &[
            DOTNET_STRING_ENUM_WITH_DESCRIPTION_BUFFER1,
            DOTNET_STRING_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}

#[test]
pub fn rust_string_enum_with_description_buffer() {
    do_string_enum_with_description_buffer_test(
        Language::Rust,
        &[
            RUST_STRING_ENUM_WITH_DESCRIPTION_BUFFER1,
            RUST_STRING_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}

#[test]
pub fn typescript_string_enum_with_description_buffer() {
    do_string_enum_with_description_buffer_test(
        Language::Typescript,
        &[
            TYPESCRIPT_STRING_ENUM_WITH_DESCRIPTION_BUFFER1,
            TYPESCRIPT_STRING_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}

fn do_object_like_buffer_test(lang: Language, expecteds: &[&str]) {
    // arrange
    let table_constants = get_table_constants_for_object_like_buffer_test();
    let output_options = get_output_options_for_filename_test(lang);

    do_buffer_assertion(&table_constants, &output_options, expecteds);
}

#[test]
pub fn dotnet_object_like_enum_buffer() {
    do_object_like_buffer_test(
        Language::Dotnet,
        &[
            DOTNET_OBJECT_LIKE_ENUM_BUFFER1,
            DOTNET_OBJECT_LIKE_ENUM_BUFFER2,
        ],
    )
}

#[test]
pub fn typescript_object_like_enum_buffer() {
    do_object_like_buffer_test(
        Language::Typescript,
        &[
            TYPESCRIPT_OBJECT_LIKE_ENUM_BUFFER1,
            TYPESCRIPT_OBJECT_LIKE_ENUM_BUFFER2,
        ],
    )
}

fn do_object_like_with_description_buffer_test(lang: Language, expecteds: &[&str]) {
    // arrange
    let table_constants = get_table_constants_for_object_like_with_description_buffer_test();
    let output_options = get_output_options_for_filename_test(lang);

    do_buffer_assertion(&table_constants, &output_options, expecteds);
}

#[test]
pub fn dotnet_object_like_enum_with_description_buffer() {
    do_object_like_with_description_buffer_test(
        Language::Dotnet,
        &[
            DOTNET_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER1,
            DOTNET_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}

#[test]
pub fn typescript_object_like_enum_with_description_buffer() {
    do_object_like_with_description_buffer_test(
        Language::Typescript,
        &[
            TYPESCRIPT_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER1,
            TYPESCRIPT_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER2,
        ],
    )
}
