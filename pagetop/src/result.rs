//! Handling safe results.

use crate::locale::L10n;
use crate::trace;

pub struct TraceErr<T> {
    message: String,
    fallback: T,
}

impl<T> TraceErr<T> {
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
    pub fn unwrap_or_fallback(self) -> T {
        match self {
            SafeResult::Ok(result) => result,
            SafeResult::Err(trace) => trace.fallback(),
        }
    }
}
