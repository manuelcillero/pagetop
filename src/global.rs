//! Opciones de configuración globales.

use crate::include_config;

use serde::Deserialize;

mod lang_negotiation;
pub use lang_negotiation::LangNegotiation;

mod startup_banner;
pub use startup_banner::StartupBanner;

mod log_rolling;
pub use log_rolling::LogRolling;

mod log_format;
pub use log_format::LogFormat;

// **< SETTINGS >***********************************************************************************

include_config!(SETTINGS: Settings => [
    // [app]
    "app.name"                => "PageTop App",
    "app.description"         => "Developed with the amazing PageTop framework.",
    "app.theme"               => "Basic",
    "app.lang_negotiation"    => "Full",
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

// **< Settings >***********************************************************************************

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
    /// Idioma predeterminado de la aplicación (p. ej., *"es-ES"* o *"en-US"*).
    ///
    /// Cuando tiene un valor validado por [`Locale`](crate::locale::Locale), se usa como candidato
    /// para resolver el idioma efectivo de cada petición según la estrategia definida en
    /// [`lang_negotiation`](Self::lang_negotiation) y aplicada por
    /// [`RequestLocale`](crate::locale::RequestLocale).
    ///
    /// Si es `None` o no contiene un valor válido, la negociación del idioma pasa a depender de
    /// otras fuentes como la cabecera `Accept-Language` de la petición o, en último término, del
    /// idioma de respaldo configurado en el sistema.
    pub language: Option<String>,
    /// Estrategia para resolver el idioma usado en la petición: *"Full"*, *"NoQuery"* o
    /// *"ConfigOnly"*.
    ///
    /// Define las fuentes que intervienen en la negociación del idioma para el renderizado de los
    /// documentos y la generación de URLs. Ver [`LangNegotiation`] para los modos disponibles.
    pub lang_negotiation: LangNegotiation,
    /// Banner ASCII mostrado al inicio: *"Off"* (desactivado), *"Slant"*, *"Small"*, *"Speed"* o
    /// *"Starwars"*.
    pub startup_banner: StartupBanner,
    /// Activa la página de bienvenida de PageTop.
    ///
    /// Si está activada, se instala la extensión [`Welcome`](crate::base::extension::Welcome), que
    /// ofrece una página de bienvenida predefinida en `"/"`.
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
    /// Gestión de trazas y registro de eventos activada (*true*) o desactivada (*false*).
    pub enabled: bool,
    /// Opciones, o combinación de opciones separadas por comas, para filtrar las trazas: *"Error"*,
    /// *"Warn"*, *"Info"*, *"Debug"* o *"Trace"*.
    /// Ejemplo: *"Error,actix_server::builder=Info,tracing_actix_web=Debug"*.
    pub tracing: String,
    /// Muestra los mensajes de traza en el terminal (*"Stdout"*) o los vuelca en archivos con
    /// rotación: *"Daily"*, *"Hourly"*, *"Minutely"* o *"Endless"*.
    pub rolling: LogRolling,
    /// Directorio para los archivos de traza (si [`rolling`](Self::rolling) ≠ *"Stdout"*).
    pub path: String,
    /// Prefijo para los archivos de traza (si [`rolling`](Self::rolling) ≠ *"Stdout"*).
    pub prefix: String,
    /// Formato de salida de las trazas. Opciones: *"Full"*, *"Compact"*, *"Pretty"* o *"Json"*.
    pub format: LogFormat,
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
