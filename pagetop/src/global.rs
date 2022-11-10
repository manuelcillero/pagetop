use crate::{pub_config, trace, LazyStatic};

use serde::Deserialize;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

use unic_langid::LanguageIdentifier;

// CONFIGURACIÓN ***********************************************************************************

#[derive(Debug, Deserialize)]
/// Ajustes globales para las secciones reservadas [`[app]`](App), [`[database]`](Database),
/// [`[dev]`](Dev), [`[log]`](Log) y [`[server]`](Server) (ver [`SETTINGS`]).
pub struct Settings {
    pub app: App,
    pub database: Database,
    pub dev: Dev,
    pub log: Log,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
/// Sección `[app]` de los ajustes globales.
///
/// Ver [`Settings`].
pub struct App {
    /// Valor predefinido: *"PageTop Application"*
    pub name: String,
    /// Valor predefinido: *"Developed with the amazing PageTop framework."*
    pub description: String,
    /// Valor predefinido: *"Bootsier"*
    pub theme: String,
    /// Valor predefinido: *"en-US"*
    pub language: String,
    /// Valor predefinido: *"ltr"*
    pub direction: String,
    /// Valor predefinido: *"Slant"*
    pub startup_banner: String,
    /// Valor predefinido: según variable de entorno PAGETOP_RUN_MODE, o *"default"* si no lo está
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[database]` de los ajustes globales.
///
/// Ver [`Settings`].
pub struct Database {
    /// Valor predefinido: *""*
    pub db_type: String,
    /// Valor predefinido: *""*
    pub db_name: String,
    /// Valor predefinido: *""*
    pub db_user: String,
    /// Valor predefinido: *""*
    pub db_pass: String,
    /// Valor predefinido: *"localhost"*
    pub db_host: String,
    /// Valor predefinido: *0*
    pub db_port: u16,
    /// Valor predefinido: *5*
    pub max_pool_size: u32,
}

#[derive(Debug, Deserialize)]
/// Sección `[dev]` de los ajustes globales.
///
/// Ver [`Settings`].
pub struct Dev {
    /// Valor predefinido: *""*
    pub static_files: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[log]` de los ajustes globales.
///
/// Ver [`Settings`].
pub struct Log {
    /// Valor predefinido: *"Info"*
    pub tracing: String,
    /// Valor predefinido: *"Stdout"*
    pub rolling: String,
    /// Valor predefinido: *"log"*
    pub path: String,
    /// Valor predefinido: *"tracing.log"*
    pub prefix: String,
    /// Valor predefinido: *"Full"*
    pub format: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[server]` de los ajustes globales.
///
/// Ver [`Settings`].
pub struct Server {
    /// Valor predefinido: *"localhost"*
    pub bind_address: String,
    /// Valor predefinido: *8088*
    pub bind_port: u16,
}

pub_config!(SETTINGS: Settings,
    // [app]
    "app.name"               => "PageTop Application",
    "app.description"        => "Developed with the amazing PageTop framework.",
    "app.theme"              => "Bootsier",
    "app.language"           => "en-US",
    "app.direction"          => "ltr",
    "app.startup_banner"     => "Slant",

    // [database]
    "database.db_type"       => "",
    "database.db_name"       => "",
    "database.db_user"       => "",
    "database.db_pass"       => "",
    "database.db_host"       => "localhost",
    "database.db_port"       => 0,
    "database.max_pool_size" => 5,

    // [dev]
    "dev.static_files"       => "",

    // [log]
    "log.tracing"            => "Info",
    "log.rolling"            => "Stdout",
    "log.path"               => "log",
    "log.prefix"             => "tracing.log",
    "log.format"             => "Full",

    // [server]
    "server.bind_address"    => "localhost",
    "server.bind_port"       => 8088,
);

// REGISTRO DE TRAZAS Y EVENTOS ********************************************************************

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
    let env_filter =
        EnvFilter::try_new(&SETTINGS.log.tracing).unwrap_or_else(|_| EnvFilter::new("Info"));

    let rolling = SETTINGS.log.rolling.to_lowercase();
    let (non_blocking, guard) = match rolling.as_str() {
        "stdout" => tracing_appender::non_blocking(std::io::stdout()),
        _ => tracing_appender::non_blocking({
            let path = &SETTINGS.log.path;
            let prefix = &SETTINGS.log.prefix;
            match rolling.as_str() {
                "daily"    => tracing_appender::rolling::daily(path, prefix),
                "hourly"   => tracing_appender::rolling::hourly(path, prefix),
                "minutely" => tracing_appender::rolling::minutely(path, prefix),
                "endless"  => tracing_appender::rolling::never(path, prefix),
                _ => {
                    println!(
                        "Rolling value \"{}\" not valid. Using \"daily\". Check the settings file.",
                        SETTINGS.log.rolling,
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
    match SETTINGS.log.format.to_lowercase().as_str() {
        "json"    => subscriber.json().init(),
        "full"    => subscriber.init(),
        "compact" => subscriber.compact().init(),
        "pretty"  => subscriber.pretty().init(),
        _ => {
            println!(
                "Tracing format \"{}\" not valid. Using \"Full\". Check the settings file.",
                SETTINGS.log.format,
            );
            subscriber.init();
        }
    }

    guard
});

// LOCALIZACIÓN ************************************************************************************

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// de la aplicación, obtenido de `SETTINGS.app.language`.
pub static LANGID: LazyStatic<LanguageIdentifier> =
    LazyStatic::new(|| match SETTINGS.app.language.parse() {
        Ok(language) => language,
        Err(_) => {
            trace::warn!(
                "{}, {} \"{}\"! {}, {}",
                "Failed to parse language",
                "unrecognized Unicode Language Identifier",
                SETTINGS.app.language,
                "Using \"en-US\"",
                "check the settings file",
            );
            "en-US".parse().unwrap()
        }
    });
