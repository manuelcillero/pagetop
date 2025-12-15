//! Gestión de trazas y registro de eventos de la aplicación.
//!
//! PageTop recopila información de diagnóstico de la aplicación de forma estructurada y basada en
//! eventos.
//!
//! En los sistemas asíncronos, interpretar los mensajes de log tradicionales suele volverse
//! complicado. Las tareas individuales se multiplexan en el mismo hilo y los eventos y registros
//! asociados se entremezclan, lo que dificulta seguir la secuencia lógica.
//!
//! PageTop usa [`tracing`](https://docs.rs/tracing) para registrar eventos estructurados y con
//! información adicional sobre la *temporalidad* y la *causalidad*. A diferencia de un mensaje de
//! log, un *span* (intervalo) tiene un momento de inicio y de fin, puede entrar y salir del flujo
//! de ejecución y puede existir dentro de un árbol anidado de *spans* similares. Además, estos
//! *spans* son estructurados, con la capacidad de registrar tipos de datos y mensajes de texto.

use crate::global;
use crate::global::{LogFormat, LogRolling};

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
pub(crate) static TRACING: LazyLock<WorkerGuard> = LazyLock::new(|| {
    if !global::SETTINGS.log.enabled || cfg!(test) || cfg!(feature = "testing") {
        // Tracing desactivado, se instala un subscriber nulo.
        tracing::subscriber::set_global_default(tracing::subscriber::NoSubscriber::default())
            .expect("Failed to install global NoSubscriber (tracing disabled)");
        let (_, guard) = tracing_appender::non_blocking(std::io::sink());
        return guard;
    }

    let env_filter = EnvFilter::try_new(&global::SETTINGS.log.tracing)
        .unwrap_or_else(|_| EnvFilter::new("Info"));

    let rolling = global::SETTINGS.log.rolling;

    let (non_blocking, guard) = match rolling {
        LogRolling::Stdout => tracing_appender::non_blocking(std::io::stdout()),
        _ => tracing_appender::non_blocking({
            let path = &global::SETTINGS.log.path;
            let prefix = &global::SETTINGS.log.prefix;
            match rolling {
                LogRolling::Daily => tracing_appender::rolling::daily(path, prefix),
                LogRolling::Hourly => tracing_appender::rolling::hourly(path, prefix),
                LogRolling::Minutely => tracing_appender::rolling::minutely(path, prefix),
                LogRolling::Endless => tracing_appender::rolling::never(path, prefix),
                LogRolling::Stdout => unreachable!("Stdout rolling already handled above"),
            }
        }),
    };

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_ansi(matches!(rolling, LogRolling::Stdout));

    match global::SETTINGS.log.format {
        LogFormat::Json => subscriber.json().init(),
        LogFormat::Full => subscriber.init(),
        LogFormat::Compact => subscriber.compact().init(),
        LogFormat::Pretty => subscriber.pretty().init(),
    }

    guard
});
