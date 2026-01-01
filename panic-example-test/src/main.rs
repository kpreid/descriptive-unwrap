//! Binary used by the `stderr_format` test target to as a way to examine the full formatted
//! panic output including the Rust parts.

use std::error::Error;
use std::fmt;

use descriptive_unwrap::{OptionUnwrapExt as _, ResultUnwrapExt as _};

fn main() {
    let args_strings: Vec<String> = std::env::args().collect();
    let args_strs: Vec<&str> = args_strings.iter().map(String::as_str).collect();

    let a_result: Result<(), ParseError> = "nan".parse::<i32>().map(drop).map_err(ParseError);

    match args_strs[1..] {
        ["Option::none_is_unreachable"] => None.none_is_unreachable(),
        ["Option::none_is_todo"] => None.none_is_todo(),
        ["Result::err_is_unreachable"] => err_is_unreachable_example::run(),
        ["Result::err_is_todo"] => a_result.err_is_todo(),
        _ => panic!("unrecognized subcommand: {args_strs:?}"),
    }
}

#[derive(std::fmt::Debug)]
struct ParseError(std::num::ParseIntError);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("failed to parse input")
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

#[allow(unused_variables)]
mod err_is_unreachable_example {
    include!("../../doc-example-parts/err_is_unreachable.rs");
    pub fn run() {
        main()
    }
}
