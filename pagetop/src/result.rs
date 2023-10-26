//! Handling safe results.

use crate::locale::L10n;
use crate::trace;

pub struct TraceErr<T> {
    message: String,
    fallback: T,
}

impl<T> TraceErr<T> {
    pub fn trace(trace: L10n, fallback: T) -> Self {
        let message = trace.message();
        trace::trace!(message);
        TraceErr { message, fallback }
    }

    pub fn debug(trace: L10n, fallback: T) -> Self {
        let message = trace.message();
        trace::debug!(message);
        TraceErr { message, fallback }
    }

    pub fn info(trace: L10n, fallback: T) -> Self {
        let message = trace.message();
        trace::info!(message);
        TraceErr { message, fallback }
    }

    pub fn warn(trace: L10n, fallback: T) -> Self {
        let message = trace.message();
        trace::warn!(message);
        TraceErr { message, fallback }
    }

    pub fn error(trace: L10n, fallback: T) -> Self {
        let message = trace.message();
        trace::error!(message);
        TraceErr { message, fallback }
    }

    // TraceErr GETTERS.

    pub fn message(self) -> String {
        self.message
    }

    pub fn fallback(self) -> T {
        self.fallback
    }
}

pub enum SafeResult<T> {
    Ok(T),
    Err(TraceErr<T>),
}

impl<T> SafeResult<T> {
    #[inline]
    pub fn unwrap_or_error<F, E>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(TraceErr<T>) -> E,
    {
        match self {
            SafeResult::Ok(r) => Ok(r),
            SafeResult::Err(e) => Err(f(e)),
        }
    }

    #[inline]
    pub fn unwrap_or_fallback(self) -> T {
        match self {
            SafeResult::Ok(r) => r,
            SafeResult::Err(e) => e.fallback(),
        }
    }
}
