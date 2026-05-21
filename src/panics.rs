use core::error::Error;
use core::fmt;

#[inline(never)]
#[track_caller]
pub(crate) fn panic_with_error_value(explanation: &'static str, error: &dyn Error) -> ! {
    panic!("{explanation}:{chain}", chain = ErrorChainList(error))
}

struct ErrorChainList<'a>(&'a (dyn Error + 'a));

impl fmt::Display for ErrorChainList<'_> {
    #[inline(never)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut error = self.0;

        // Write the source, the source’s source, etc., all with a prefix.
        loop {
            write!(fmt, "\n    ↳ {error}")?;
            if let Some(source) = error.source() {
                error = source;
            } else {
                break;
            }
        }

        Ok(())
    }
}
