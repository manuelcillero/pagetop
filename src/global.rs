//! Ajustes de configuración globales de PageTop.

use crate::core::extension::all::StartupError;
use crate::web::http::{HeaderName, Uri};
use crate::{config, settings};

use serde::de::Error;
use serde::{Deserialize, Deserializer};

use std::sync::LazyLock;

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

/// Ajustes de las secciones `[app]`, `[dev]`, `[log]`, `[server]` y `[upload]`.
///
/// Se cargan al arrancar la aplicación, antes que los ajustes de las extensiones.
pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| Settings {
    app: config::load(),
    dev: config::load(),
    log: config::load(),
    server: config::load(),
    upload: config::load(),
});

// Comprueba que la configuración se puede leer y que los ajustes globales son válidos.
// La aplicación lo llama al arrancar, antes del primer acceso a `SETTINGS`, que lanzaría `panic!`.
pub(crate) fn check_settings() -> Result<(), StartupError> {
    config::check_sources().map_err(|err| StartupError::ReadConfig(err.into()))?;
    check_section::<AppSettings>()?;
    check_section::<DevSettings>()?;
    check_section::<LogSettings>()?;
    check_section::<ServerSettings>()?;
    check_section::<UploadSettings>()
}

fn check_section<T: config::SettingsSection>() -> Result<(), StartupError> {
    config::check::<T>().map_err(|err| StartupError::InvalidSettings {
        section: T::SECTION,
        source: err.into(),
    })
}

// **< Settings >***********************************************************************************

/// Ajustes globales de [`SETTINGS`].
///
/// Secciones [`AppSettings`], [`DevSettings`], [`LogSettings`], [`ServerSettings`] y
/// [`UploadSettings`].
#[derive(Debug)]
pub struct Settings {
    pub app: AppSettings,
    pub dev: DevSettings,
    pub log: LogSettings,
    pub server: ServerSettings,
    pub upload: UploadSettings,
}

// Secciones de los ajustes globales, para reconocerlas en los archivos de configuración.
pub(crate) const SECTIONS: [&str; 5] = [
    <AppSettings as config::SettingsSection>::SECTION,
    <DevSettings as config::SettingsSection>::SECTION,
    <LogSettings as config::SettingsSection>::SECTION,
    <ServerSettings as config::SettingsSection>::SECTION,
    <UploadSettings as config::SettingsSection>::SECTION,
];

/// Sección **`[app]`** de la configuración. Forma parte de [`Settings`].
#[settings(section = "app")]
pub struct AppSettings {
    /// Nombre de la aplicación.
    #[default = "PageTop App"]
    pub name: String,
    /// Tema predeterminado: *"Basic"* o el de una extensión enlazada por la aplicación, sin
    /// distinguir mayúsculas y minúsculas. Si no está habilitado se usa *"Basic"*, con un aviso al
    /// arrancar. Ver [`default_theme()`](crate::core::theme::default_theme).
    #[default = "Basic"]
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
    #[default(LangNegotiation::Full)]
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
    /// Si está vacío se ofrecen todas, y también si ninguna de las indicadas es válida, con un
    /// aviso al arrancar. Se avisa igualmente si la región de [`timezone`] queda fuera de las
    /// ofrecidas.
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
    /// [`timezone`]: Self::timezone
    pub timezone_regions: String,
    /// Orden de las regiones de zonas horarias que se ofrecen para elegir: *"Nearest"*,
    /// *"Alphabetical"* o *"Listed"*.
    ///
    /// Ver [`TimezoneOrder`] para los criterios disponibles. Es el valor por defecto de
    /// [`form::SelectTimezone`], que puede cambiarse para un componente concreto con
    /// `with_order()`.
    ///
    /// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
    #[default(TimezoneOrder::Nearest)]
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
    #[default = true]
    pub timezone_per_user: bool,
    /// Banner ASCII mostrado al inicio: *"Off"* (desactivado), *"Slant"*, *"Small"*, *"Speed"* o
    /// *"Starwars"*.
    #[default(StartupBanner::Slant)]
    pub startup_banner: StartupBanner,
    /// Modo de ejecución, dado por la variable de entorno `PAGETOP_RUN_MODE`, o *"default"* si no
    /// está definida o está vacía. Con la *feature* `testing` activa es siempre *"test"*.
    pub run_mode: String,
}

