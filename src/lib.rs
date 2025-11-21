//! Replacements for [`Result::unwrap()`] and [`Option::unwrap()`] whose overall function
//! is the same, but with clearer names and better formatting.
//!
//! Each function in this library still panics on [`Err`] or [`None`], but their names document
//! the intent of the panic: for example, is it a situation that should be impossible, or
//! is it code whose proper error handling has not been written yet?
//!
//! Additionally, when an error is formatted into a panic message, it is formatted using
//! [`Display`][core::fmt::Display] and [`Error::source()`] instead of [`Debug`], so that errors
//! can be viewed in their intended human-readable form.
//!
//! This library is `no_std` compatible.

#![no_std]
#![forbid(unsafe_code)]
#![deny(
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc
)]
#![warn(unreachable_pub)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]

use core::error::Error;

mod panics;

/// Alternatives to [`Result::unwrap()`].
pub trait ResultUnwrapExt<T, E> {
    /// When `self` is [`Ok`], teturns the contained value.
    /// If `self` is [`Err`] instead, panics with a message indicating that an error case which
    /// should not have been reached was reached, and which includes the complete error message
    /// and source chain of the error value.
    ///
    /// Use this like [`unreachable!`]:
    /// when you believe that the error case cannot occur.
    ///
    /// # Example
    ///
    /// This can be used, for example, when using a fallible constructor with a constant:
    ///
    /// ```rust
    /// use descriptive_unwrap::ResultUnwrapExt as _;
    /// use core::num::NonZeroU8;
    ///
    /// let ten = NonZeroU8::new(10).err_is_unreachable();
    /// ```
    fn err_is_unreachable(self) -> T;

    /// Use this like [`todo!`]:
    /// the code is incomplete and error handling should be added.
    fn err_is_todo(self) -> T;

    #[doc(hidden)]
    #[expect(private_interfaces)]
    fn _this_trait_is_sealed_and_you_cannot_add_implementations_of_it() -> Sealed;
}

impl<T, E: Error> ResultUnwrapExt<T, E> for Result<T, E> {
    #[inline(always)]
    #[track_caller]
    fn err_is_unreachable(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => panics::panic_with_error_value("unreachable error case reached", &error),
        }
    }

    #[inline(always)]
    #[track_caller]
    fn err_is_todo(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => {
                panics::panic_with_error_value("handling this error is not yet implemented", &error)
            }
        }
    }

    #[doc(hidden)]
    #[expect(private_interfaces)]
    fn _this_trait_is_sealed_and_you_cannot_add_implementations_of_it() -> Sealed {
        Sealed
    }
}

/// This type cannot be constructed outside this crate,
/// and functions to seal the crate’s traits (makes them impossible to implement from outside).
struct Sealed;
