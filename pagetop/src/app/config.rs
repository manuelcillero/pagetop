//! Ajustes globales de la configuración.

use crate::config;
use crate::predefined_settings;
use crate::LazyStatic;

use serde::Deserialize;

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
    /// Valor predefinido: *"0"*
    pub db_port: u16,
    /// Valor predefinido: *"5"*
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
    /// Valor predefinido: *"8088"*
    pub bind_port: u16,
}

/// Declara e inicializa los ajustes globales para la estructura [`Settings`].
///
/// ```
/// use pagetop::prelude::*;
///
/// fn demo() {
///     println!("App name: {}", &SETTINGS.app.name);
///     println!("App description: {}", &SETTINGS.app.description);
///     println!("Value of PAGETOP_RUN_MODE: {}", &SETTINGS.app.run_mode);
/// }
/// ```
pub static SETTINGS: LazyStatic<Settings> = LazyStatic::new(|| {
    config::try_into::<Settings>(predefined_settings!(
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
        "database.db_port"       => "0",
        "database.max_pool_size" => "5",

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
        "server.bind_port"       => "8088"
    ))
});
