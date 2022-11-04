use crate::config;
use crate::predefined_settings;
use crate::LazyStatic;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Ajustes globales para las secciones [`[app]`](App), [`[log]`](Log), [`[database]`](Database),
/// [`[webserver]`](Webserver) y [`[dev]`](Dev) reservadas para PageTop ([`SETTINGS`]).
pub struct Settings {
    pub app: App,
    pub log: Log,
    pub database: Database,
    pub webserver: Webserver,
    pub dev: Dev,
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
/// Sección `[webserver]` de los ajustes globales.
///
/// Ver [`Settings`].
pub struct Webserver {
    /// Valor predefinido: *"localhost"*
    pub bind_address: String,
    /// Valor predefinido: *"8088"*
    pub bind_port: u16,
}

#[derive(Debug, Deserialize)]
/// Sección `[dev]` de los ajustes globales.
///
/// Ver [`Settings`].
pub struct Dev {
    /// Valor predefinido: *""*
    pub static_files: String,
}

/// Declara los ajustes globales para la estructura [`Settings`].
///
/// Ver [`Cómo usar los ajustes globales de la configuración`](index.html#cómo-usar-los-ajustes-globales-de-la-configuración).
pub static SETTINGS: LazyStatic<Settings> = LazyStatic::new(|| {
    config::try_into::<Settings>(predefined_settings!(
        // [app]
        "app.name"               => "PageTop Application",
        "app.description"        => "Developed with the amazing PageTop framework.",
        "app.theme"              => "Bootsier",
        "app.language"           => "en-US",
        "app.direction"          => "ltr",
        "app.startup_banner"     => "Slant",

        // [log]
        "log.tracing"            => "Info",
        "log.rolling"            => "Stdout",
        "log.path"               => "log",
        "log.prefix"             => "tracing.log",
        "log.format"             => "Full",

        // [database]
        "database.db_type"       => "",
        "database.db_name"       => "",
        "database.db_user"       => "",
        "database.db_pass"       => "",
        "database.db_host"       => "localhost",
        "database.db_port"       => "0",
        "database.max_pool_size" => "5",

        // [webserver]
        "webserver.bind_address" => "localhost",
        "webserver.bind_port"    => "8088",

        // [dev]
        "dev.static_files"       => ""
    ))
});
