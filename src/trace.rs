//! Gestión de trazas y registro de eventos de la aplicación.
//!
//! `PageTop` recopila información de diagnóstico de la aplicación de forma estructurada y basada en
//! eventos.
//!
//! En los sistemas asíncronos, interpretar los mensajes de log tradicionales suele volverse
//! complicado. Las tareas individuales se multiplexan en el mismo hilo y los eventos y registros
//! asociados se entremezclan, lo que dificulta seguir la secuencia lógica.
//!
//! `PageTop` usa [`tracing`](https://docs.rs/tracing) para registrar eventos estructurados y con
//! información adicional sobre la *temporalidad* y la *causalidad*. A diferencia de un mensaje de
//! log, un *span* (intervalo) tiene un momento de inicio y de fin, puede entrar y salir del flujo
//! de ejecución y puede existir dentro de un árbol anidado de *spans* similares. Además, estos
//! *spans* son estructurados, con la capacidad de registrar tipos de datos y mensajes de texto.

use crate::global;

pub use tracing::{debug, error, info, trace, warn};
pub use tracing::{debug_span, error_span, info_span, trace_span, warn_span};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

use std::sync::LazyLock;

/// Trazado y registro de eventos de la aplicación.
///
/// Para aumentar el rendimiento, un hilo dedicado utiliza un sistema de escritura no bloqueante que
/// actúa de forma periódica en lugar de enviar cada traza o evento al instante. Si el programa
/// termina abruptamente (por ejemplo, debido a un `panic!` o a una llamada a `std::process::exit`),
/// es posible que algunas trazas o eventos no se envíen.
///
/// Dado que las trazas o eventos registrados poco antes de un fallo suelen ser cruciales para
/// diagnosticar la causa, `Lazy<WorkerGuard>` garantiza que todos los registros almacenados se
/// envíen antes de finalizar la ejecución.

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
