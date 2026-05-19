//! Binary used by the `stderr_format` test target to as a way to examine the full formatted
//! panic output including the Rust parts.

#![allow(invalid_from_utf8)]

use descriptive_unwrap::OptionUnwrapExt as _;

fn main() {
    let args_strings: Vec<String> = std::env::args().collect();
    let args_strs: Vec<&str> = args_strings.iter().map(String::as_str).collect();

    match args_strs[1..] {
        ["Option::none_is_unreachable"] => none_is_unreachable_example::run(),
        ["Option::none_is_todo"] => None.none_is_todo(),
        ["Result::err_is_unreachable"] => err_is_unreachable_example::run(),
        ["Result::err_is_todo"] => err_is_todo_example::run(),
        _ => panic!("unrecognized subcommand: {args_strs:?}"),
    }
}

#[allow(unused_variables)]
mod err_is_unreachable_example {
    include!("../../doc-example-parts/err_is_unreachable.rs");
    pub fn run() {
        main()
    }
}

#[allow(unused_variables)]
mod err_is_todo_example {
    include!("../../doc-example-parts/err_is_todo.rs");
    pub fn run() {
        main()
    }
}

#[allow(unused_variables)]
mod none_is_unreachable_example {
    include!("../../doc-example-parts/none_is_unreachable.rs");
    pub fn run() {
        main()
    }
}
