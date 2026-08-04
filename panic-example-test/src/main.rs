//! Binary used by the `stderr_format` test target to as a way to examine the full formatted
//! panic output including the Rust parts.

#![allow(invalid_from_utf8)]

fn main() {
    let args_strings: Vec<String> = std::env::args().collect();
    let args_strs: Vec<&str> = args_strings.iter().map(String::as_str).collect();

    match args_strs[1..] {
        ["descriptive_unwrap::err_is_unreachable"] => generated::err_is_unreachable_fn(),
        ["descriptive_unwrap::err_is_todo"] => generated::err_is_todo_fn(),
        ["descriptive_unwrap::none_is_unreachable"] => generated::none_is_unreachable_fn(),
        ["descriptive_unwrap::none_is_todo"] => generated::none_is_todo_fn(),
        ["Option::none_is_unreachable"] => generated::none_is_unreachable_trait(),
        ["Option::none_is_todo"] => generated::none_is_todo_trait(),
        ["Result::err_is_unreachable"] => generated::err_is_unreachable_trait(),
        ["Result::err_is_todo"] => generated::err_is_todo_trait(),
        _ => panic!("unrecognized subcommand: {args_strs:?}"),
    }
}

mod generated {
    #![allow(unused_variables)]
    include!(concat!(env!("OUT_DIR"), "/doc-example-parts.rs"));
}
