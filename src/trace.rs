//! Application tracing and event logging.
//!
//! PageTop recopila la información de diagnóstico de la aplicación de manera estructurada y basada
//! en eventos.
//!
//! En sistemas asíncronos, interpretar los mensajes de registro tradicionales (*log*) a menudo
//! resulta complicado. Las tareas individuales se multiplexan para el mismo subproceso y los
//! eventos y mensajes de registro asociados se entremezclan, dificultando el seguimiento de la
//! secuencia lógica.
//!
//! PageTop usa [`tracing`](https://docs.rs/tracing) para permitir a las **aplicaciones** y los
//! **módulos** registrar eventos estructurados con información añadida sobre *temporalidad* y
//! *causalidad*. A diferencia de un mensaje de registro, un intervalo (*span*) tiene una hora de
//! inicio y de finalización, puede entrar y salir del flujo de la ejecución y puede existir dentro
//! de un árbol anidado de intervalos similares. Además, estos intervalos están *estructurados*, con
//! capacidad para grabar tipos de datos y mensajes de texto.

use crate::{config, LazyStatic};

pub use tracing::{debug, error, info, trace, warn};
pub use tracing::{debug_span, error_span, info_span, trace_span, warn_span};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

/// Registro de trazas y eventos de la aplicación.
///
/// Para aumentar el rendimiento, un subproceso dedicado utiliza un sistema de escritura sin bloqueo
/// (*non-blocking writer*) que actúa periódicamente en vez de enviar cada traza o evento al
/// instante. Si el programa termina abruptamente (por ejemplo, por un panic! o un
/// std::process::exit), es posible que algunas trazas o eventos no se envíen.
///
/// Puesto que las trazas o eventos registrados poco antes de la caída de una aplicación suelen ser
/// importantes para diagnosticar la causa del fallo, con `Lazy<WorkerGuard>` se garantiza que todos
/// los registros almacenados se enviarán antes de terminar la ejecución.

#[rustfmt::skip]
pub(crate) static TRACING: LazyStatic<WorkerGuard> = LazyStatic::new(|| {
    let env_filter = EnvFilter::try_new(&config::SETTINGS.log.tracing)
        .unwrap_or_else(|_| EnvFilter::new("Info"));

    let rolling = config::SETTINGS.log.rolling.to_lowercase();

    let (non_blocking, guard) = match rolling.as_str() {
        "stdout" => tracing_appender::non_blocking(std::io::stdout()),
        _ => tracing_appender::non_blocking({
            let path = &config::SETTINGS.log.path;
            let prefix = &config::SETTINGS.log.prefix;
            match rolling.as_str() {
                "daily"    => tracing_appender::rolling::daily(path, prefix),
                "hourly"   => tracing_appender::rolling::hourly(path, prefix),
                "minutely" => tracing_appender::rolling::minutely(path, prefix),
                "endless"  => tracing_appender::rolling::never(path, prefix),
                _ => {
                    println!(
                        "Rolling value \"{}\" not valid. Using \"daily\". Check the settings file.",
                        config::SETTINGS.log.rolling,
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

    match config::SETTINGS.log.format.to_lowercase().as_str() {
        "json"    => subscriber.json().init(),
        "full"    => subscriber.init(),
        "compact" => subscriber.compact().init(),
        "pretty"  => subscriber.pretty().init(),
        _ => {
            println!(
                "Tracing format \"{}\" not valid. Using \"Full\". Check the settings file.",
                config::SETTINGS.log.format,
            );
            subscriber.init();
        }
    }

    guard
});