/// Sección **`[dev]`** de la configuración. Forma parte de [`Settings`].
#[settings(section = "dev")]
pub struct DevSettings {
    /// Directorio raíz de `static/` para servir los archivos estáticos propios de PageTop.
    ///
    /// Si se indica una ruta válida, absoluta o relativa al directorio del proyecto (o, al ejecutar
    /// el binario, al directorio de trabajo actual), los archivos estáticos se servirán desde
    /// disco. Útil para poder modificar los archivos estáticos mientras la aplicación está en
    /// ejecución, sin necesidad de recompilar.
    ///
    /// Si la cadena está vacía, se ignora este ajuste.
    pub pagetop_static_dir: String,
}

/// Sección **`[log]`** de la configuración. Forma parte de [`Settings`].
#[settings(section = "log")]
pub struct LogSettings {
    /// Gestión de trazas y registro de eventos activada (*true*) o desactivada (*false*).
    #[default = true]
    pub enabled: bool,
    /// Opciones, o combinación de opciones separadas por comas, para filtrar las trazas: *"Error"*,
    /// *"Warn"*, *"Info"*, *"Debug"* o *"Trace"*.
    /// Ejemplo: *"Error,tower_http=Debug,axum::rejection=trace"*. Si no es válido se usa *"Info"*,
    /// con un aviso al arrancar.
    #[default = "Info"]
    pub tracing: String,
    /// Muestra los mensajes de traza en el terminal (*"Stdout"*) o los vuelca en archivos con
    /// rotación: *"Daily"*, *"Hourly"*, *"Minutely"* o *"Endless"*.
    #[default(LogRolling::Stdout)]
    pub rolling: LogRolling,
    /// Directorio para los archivos de traza (si [`rolling`](Self::rolling) ≠ *"Stdout"*).
    #[default = "log"]
    pub path: String,
    /// Prefijo para los archivos de traza (si [`rolling`](Self::rolling) ≠ *"Stdout"*).
    #[default = "tracing.log"]
    pub prefix: String,
    /// Formato de salida de las trazas. Opciones: *"Full"*, *"Compact"*, *"Pretty"* o *"Json"*.
    #[default(LogFormat::Full)]
    pub format: LogFormat,
}

/// Sección **`[server]`** de la configuración. Forma parte de [`Settings`].
#[settings(section = "server")]
pub struct ServerSettings {
    /// Dirección de enlace para el servidor web.
    #[default = "localhost"]
    pub bind_address: String,
    /// Puerto de escucha del servidor web.
    #[default = 8080]
    pub bind_port: u16,
    /// Tamaño máximo en bytes del cuerpo de una petición que se lee entero en memoria (por defecto
    /// 2 MiB, el mismo que aplica Axum). Con *0* no hay límite.
    ///
    /// Se aplica a los extractores que leen el cuerpo completo, como [`Form`], [`RawForm`] o
    /// [`Json`], y también a la suma de los campos de texto de un formulario con archivos recibido
    /// con [`Upload::receive()`]. Los envíos `multipart/form-data` completos, con sus archivos, los
    /// limita en cambio [`max_request_size`] de `[upload]`, porque los archivos se escriben en
    /// disco a medida que llegan. Una petición mayor se rechaza con `413 Payload Too Large`.
    ///
    /// [`Form`]: crate::web::Form
    /// [`RawForm`]: crate::web::RawForm
    /// [`Json`]: crate::response::Json
    /// [`Upload::receive()`]: crate::web::Upload::receive
    /// [`max_request_size`]: UploadSettings::max_request_size
    #[default = 2_097_152]
    pub max_body_size: usize,
    /// Cabecera con la dirección IP del cliente, cuando la aplicación está detrás de un *proxy*
    /// inverso o de un balanceador (p. ej. `"X-Forwarded-For"` o `"X-Real-IP"`). Vacía, la
    /// dirección es la de la conexión. Ver [`HttpRequest::client_ip()`].
    ///
    /// Si la cabecera tiene varias direcciones, vale la última, la que añade el *proxy* más cercano
    /// a la aplicación. Con varios *proxies* encadenados, esa última sería la del anterior: usa
    /// entonces una cabecera con una única dirección que escriba el primero, el que recibe la
    /// conexión del cliente.
    ///
    /// Configúrala sólo si todas las peticiones pasan por el *proxy* y éste escribe siempre la
    /// cabecera, sustituyendo o completando la que llegue: si no, cualquiera podría enviarla con la
    /// dirección que quisiera.
    ///
    /// Debe ser un nombre de cabecera HTTP válido; si no, la aplicación no arranca. Tampoco admite
    /// `Forwarded` (RFC 7239), cuyos valores (`for=...;proto=...`) no son sólo direcciones.
    ///
    /// [`HttpRequest::client_ip()`]: crate::web::HttpRequest::client_ip
    #[default = ""]
    #[serde(deserialize_with = "header_name")]
    pub client_ip_header: String,
    /// Otros orígenes desde los que un navegador puede enviar peticiones que cambian estado
    /// (`POST`, `PUT`, `PATCH`, `DELETE`...), p. ej. `["https://app.example.com"]`.
    ///
    /// PageTop rechaza con `403 Forbidden` esas peticiones cuando el navegador indica que llegan
    /// desde otro origen, también desde otro subdominio del mismo sitio, para proteger la
    /// aplicación frente a CSRF (*Cross-Site Request Forgery*). Esta lista sólo hace falta si otra
    /// aplicación propia, servida desde otro origen, envía formularios o peticiones `fetch` a ésta.
    /// Las peticiones que no hace un navegador (`curl`, otros servidores...) no se ven afectadas.
    ///
    /// Cada origen es `esquema://servidor[:puerto]`, sin ruta. Uno no válido impide arrancar.
    #[serde(deserialize_with = "origins")]
    pub trusted_origins: Vec<String>,
}

