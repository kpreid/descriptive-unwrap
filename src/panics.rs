use core::error::Error;
use core::fmt;

#[inline(never)]
#[track_caller]
pub(crate) fn panic_with_error_value(explanation: &'static str, error: &dyn Error) -> ! {
    panic!("{explanation}: {chain}", chain = ErrorChain(error))
}

struct ErrorChain<'a>(&'a (dyn Error + 'a));

impl fmt::Display for ErrorChain<'_> {
    #[inline(never)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut error = self.0;

        // Write the error's own message.
        write!(fmt, "{error}")?;

        // Write the source, the source’s source, etc., all with a prefix.
        while let Some(source) = error.source() {
            error = source;
            write!(fmt, "\n    • {error}")?;
        }

        Ok(())
    }
}
