//! Opciones de configuración globales.

use crate::include_config;

use serde::Deserialize;

mod lang_negotiation;
pub use lang_negotiation::LangNegotiation;

mod startup_banner;
pub use startup_banner::StartupBanner;

mod timezone_order;
pub use timezone_order::TimezoneOrder;

mod log_rolling;
pub use log_rolling::LogRolling;

mod log_format;
pub use log_format::LogFormat;

// **< SETTINGS >***********************************************************************************

include_config!(SETTINGS: Settings => [
    // [app]
    "app.name"               => "PageTop App",
    "app.theme"              => "Basic",
    "app.lang_negotiation"   => "Full",
    "app.timezone"           => "UTC",
    "app.timezone_regions"   => "",
    "app.timezone_order"     => "Nearest",
    "app.timezone_per_user"  => true,
    "app.startup_banner"     => "Slant",

    // [dev]
    "dev.pagetop_static_dir" => "",

    // [log]
    "log.enabled"            => true,
    "log.tracing"            => "Info",
    "log.rolling"            => "Stdout",
    "log.path"               => "log",
    "log.prefix"             => "tracing.log",
    "log.format"             => "Full",

    // [server]
    "server.bind_address"    => "localhost",
    "server.bind_port"       => 8080,
]);

// **< Settings >***********************************************************************************

/// Ajustes para las secciones globales [`App`], [`Dev`], [`Log`] y [`Server`] de [`SETTINGS`].
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app: App,
    pub dev: Dev,
    pub log: Log,
    pub server: Server,
}

/// Sección **`[app]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct App {
    /// Nombre de la aplicación.
    pub name: String,
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
    /// Zona horaria predeterminada de la aplicación (p. ej. *"UTC"* o *"Europe/Madrid"*).
    ///
    /// Se usa como zona horaria efectiva para las peticiones de usuarios anónimos o sin zona
    /// horaria propia, y para todos si [`timezone_per_user`] es *false*. Ver [`Timezone`] y
    /// [`CurrentUser::timezone()`].
    ///
    /// Si es `None` o no contiene un valor válido, se aplica `UTC`.
    ///
    /// [`timezone_per_user`]: Self::timezone_per_user
    /// [`Timezone`]: crate::datetime::Timezone
    /// [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
    pub timezone: Option<String>,
    /// Regiones de zonas horarias que se ofrecen para elegir, separadas por comas (p. ej.
    /// *"Europe, America"*); da igual escribirlas en mayúsculas o minúsculas.
    ///
    /// Limita las zonas horarias de [`Timezone::supported_by_region()`] a las de esas regiones (ver
    /// [`TzRegion`]): *"Africa"*, *"America"*, *"Antarctica"*, *"Arctic"*, *"Asia"*, *"Atlantic"*,
    /// *"Australia"*, *"Europe"*, *"Indian"* y *"Pacific"*. Los nombres desconocidos se ignoran con
    /// un aviso al arrancar. El grupo *"Etc"*, sólo con `Etc/UTC`, se ofrece siempre, se indique o
    /// no; por eso *"Etc"* como único valor deja `Etc/UTC` como la única zona horaria disponible.
    /// Si está vacío, o ninguna de las indicadas es válida, se ofrecen todas.
    ///
    /// Es el valor por defecto de [`form::SelectTimezone`], que puede cambiarse para un componente
    /// concreto con `with_regions()`.
    ///
    /// Restringir las regiones no cambia las zonas horarias que los usuarios ya tengan asignadas:
    /// se les siguen aplicando ([`CurrentUser::timezone()`]) aunque queden fuera, y
    /// [`form::SelectTimezone`] las sigue mostrando como zona horaria actual para que volver a
    /// guardar el formulario no las descarte. Sólo dejan de ofrecerse para nuevas elecciones; quien
    /// valide el valor recibido contra [`Timezone::supported_by_region()`] debe aceptar también,
    /// sin cambios, el que ya estuviera guardado.
    ///
    /// [`Timezone::supported_by_region()`]: crate::datetime::Timezone::supported_by_region
    /// [`TzRegion`]: crate::datetime::TzRegion
    /// [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
    /// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
    pub timezone_regions: String,
    /// Orden de las regiones de zonas horarias que se ofrecen para elegir: *"Nearest"*,
    /// *"Alphabetical"* o *"Listed"*.
    ///
    /// Ver [`TimezoneOrder`] para los criterios disponibles. Es el valor por defecto de
    /// [`form::SelectTimezone`], que puede cambiarse para un componente concreto con
    /// `with_order()`.
    ///
    /// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
    pub timezone_order: TimezoneOrder,
    /// Si cada usuario puede tener su propia zona horaria (*true*) o se usa siempre la de la
    /// aplicación (*false*).
    ///
    /// Con *false*, [`CurrentUser::timezone()`] devuelve siempre la zona horaria de la aplicación
    /// ([`timezone`](Self::timezone)), aunque el usuario tenga otra guardada, y las extensiones que
    /// gestionan usuarios no deben ofrecer elegirla; la zona guardada se conserva por si se vuelve
    /// a activar. Es el equivalente de [`LangNegotiation::ConfigOnly`] para el idioma.
    ///
    /// [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
    pub timezone_per_user: bool,
    /// Banner ASCII mostrado al inicio: *"Off"* (desactivado), *"Slant"*, *"Small"*, *"Speed"* o
    /// *"Starwars"*.
    pub startup_banner: StartupBanner,
    /// Modo de ejecución, dado por la variable de entorno `PAGETOP_RUN_MODE`, o *"default"* si no
    /// está definido.
    pub run_mode: String,
}

/// Sección **`[dev]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct Dev {
    /// Directorio raíz de `static/` para servir los archivos estáticos propios de PageTop.
    ///
    /// Si se indica una ruta válida, absoluta o relativa al directorio del proyecto o del binario
    /// en ejecución, los archivos estáticos se servirán desde disco. Útil para poder modificar los
    /// archivos estáticos mientras la aplicación está en ejecución, sin necesidad de recompilar.
    ///
    /// Si la cadena está vacía, se ignora este ajuste.
    pub pagetop_static_dir: String,
}

/// Sección **`[log]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct Log {
    /// Gestión de trazas y registro de eventos activada (*true*) o desactivada (*false*).
    pub enabled: bool,
    /// Opciones, o combinación de opciones separadas por comas, para filtrar las trazas: *"Error"*,
    /// *"Warn"*, *"Info"*, *"Debug"* o *"Trace"*.
    /// Ejemplo: *"Error,tower_http=Debug,axum::rejection=trace"*.
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

/// Sección **`[server]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct Server {
    /// Dirección de enlace para el servidor web.
    pub bind_address: String,
    /// Puerto de escucha del servidor web.
    pub bind_port: u16,
}
