//! Replacements for [`Result::unwrap()`] and [`Option::unwrap()`] whose overall function
//! is the same, but with meaningful names and [`Error`] formatting.
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
#![allow(
    clippy::missing_panics_doc,
    reason = "panicking is the main point of all docs"
)]
#![allow(
    clippy::needless_doctest_main,
    reason = "examples need to be include!()able code for panic-example-test"
)]

use core::error::Error;

mod panics;

/// Alternatives to [`Result::unwrap()`].
pub trait ResultUnwrapExt<T, E> {
    /// When `self` is [`Ok`], returns the contained value.
    /// If `self` is [`Err`] instead, panics with a message indicating that an error case which
    /// should not have been reached was reached, and which includes the complete error message
    /// and source chain of the error value.
    ///
    /// Use this like [`unreachable!`]:
    /// when you believe that the error case cannot occur.
    ///
    /// # Example
    ///
    /// ```rust
    /// use descriptive_unwrap::ResultUnwrapExt as _;
    /// use std::net::IpAddr;
    ///
    /// let constant_addr: IpAddr = "192.168.0.1".parse().err_is_unreachable();
    /// ```
    ///
    /// If instead there is an error, it will panic with a message using the `Display` formatting
    /// of the error value:
    ///
    /// ```rust,should_panic
    #[doc = include_str!("../doc-example-parts/err_is_unreachable.rs")]
    /// ```
    ///
    /// ```text
    #[doc = include_str!("../doc-example-parts/err_is_unreachable.stderr")]
    /// ```
    fn err_is_unreachable(self) -> T;

    /// When `self` is [`Ok`], returns the contained value.
    /// If `self` is [`Err`] instead, panics with a message indicating that error handling is
    /// not yet implemented.
    ///
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

/// Alternatives to [`Option::unwrap()`].
pub trait OptionUnwrapExt<T> {
    /// When `self` is [`Some`], returns the contained value.
    /// If `self` is [`None`] instead, panics with a message indicating that an error case which
    /// should not have been reached was reached.
    ///
    /// Use this like [`unreachable!`]:
    /// when you believe that the error case cannot occur.
    ///
    /// # Example
    ///
    /// This can be used, for example, when using [`Option::take()`] in a situation where you know
    /// the [`Option`] will never actually be [`None`]:
    ///
    /// ```rust
    /// use descriptive_unwrap::OptionUnwrapExt as _;
    ///
    /// let mut option = Some(10);
    /// let value = option.take().none_is_unreachable();
    /// ```
    fn none_is_unreachable(self) -> T;

    /// Use this like [`todo!`]:
    /// the code is incomplete and missing value handling should be added.
    fn none_is_todo(self) -> T;

    #[doc(hidden)]
    #[expect(private_interfaces)]
    fn _this_trait_is_sealed_and_you_cannot_add_implementations_of_it() -> Sealed;
}

impl<T> OptionUnwrapExt<T> for Option<T> {
    #[inline(always)]
    #[track_caller]
    fn none_is_unreachable(self) -> T {
        match self {
            Some(value) => value,
            None => panic!("value missing that should always be present"),
        }
    }

    #[inline(always)]
    #[track_caller]
    fn none_is_todo(self) -> T {
        match self {
            Some(value) => value,
            None => {
                panic!("handling missing value is not yet implemented")
            }
        }
    }

    #[doc(hidden)]
    #[expect(private_interfaces)]
    fn _this_trait_is_sealed_and_you_cannot_add_implementations_of_it() -> Sealed {
        Sealed
    }
}

/// When `option` is [`Some`], returns the contained value.
/// If `option` is [`None`] instead, panics with a message indicating that an error case which
/// should not have been reached was reached.
///
/// Use this like [`unreachable!`]:
/// when you believe that the error case cannot occur.
///
/// This is identical to the extension trait method [`OptionUnwrapExt::none_is_unreachable()`]
/// except that it is a `const fn`, and is not a method (so it cannot cause a method name conflict).
///
/// # Example
///
/// This can be used, for example, when using [`Option::take()`] in a situation where you know
/// the [`Option`] will never actually be [`None`]:
///
/// ```rust
/// use descriptive_unwrap::none_is_unreachable;
///
/// let mut option = Some(10);
/// let value = none_is_unreachable(option.take());
/// ```
#[inline(always)]
#[track_caller]
pub const fn none_is_unreachable<T>(option: Option<T>) -> T {
    match &option {
        // This unwrap() will never panic.
        // It is a workaround for lack of feature(const_precise_live_drops);
        // we borrow Option::unwrap()’s standard library magic powers.
        Some(_) => option.unwrap(),
        None => panic!("value missing that should always be present"),
    }
}

/// Use this like [`todo!`]:
/// the code is incomplete and missing value handling should be added.
#[inline(always)]
#[track_caller]
pub const fn none_is_todo<T>(option: Option<T>) -> T {
    match &option {
        // This unwrap() will never panic.
        // It is a workaround for lack of feature(const_precise_live_drops);
        // we borrow Option::unwrap()’s standard library magic powers.
        Some(_) => option.unwrap(),
        None => {
            panic!("handling missing value is not yet implemented")
        }
    }
}

/// This type cannot be constructed outside this crate,
/// and functions to seal the crate’s traits (makes them impossible to implement from outside).
struct Sealed;