// Orígenes para `trusted_origins`, tal y como los envía un navegador en la cabecera `Origin`: en
// minúsculas, sin `/` final y sin el puerto por defecto del esquema. Un origen con ruta, consulta,
// fragmento o credenciales nunca coincidiría con esa cabecera, y quedaría sin efecto sin que nada
// lo indicase.
fn origins<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<String>, D::Error> {
    Vec::<String>::deserialize(deserializer)?
        .iter()
        .map(|raw| {
            let origin = raw.trim().trim_end_matches('/').to_ascii_lowercase();
            let Some(uri) = origin.parse::<Uri>().ok().filter(|uri| {
                matches!(uri.scheme_str(), Some("http" | "https"))
                    && uri.authority().is_some_and(|a| !a.as_str().contains('@'))
                    && uri.path_and_query().is_none_or(|pq| pq.as_str() == "/")
                    && !origin.contains('#')
            }) else {
                return Err(D::Error::custom(format!(
                    "invalid origin \"{raw}\", expected scheme://host[:port] (e.g. \
                     \"https://app.example.com\")"
                )));
            };
            match (uri.scheme_str(), uri.port_u16()) {
                (Some("http"), Some(80)) | (Some("https"), Some(443)) => Ok(origin
                    .rsplit_once(':')
                    .map_or(origin.clone(), |(o, _)| o.to_owned())),
                _ => Ok(origin),
            }
        })
        .collect()
}

// Nombre de cabecera para `client_ip_header`: vacío o un nombre de cabecera HTTP válido, sin
// espacios en los extremos. Un nombre que no lo fuera nunca encontraría la cabecera, y
// `HttpRequest::client_ip()` devolvería siempre `None` sin que nada lo indicase.
fn header_name<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let raw = String::deserialize(deserializer)?;
    let name = raw.trim();
    if name.eq_ignore_ascii_case("forwarded") {
        return Err(D::Error::custom(
            "the Forwarded header (RFC 7239) is not supported; use one with only addresses, such \
             as X-Forwarded-For or X-Real-IP",
        ));
    }
    if !name.is_empty() && HeaderName::from_bytes(name.as_bytes()).is_err() {
        return Err(D::Error::custom(format!(
            "invalid HTTP header name \"{name}\""
        )));
    }
    Ok(name.to_owned())
}

