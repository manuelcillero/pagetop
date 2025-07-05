//! Opciones de configuración globales.

use crate::include_config;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [app]
    "app.name"            => "Sample",
    "app.description"     => "Developed with the amazing PageTop framework.",
    "app.startup_banner"  => "Slant",

    // [server]
    "server.bind_address" => "localhost",
    "server.bind_port"    => 8080,
]);

#[derive(Debug, Deserialize)]
/// Ajustes de configuración para las secciones globales [`[app]`](App) y [`[server]`](Server).
/// Consulta [`SETTINGS`] para los valores por defecto.
pub struct Settings {
    pub app: App,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
/// Sección `[app]` de la configuración.
///
/// Forma parte de [`Settings`].
pub struct App {
    /// Nombre de la aplicación.
    /// Valor por defecto: *"Sample"*.
    pub name: String,
    /// Breve descripción de la aplicación.
    /// Valor por defecto: *"Developed with the amazing PageTop framework."*.
    pub description: String,
    /// ASCII banner printed at startup: *"Off"*, *"Slant"*, *"Small"*, *"Speed"*, or *"Starwars"*.
    /// Default: *"Slant"*.
    pub startup_banner: String,
    /// Modo de ejecución.
    /// Valor por defecto: el definido por la variable de entorno
    /// `PAGETOP_RUN_MODE`, o *"default"* si no está establecida.
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[server]` de la configuración.
///
/// Forma parte de [`Settings`].
pub struct Server {
    /// Dirección de enlace para el servidor web.
    /// Valor por defecto: *"localhost"*.
    pub bind_address: String,
    /// Puerto de escucha del servidor web.
    /// Valor por defecto: *8088*.
    pub bind_port: u16,
}
