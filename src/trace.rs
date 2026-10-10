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

use std::fmt;
use std::sync::{LazyLock, Mutex, PoisonError};

// Avisos generados antes de iniciar las trazas. La configuración se lee antes, porque las trazas
// dependen de la sección `[log]`, así que sus avisos se guardan aquí y se emiten al iniciarlas.
static DEFERRED_WARNINGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

// Guarda el aviso de un valor no válido para la opción `key` y devuelve `default`, el valor que se
// usa en su lugar. Lo usan las opciones enumeradas de la configuración al leerse.
pub(crate) fn warn_invalid_setting<T: fmt::Debug>(key: &str, raw: &str, default: T) -> T {
    DEFERRED_WARNINGS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push(format!(
            "Invalid value \"{raw}\" for {key}, using \"{default:?}\". Check settings"
        ));
    default
}

/// Trazado y registro de eventos de la aplicación.
///
/// La escritura no es bloqueante: el código que genera una traza o un evento la deja en una cola y
/// sigue, y un hilo dedicado la escribe en el destino configurado en cuanto la recibe.
///
/// El [`WorkerGuard`] vuelca las trazas pendientes al destruirse, pero un `static` no se destruye
/// nunca, así que al terminar el programa pueden perderse las últimas que quedaran en la cola.
pub(crate) static TRACING: LazyLock<WorkerGuard> = LazyLock::new(|| {
    let enabled = global::SETTINGS.log.enabled && !cfg!(test) && !cfg!(feature = "testing");
    // Al leer `SETTINGS` en la línea anterior ya se han guardado los avisos de la configuración.
    let mut warnings = std::mem::take(
        &mut *DEFERRED_WARNINGS
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    );
    if !enabled {
        // Tracing desactivado, se instala un subscriber nulo.
        tracing::subscriber::set_global_default(tracing::subscriber::NoSubscriber::default())
            .expect("Failed to install global NoSubscriber (tracing disabled)");
        let (_, guard) = tracing_appender::non_blocking(std::io::sink());
        // Los avisos pendientes salen por la salida estándar para que no se pierdan.
        for message in warnings {
            println!("\n{message}");
        }
        return guard;
    }

    let filter = &global::SETTINGS.log.tracing;
    let env_filter = EnvFilter::try_new(filter).unwrap_or_else(|_| {
        warnings.push(format!(
            "Invalid value \"{filter}\" for [log].tracing, using \"Info\". Check settings"
        ));
        EnvFilter::new("Info")
    });

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

    for message in warnings {
        warn!(target: "pagetop::global", "{message}");
    }
    guard
});
