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
    "app.welcome"             => true,

    // [dev]
    "dev.pagetop_static_dir"  => "",

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
/// Tipos para las secciones globales [`[app]`](App), [`[dev]`](Dev), [`[log]`](Log) y
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
    /// Si no está definido o no es válido, [`LangId`](crate::locale::LangId) determinará el idioma
    /// efectivo para el renderizado en este orden: primero intentará usar el establecido mediante
    /// [`Contextual::with_langid()`](crate::core::component::Contextual::with_langid); si no se ha
    /// definido explícitamente, probará el indicado en la cabecera `Accept-Language` del navegador;
    /// y, si ninguno aplica, se empleará el idioma de respaldo ("en-US").
    pub language: String,
    /// Banner ASCII mostrado al inicio: *"Off"* (desactivado), *"Slant"*, *"Small"*, *"Speed"* o
    /// *"Starwars"*.
    pub startup_banner: String,
    /// Activa la página de bienvenida de PageTop.
    ///
    /// Si está activada, se instala la extensión [`Welcome`](crate::base::extension::Welcome), que
    /// ofrece una página de bienvenida predefinida en `"/"` y también en `"/lang/{lang}"`, para
    /// mostrar el contenido en el idioma `{lang}`, siempre que esté soportado.
    pub welcome: bool,
    /// Modo de ejecución, dado por la variable de entorno `PAGETOP_RUN_MODE`, o *"default"* si no
    /// está definido.
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Sección `[dev]` de la configuración. Forma parte de [`Settings`].
pub struct Dev {
    /// Directorio desde el que servir los archivos estáticos de PageTop.
    ///
    /// Por defecto, los archivos se integran en el binario de la aplicación. Si aquí se indica una
    /// ruta válida, ya sea absoluta o relativa al directorio del proyecto o del binario en
    /// ejecución, se servirán desde el sistema de ficheros en su lugar. Esto es especialmente útil
    /// en desarrollo, ya que evita recompilar el proyecto por cambios en estos archivos.
    ///
    /// Si la cadena está vacía, se ignora este ajuste.
    pub pagetop_static_dir: String,
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
    /// Muestra los mensajes de traza en el terminal (*"Stdout"*) o los vuelca en archivos con
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
    /// Duración de la cookie de sesión en segundos (p. ej., `604_800` para una semana).
    ///
    /// El valor `0` indica que la cookie permanecerá activa hasta que se cierre el navegador.
    pub session_lifetime: i64,
}
