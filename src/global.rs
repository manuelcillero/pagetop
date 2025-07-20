//! Opciones de configuración globales.

use crate::include_config;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [app]
    "app.name"            => "Sample",
    "app.description"     => "Developed with the amazing PageTop framework.",
    "app.theme"           => "Basic",
    "app.language"        => "en-US",
    "app.startup_banner"  => "Slant",

    // [log]
    "log.enabled"         => true,
    "log.tracing"         => "Info",
    "log.rolling"         => "Stdout",
    "log.path"            => "log",
    "log.prefix"          => "tracing.log",
    "log.format"          => "Full",

    // [server]
    "server.bind_address" => "localhost",
    "server.bind_port"    => 8080,
]);

#[derive(Debug, Deserialize)]
/// Ajustes para las secciones globales [`[app]`](App), [`[log]`](Log) y [`[server]`](Server) de
/// [`SETTINGS`].
pub struct Settings {
    pub app: App,
    pub log: Log,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
/// Sección `[app]` de la configuración. Forma parte de [`Settings`].
pub struct App {
    /// Nombre de la aplicación.
    pub name: String,
    /// Breve descripción de la aplicación.
    pub description: String,
    /// Tema predeterminado.
    pub theme: String,
    /// Idioma predeterminado (localización).
    pub language: String,
    /// Banner ASCII mostrado al inicio: *"Off"* (desactivado), *"Slant"*, *"Small"*, *"Speed"* o
    /// *"Starwars"*.
    pub startup_banner: String,
    /// Modo de ejecución, dado por la variable de entorno `PAGETOP_RUN_MODE`, o *"default"* si no
    /// está definido.
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[log]` de la configuración. Forma parte de [`Settings`].
pub struct Log {
    /// Gestión de trazas y registro de eventos activado (`true`) o desactivado (`false`).
    pub enabled: bool,
    /// Opciones, o combinación de opciones separadas por comas, para filtrar las trazas: *"Error"*,
    /// *"Warn"*, *"Info"*, *"Debug"* o *"Trace"*.
    /// Ejemplo: "Error,actix_server::builder=Info,tracing_actix_web=Debug".
    pub tracing: String,
    /// Muestra los mensajes de traza en el terminal (*"Stdout"*) o las registra en archivos con
    /// rotación: *"Daily"*, *"Hourly"*, *"Minutely"* o *"Endless"*.
    pub rolling: String,
    /// Directorio para los archivos de traza (si `rolling` ≠ *"Stdout"*).
    pub path: String,
    /// Prefijo para los archivos de traza (si `rolling` ≠ *"Stdout"*).
    pub prefix: String,
    /// Formato de salida de las trazas. Opciones: *"Full"*, *"Compact"*, *"Pretty"* o *"Json"*.
    pub format: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[server]` de la configuración. Forma parte de [`Settings`].
pub struct Server {
    /// Dirección de enlace para el servidor web.
    pub bind_address: String,
    /// Puerto de escucha del servidor web.
    pub bind_port: u16,
}
