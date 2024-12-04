//! Application tracing and event logging.
//!
//! `PageTop` collects application diagnostic information in a structured and event-based manner.
//!
//! In asynchronous systems, interpreting traditional log messages often becomes complicated.
//! Individual tasks are multiplexed to the same thread, and associated events and log messages get
//! intermingled, making it difficult to follow the logical sequence.
//!
//! `PageTop` uses [`tracing`](https://docs.rs/tracing) to allow **applications** and **modules** to
//! log structured events with added information about *temporality* and *causality*. Unlike a log
//! message, a span has a start and end time, can enter and exit the execution flow, and can exist
//! within a nested tree of similar spans. Additionally, these spans are *structured*, with the
//! ability to record data types and text messages.

use crate::global;

pub use tracing::{debug, error, info, trace, warn};
pub use tracing::{debug_span, error_span, info_span, trace_span, warn_span};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

use std::sync::LazyLock;

/// Application tracing and event logging.
///
/// To increase performance, a dedicated thread uses a non-blocking writer system that acts
/// periodically instead of sending each trace or event instantly. If the program terminates
/// abruptly (e.g., due to a panic! or a `std::process::exit`), some traces or events might not be
/// sent.
///
/// Since traces or events logged shortly before an application crash are often important for
/// diagnosing the cause of the failure, `Lazy<WorkerGuard>` ensures that all stored logs are sent
/// before terminating execution.

#[rustfmt::skip]
pub(crate) static TRACING: LazyLock<WorkerGuard> = LazyLock::new(|| {
    let env_filter = EnvFilter::try_new(&global::SETTINGS.log.tracing)
        .unwrap_or_else(|_| EnvFilter::new("Info"));

    let rolling = global::SETTINGS.log.rolling.to_lowercase();

    let (non_blocking, guard) = match rolling.as_str() {
        "stdout" => tracing_appender::non_blocking(std::io::stdout()),
        _ => tracing_appender::non_blocking({
            let path = &global::SETTINGS.log.path;
            let prefix = &global::SETTINGS.log.prefix;
            match rolling.as_str() {
                "daily"    => tracing_appender::rolling::daily(path, prefix),
                "hourly"   => tracing_appender::rolling::hourly(path, prefix),
                "minutely" => tracing_appender::rolling::minutely(path, prefix),
                "endless"  => tracing_appender::rolling::never(path, prefix),
                _ => {
                    println!(
                        "Rolling value \"{}\" not valid. Using \"daily\". Check the settings file.",
                        global::SETTINGS.log.rolling,
                    );
                    tracing_appender::rolling::daily(path, prefix)
                }
            }
        }),
    };

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_ansi(rolling.as_str() == "stdout");

    match global::SETTINGS.log.format.to_lowercase().as_str() {
        "json"    => subscriber.json().init(),
        "full"    => subscriber.init(),
        "compact" => subscriber.compact().init(),
        "pretty"  => subscriber.pretty().init(),
        _ => {
            println!(
                "Tracing format \"{}\" not valid. Using \"Full\". Check the settings file.",
                global::SETTINGS.log.format,
            );
            subscriber.init();
        }
    }

    guard
});
