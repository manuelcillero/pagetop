//! Opciones de configuración globales.

use crate::include_config;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [app]
    "app.name"                => "My App",
    "app.description"         => "Developed with the amazing PageTop framework.",
    "app.layout"              => "",
    "app.language"            => "en-US",
    "app.text_direction"      => "ltr",
    "app.startup_banner"      => "Slant",

    // [dev]
    "dev.pagetop_project_dir" => "",

    // [log]
    "log.tracing"             => "Info",
    "log.rolling"             => "Stdout",
    "log.path"                => "log",
    "log.prefix"              => "tracing.log",
    "log.format"              => "Full",

    // [server]
    "server.bind_address"     => "localhost",
    "server.bind_port"        => 8088,
    "server.session_lifetime" => 604_800,
]);

#[derive(Debug, Deserialize)]
/// Configuration settings for the global [`[app]`](App), [`[dev]`](Dev), [`[log]`](Log), and
/// [`[server]`](Server) sections (see [`SETTINGS`]).
pub struct Settings {
    pub app: App,
    pub dev: Dev,
    pub log: Log,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
/// Section `[app]` of the configuration settings.
///
/// See [`Settings`].
pub struct App {
    /// The name of the application.
    /// Default: *"My App"*.
    pub name: String,
    /// A brief description of the application.
    /// Default: *"Developed with the amazing PageTop framework."*.
    pub description: String,
    /// Default layout.
    /// Default: *""*.
    pub layout: String,
    /// Default language (localization).
    /// Default: *"en-US"*.
    pub language: String,
    /// Default text direction: *"ltr"* (left-to-right), *"rtl"* (right-to-left), or *"auto"*.
    /// Default: *"ltr"*.
    pub text_direction: String,
    /// ASCII banner printed at startup: *"Off"*, *"Slant"*, *"Small"*, *"Speed"*, or *"Starwars"*.
    /// Default: *"Slant"*.
    pub startup_banner: String,
    /// Default: according to the `PAGETOP_RUN_MODE` environment variable, or *"default"* if unset.
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Section `[dev]` of the configuration settings.
///
/// See [`Settings`].
pub struct Dev {
    /// Static files required by the application are integrated by default into the executable
    /// binary. However, during development, it can be useful to serve these files from their own
    /// directory to avoid recompilation every time they are modified. In this case, specify the
    /// full path to the project's root directory.
    /// Default: *""*.
    pub pagetop_project_dir: String,
}

#[derive(Debug, Deserialize)]
/// Section `[log]` of the configuration settings.
///
/// See [`Settings`].
pub struct Log {
    /// Filter, or a comma-separated combination of filters, for execution traces: *"Error"*,
    /// *"Warn"*, *"Info"*, *"Debug"*, or *"Trace"*.
    /// Example: "Error,actix_server::builder=Info,tracing_actix_web=Debug".
    /// Default: *"Info"*.
    pub tracing: String,
    /// Displays traces in the terminal (*"Stdout"*) or logs them in files with rotation: *"Daily"*,
    /// *"Hourly"*, *"Minutely"*, or *"Endless"*.
    /// Default: *"Stdout"*.
    pub rolling: String,
    /// Directory for trace files (if `rolling` != *"Stdout"*).
    /// Default: *"log"*.
    pub path: String,
    /// Prefix for trace files (if `rolling` != *"Stdout"*).
    /// Default: *"tracing.log"*.
    pub prefix: String,
    /// Trace output format. Options are *"Full"*, *"Compact"*, *"Pretty"*, or *"Json"*.
    /// Default: *"Full"*.
    pub format: String,
}

#[derive(Debug, Deserialize)]
/// Section `[server]` of the configuration settings.
///
/// See [`Settings`].
pub struct Server {
    /// Web server bind address.
    /// Default: *"localhost"*.
    pub bind_address: String,
    /// Web server bind port.
    /// Default: *8088*.
    pub bind_port: u16,
    /// Session cookie duration in seconds (0 means "until the browser is closed").
    /// Default: *604800* (7 days).
    pub session_lifetime: i64,
}
