//! Tests of the exact text produced by a program panicking using the library functions.
//! This allows us to present a complete and accurate example in the documentation.

#[test]
fn tests() {
    trycmd::TestCases::new().case("../doc-example-parts/*.toml");
}
