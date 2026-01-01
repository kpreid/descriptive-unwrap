//! Tests of the panic messages produced by `descriptive-unwrap` library functions.

use std::error::Error;
use std::fmt;
use std::panic::{catch_unwind, UnwindSafe};

use descriptive_unwrap::ResultUnwrapExt as _;

/// [`Error`] type for use in tests.
#[derive(Debug)]
struct ErrorWithSource {
    message: &'static str,
    source: Option<Box<ErrorWithSource>>,
}

impl ErrorWithSource {
    pub fn new(message: &'static str) -> Self {
        Self {
            message,
            source: None,
        }
    }

    pub fn wrap(self, message: &'static str) -> Self {
        Self {
            message,
            source: Some(Box::new(self)),
        }
    }
}

impl fmt::Display for ErrorWithSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.message)
    }
}

impl Error for ErrorWithSource {
    #[expect(clippy::match_as_ref, reason = "prefer this over `as`")]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source {
            Some(ref err) => Some(err), // coerces to dyn
            None => None,
        }
    }
}

fn catch_string(f: impl FnOnce() + UnwindSafe) -> String {
    *catch_unwind(f)
        .unwrap_err()
        .downcast::<String>()
        .expect("payload not a String")
}

#[test]
fn unreachable_with_chain() {
    let error = ErrorWithSource::new("foo").wrap("bar").wrap("baz");
    assert_eq!(
        catch_string(|| Err(error).err_is_unreachable()),
        "unreachable error case reached: baz\n    • bar\n    • foo"
    );
}

#[test]
fn todo_with_chain() {
    let error = ErrorWithSource::new("foo").wrap("bar").wrap("baz");
    assert_eq!(
        catch_string(|| Err(error).err_is_todo()),
        "handling this error is not yet implemented: baz\n    • bar\n    • foo"
    );
}