/// Sección **`[upload]`** de la configuración. Forma parte de [`Settings`].
///
/// Ajustes para guardar en el servidor los archivos subidos con [`Upload`](crate::web::Upload).
#[settings(section = "upload")]
pub struct UploadSettings {
    /// Directorio donde se guardan los archivos subidos, absoluto o relativo al directorio de
    /// trabajo. Se crea, junto con sus subdirectorios, al guardar el primer archivo.
    #[default = "uploads"]
    pub dir: String,
    /// Tamaño máximo en bytes de una petición `multipart/form-data` completa, con todos sus
    /// archivos y campos (por defecto 32 MiB). Con *0* no hay límite.
    ///
    /// Sustituye a [`max_body_size`](ServerSettings::max_body_size) de `[server]` en las peticiones
    /// con ese tipo de contenido. Para admitir varios archivos en un envío debe ser varias veces
    /// mayor que [`max_file_size`](Self::max_file_size). Una petición mayor se rechaza con
    /// `413 Payload Too Large`.
    #[default = 33_554_432]
    pub max_request_size: usize,
    /// Tamaño máximo en bytes de cada archivo (por defecto 8 MiB). Con *0* sólo limita
    /// [`max_request_size`](Self::max_request_size), que acota además la suma de todos los archivos
    /// de un mismo envío.
    ///
    /// Es el valor por defecto de [`Upload`](crate::web::Upload), que puede cambiarse para una
    /// subida concreta con `with_max_size()`.
    #[default = 8_388_608]
    pub max_file_size: u64,
    /// Número máximo de archivos de un mismo envío (por defecto 20). Con *0* no se limita.
    ///
    /// Lo aplica [`Upload::receive()`](crate::web::Upload::receive) para que un envío con cientos
    /// de archivos diminutos, que cabrían en `max_request_size`, no llene el disco de archivos.
    /// Puede cambiarse para una subida concreta con `with_max_files()`.
    #[default = 20]
    pub max_files: usize,
}

#[cfg(test)]
mod tests {
    use super::{AppSettings, DevSettings, LogSettings, ServerSettings, UploadSettings, config};

    // `reference.toml` documents every option; with unknown keys rejected, loading it proves that
    // it only documents keys that exist, with values of the right type.
    const REFERENCE: &str = include_str!("../config/reference.toml");

    #[test]
    fn reference_toml_documents_only_existing_keys() {
        config::load_from_str::<AppSettings>(REFERENCE).unwrap();
        config::load_from_str::<DevSettings>(REFERENCE).unwrap();
        config::load_from_str::<LogSettings>(REFERENCE).unwrap();
        config::load_from_str::<ServerSettings>(REFERENCE).unwrap();
        config::load_from_str::<UploadSettings>(REFERENCE).unwrap();
    }

    #[test]
    fn enum_values_are_case_insensitive() {
        let log = config::load_from_str::<LogSettings>("[log]\nrolling = \"daily\"\n").unwrap();
        assert_eq!(log.rolling, super::LogRolling::Daily);
    }

    #[test]
    fn invalid_enum_values_are_rejected() {
        let error = config::load_from_str::<LogSettings>("[log]\nrolling = \"Daly\"\n")
            .expect_err("invalid value accepted")
            .to_string();
        assert!(error.contains("Daly"));
        assert!(error.contains("Daily"));
    }

    #[test]
    fn client_ip_header_must_be_a_valid_header_name() {
        let load = |value: &str| {
            let toml = format!("[server]\nclient_ip_header = \"{value}\"\n");
            config::load_from_str::<ServerSettings>(&toml)
        };
        assert_eq!(load(" X-Real-IP ").unwrap().client_ip_header, "X-Real-IP");
        assert_eq!(load("").unwrap().client_ip_header, "");
        let error = load("X Real IP").expect_err("invalid name accepted");
        assert!(error.to_string().contains("X Real IP"), "{error}");
        // Its values are not plain addresses, so it would never give one.
        assert!(load("Forwarded").is_err());
    }

    #[test]
    fn trusted_origins_are_normalized_and_validated() {
        let load = |value: &str| {
            let toml = format!("[server]\ntrusted_origins = [\"{value}\"]\n");
            config::load_from_str::<ServerSettings>(&toml)
        };
        let origins = |value| load(value).unwrap().trusted_origins;
        // Browsers send the Origin header in lowercase and without a trailing slash.
        assert_eq!(
            origins(" HTTPS://App.Example.com/ "),
            ["https://app.example.com"]
        );
        assert_eq!(origins("http://localhost:3000"), ["http://localhost:3000"]);
        // Nor with the default port of the scheme, which they leave out.
        assert_eq!(
            origins("https://app.example.com:443"),
            ["https://app.example.com"]
        );
        assert_eq!(origins("http://[::1]:80"), ["http://[::1]"]);
        assert_eq!(
            origins("http://app.example.com:443"),
            ["http://app.example.com:443"]
        );
        // None of these could ever match an Origin header.
        for invalid in [
            "app.example.com",
            "ftp://app.example.com",
            "https://app.example.com/path",
            "https://app.example.com?x=1",
            "https://user@app.example.com",
            "https://app.example.com#top",
            "",
        ] {
            let error = load(invalid).expect_err(invalid);
            assert!(error.to_string().contains("invalid origin"), "{error}");
        }
    }
}
