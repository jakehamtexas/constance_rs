#![allow(unused_imports)]
use crate::constancerc::get_runtime_configuration::abstraction::i_cli_args::ICliArgs;
use gives_no_args::GivesNoArg;
use gives_one_arg::{GivesOneArg, FIRST_ARG};
use gives_two_args::GivesTwoArgs;

mod gives_no_args;
mod gives_one_arg;
mod gives_two_args;

#[test]
pub fn get_path_one_argument_supplied_should_return_arg() {
    // arrange
    let cli_args = GivesOneArg {};
    let expected = FIRST_ARG.to_string();

    // act
    let actual = cli_args.get_path();

    // assert
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
pub fn get_path_no_args_supplied_should_panic() {
    // arrange
    let cli_args = GivesNoArg {};

    // act
    cli_args.get_path();
}

#[test]
pub fn get_path_two_arguments_supplied_should_return_first() {
    // arrange
    let cli_args = GivesTwoArgs {};
    let expected = FIRST_ARG.to_string();

    // act
    let actual = cli_args.get_path();

    // assert
    assert_eq!(actual, expected);
}
