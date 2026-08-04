# `descriptive-unwrap`

`descriptive-unwrap` is a Rust library which provides replacements for [`Result::unwrap()`] and [`Option::unwrap()`] whose overall function is the same, but with meaningful names and [`Error`] formatting, for the benefit of developers and users.

**For developers:**
Each function in this library panics on `Err` or `None`,
just like `unwrap()` and `expect()` do, but their names document the intent of the panic:
is it a situation that should be impossible, or is it code whose proper error handling has not been written yet?

**For application users:**
When an error is formatted into a panic message, it is formatted using
[`Display`] and [`Error::source()`] instead of [`Debug`], so that errors
can be viewed in their intended human-readable form.
This means that, in the event of an unexpected failure, users are presented with a
cleaner error report more amenable to troubleshooting.

`descriptive-unwrap` is `no_std` compatible; it does not depend on `std` or `alloc`.

[`Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`Error::source()`]: https://doc.rust-lang.org/std/error/trait.Error.html#method.source
[`Option::unwrap()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
[`Result::unwrap()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
