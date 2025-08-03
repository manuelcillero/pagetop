//! Opciones de configuración globales.

use crate::include_config;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [app]
    "app.name"                => "PageTop App",
    "app.description"         => "Developed with the amazing PageTop framework.",
    "app.theme"               => "Basic",
    "app.language"            => "",
    "app.startup_banner"      => "Slant",

    // [dev]
    "dev.pagetop_project_dir" => "",

    // [log]
    "log.enabled"             => true,
    "log.tracing"             => "Info",
    "log.rolling"             => "Stdout",
    "log.path"                => "log",
    "log.prefix"              => "tracing.log",
    "log.format"              => "Full",

    // [server]
    "server.bind_address"     => "localhost",
    "server.bind_port"        => 8080,
    "server.session_lifetime" => 604_800,
]);

#[derive(Debug, Deserialize)]
/// Ajustes para las secciones globales [`[app]`](App), [`[dev]`](Dev), [`[log]`](Log) y
/// [`[server]`](Server) de [`SETTINGS`].
pub struct Settings {
    pub app: App,
    pub dev: Dev,
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
    /// Idioma por defecto para la aplicación.
    ///
    /// Si no se especifica un valor válido, normalmente se usará el idioma devuelto por la
    /// implementación de [`LangId`](crate::locale::LangId) para [`Context`](crate::html::Context),
    /// en el siguiente orden: primero, el idioma establecido explícitamente con
    /// [`Context::with_langid`](crate::html::Context::with_langid); si no se ha definido, se usará
    /// el indicado en la cabecera `Accept-Language` del navegador; y, si ninguno aplica, se
    /// empleará el idioma de respaldo ("en-US").
    pub language: String,
    /// Banner ASCII mostrado al inicio: *"Off"* (desactivado), *"Slant"*, *"Small"*, *"Speed"* o
    /// *"Starwars"*.
    pub startup_banner: String,
    /// Modo de ejecución, dado por la variable de entorno `PAGETOP_RUN_MODE`, o *"default"* si no
    /// está definido.
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[Dev]` de la configuración. Forma parte de [`Settings`].
pub struct Dev {
    /// Los archivos estáticos requeridos por `PageTop` se integran por defecto en el binario
    /// ejecutable. Sin embargo, durante el desarrollo puede resultar útil servirlos desde su propio
    /// directorio para evitar recompilar cada vez que se modifican. En ese caso, este ajuste debe
    /// indicar la ruta absoluta al directorio raíz del proyecto.
    pub pagetop_project_dir: String,
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
    /// Duración de la cookie de sesión en segundos (p.ej. `604_800` para una semana).
    ///
    /// El valor `0` indica que la cookie permanecerá activa hasta que se cierre el navegador.
    pub session_lifetime: i64,
}
