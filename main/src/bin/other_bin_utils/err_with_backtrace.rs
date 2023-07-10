use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ErrWithBacktrace<T: Error> {
    err: T,
    backtrace: Backtrace,
}

impl<T: Error> Error for ErrWithBacktrace<T> {}

impl<T: Error> Display for ErrWithBacktrace<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, Stack trace:\n{}",
            self.err,
            self.backtrace.to_string()
        )
    }
}

impl<T: Error> Debug for ErrWithBacktrace<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub fn add_stack_trace<T, E>(res: Result<T, E>) -> Result<T, ErrWithBacktrace<E>>
where
    E: Error,
{
    res.map_err(|err| {
        let backtrace = Backtrace::capture();

        ErrWithBacktrace { backtrace, err }
    })
}
