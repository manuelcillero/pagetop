//! Carga las opciones de configuración de la aplicación.
//!
//! Estos ajustes se obtienen de archivos [TOML](https://toml.io) como pares `clave = valor` que se
//! deserializan en estructuras **fuertemente tipadas**, con valores por defecto para las claves que
//! no se definan.
//!
//! Separar la configuración del código permite que cada despliegue (*dev*, *staging*, *production*,
//! etc.) tenga su propia configuración sin recompilar. Los ajustes sólo se leen de archivos; las
//! variables de entorno no los sobrescriben, únicamente eligen qué archivos cargar (`CONFIG_DIR` y
//! `PAGETOP_RUN_MODE`, cuyo valor queda además en `app.run_mode`).
//!
//! # Orden de carga
//!
//! Si tu aplicación necesita archivos de configuración, crea un directorio `config` en la raíz del
//! proyecto, al mismo nivel que el archivo *Cargo.toml*. Puedes cambiar esta ubicación mediante la
//! variable de entorno `CONFIG_DIR` (vacía equivale a no definirla); si no apunta a un directorio
//! existente, se avisa al arrancar.
//!
//! Una ruta relativa, como la predeterminada, se resuelve respecto al directorio del proyecto si
//! está definida la variable de entorno `CARGO_MANIFEST_DIR` (*cargo* la define al ejecutar con
//! `cargo run` o `cargo test`) o, en su defecto, respecto al directorio de trabajo actual. Al
//! ejecutar el binario, lánzalo desde el directorio que contiene `config` o define `CONFIG_DIR` con
//! una ruta absoluta.
//!
//! PageTop carga en este orden, y siempre de forma opcional, los siguientes archivos TOML:
//!
//! 1. **config/common.toml**, para ajustes comunes a todos los entornos. Este enfoque simplifica el
//!    mantenimiento al centralizar los valores de configuración comunes.
//!
//! 2. **config/{rm}.toml**, donde `{rm}` es el valor de la variable de entorno `PAGETOP_RUN_MODE`:
//!
//!    * Si `PAGETOP_RUN_MODE` no está definida, o está vacía, se asume el valor `default`, y
//!      PageTop intentará cargar *config/default.toml* si el archivo existe.
//!
//!    * Si se activa la *feature* `testing`, el modo de ejecución es siempre `test` y se ignora
//!      `PAGETOP_RUN_MODE`.
//!
//!    * Permite definir configuraciones específicas por entorno, garantizando que cada uno (p. ej.,
//!      *dev*, *staging* o *production*) disponga de sus propias opciones, como claves de API, URLs
//!      o ajustes de rendimiento, sin afectar a los demás.
//!
//! 3. **config/local.{rm}.toml**, útil para configuraciones locales específicas de la máquina o de
//!    la ejecución:
//!
//!    * Permite añadir o sobrescribir ajustes propios del entorno. Por ejemplo, `local.dev.toml`
//!      para desarrollo o `local.production.toml` para retoques en producción.
//!
//!    * Facilita que cada desarrollador adapte la configuración a su equipo en un entorno dado. Por
//!      lo general no se comparte ni se sube al sistema de control de versiones.
//!
//! 4. **config/local.toml**, para ajustes locales válidos en cualquier entorno, ideal para cambios
//!    rápidos o valores temporales que no dependan de un entorno concreto.
//!
//! Los archivos se combinan clave a clave en el orden anterior. Cada archivo sobrescribe a los
//! anteriores sólo en las claves que define.
//!
//! # Cómo añadir opciones de configuración a tu código
//!
//! Añade [*serde*](https://docs.rs/serde) en tu archivo *Cargo.toml* con la *feature* `derive`:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! Y usa la macro [`include_config!`](crate::include_config) para inicializar tus ajustes en una
//! estructura con tipos seguros. Por ejemplo:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use serde::Deserialize;
//!
//! include_config!(SETTINGS: Settings => [
//!     // [myapp]
//!     "myapp.name"   => "Value Name",
//!     "myapp.width"  => 900,
//!     "myapp.height" => 320,
//! ]);
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Settings {
//!     pub myapp: MyApp,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct MyApp {
//!     pub name: String,
//!     pub description: Option<String>,
//!     pub width: u16,
//!     pub height: u16,
//! }
//! ```
//!
//! De esta forma estás añadiendo una nueva sección `[myapp]` a la configuración, igual que existen
//! `[app]` o `[server]` en las opciones globales de [`Settings`](crate::global::Settings).
//!
//! Se recomienda proporcionar siempre valores por defecto o usar `Option<T>` para los ajustes
//! opcionales.
//!
//! Los valores por defecto se declaran **sólo** en [`include_config!`](crate::include_config).
//!
//! Las estructuras de configuración no necesitan implementar `Default` (ni derivar `AutoDefault`).
//! La deserialización nunca lo usa, y declararlo duplicaría los valores en un segundo sitio que
//! puede quedar desincronizado sin que nada lo detecte.
//!
//! Los ajustes se cargan la primera vez que se accede a la variable generada. Si no pueden cargarse
//! (un archivo TOML mal formado, una clave obligatoria sin valor o un valor de tipo incorrecto),
//! ese primer acceso lanza *panic*: si ocurre al arrancar, detiene la aplicación; si ocurre dentro
//! de una petición, ésta responde con un error y cada acceso posterior vuelve a fallar.
//!
//! Las estructuras de configuración son de **sólo lectura** durante la ejecución.
//!
//! # Usando tus opciones de configuración
//!
//! Los ajustes de cada extensión y los globales de PageTop
//! ([`global::SETTINGS`](crate::global::SETTINGS)) conviven sin conflicto: cada cual vive en su
//! propia sección del TOML y pueden combinarse libremente cuando se necesiten.
//!
//! ```rust,ignore
//! use pagetop::prelude::*;
//! use crate::config;
//!
//! fn global_settings() {
//!     println!("Nombre de la app: {}", &global::SETTINGS.app.name);
//!     println!("Run mode: {}",         &global::SETTINGS.app.run_mode);
//! }
//!
//! fn extension_settings() {
//!     println!("{}", &config::SETTINGS.myapp.name);
//!     println!("{}", &config::SETTINGS.myapp.width);
//! }
//! ```

use crate::{trace, util};

use config::builder::DefaultState;
use config::{Config, ConfigBuilder, File};

use std::env;
use std::fmt;
use std::path::PathBuf;
use std::sync::LazyLock;

// Nombre del directorio de configuración por defecto.
const DEFAULT_CONFIG_DIR: &str = "config";

// Modo de ejecución por defecto.
const DEFAULT_RUN_MODE: &str = "default";

/// Fuentes comunes de la configuración.
///
/// Se obtienen de los archivos TOML en el orden de carga y el modo de ejecución (`app.run_mode`).
///
/// Es la base sobre la que [`include_config!`](crate::include_config) aplica sus valores por
/// defecto antes de deserializar. Los archivos se leen cada vez que se construye la configuración.
pub static CONFIG_VALUES: LazyLock<ConfigBuilder<DefaultState>> = LazyLock::new(|| {
    // CONFIG_DIR (si existe) o DEFAULT_CONFIG_DIR. Si no se puede resolver, se usa tal cual. Sólo
    // se avisa si CONFIG_DIR está definida: que falte el directorio predeterminado es normal en una
    // aplicación sin configuración propia, pero un CONFIG_DIR erróneo podría dejar la aplicación
    // sólo con los valores por defecto sin que nada lo indicara.
    let custom_dir = env::var_os("CONFIG_DIR").filter(|dir| !dir.is_empty());
    let warn_if_unresolved = custom_dir.is_some();
    let dir = custom_dir.unwrap_or_else(|| DEFAULT_CONFIG_DIR.into());
    let config_dir = util::resolve_absolute_dir(&dir).unwrap_or_else(|err| {
        let dir = PathBuf::from(&dir);
        if warn_if_unresolved {
            trace::defer_warning(format!(
                "CONFIG_DIR \"{}\" cannot be resolved ({err}), using it as given. Check settings",
                dir.display()
            ));
        }
        dir
    });

    // Modo de ejecución. Con la *feature* `testing` activa (usada por `cargo ts` y `cargo tw`), se
    // fija en "test" en tiempo de compilación, sin manipular el entorno. En caso contrario se lee
    // de PAGETOP_RUN_MODE, o se usa DEFAULT_RUN_MODE si la variable no está definida o está vacía.
    let rm = if cfg!(feature = "testing") {
        "test".to_string()
    } else {
        env::var("PAGETOP_RUN_MODE")
            .ok()
            .filter(|rm| !rm.is_empty())
            .unwrap_or_else(|| DEFAULT_RUN_MODE.into())
    };

    Config::builder()
        // 1. Configuración común para todos los entornos (common.toml).
        .add_source(File::from(config_dir.join("common.toml")).required(false))
        // 2. Configuración específica del entorno (p. ej. default.toml o production.toml).
        .add_source(File::from(config_dir.join(format!("{rm}.toml"))).required(false))
        // 3. Configuración local reservada para cada entorno (p. ej. local.default.toml).
        .add_source(File::from(config_dir.join(format!("local.{rm}.toml"))).required(false))
        // 4. Configuración local común (local.toml).
        .add_source(File::from(config_dir.join("local.toml")).required(false))
        // Guarda el modo de ejecución explícitamente.
        .set_override("app.run_mode", rm)
        .expect("Failed to set application run mode")
});

// Guarda el aviso de un valor no válido para la opción `key` y devuelve `default`, el valor que se
// usa en su lugar. Lo usan las opciones enumeradas de la configuración al leerse.
pub(crate) fn invalid_setting<T: fmt::Debug>(key: &str, raw: &str, default: T) -> T {
    trace::defer_warning(format!(
        "Invalid value \"{raw}\" for {key}, using \"{default:?}\". Check settings"
    ));
    default
}

// **< include_config! >****************************************************************************

/// Declara una variable global con los ajustes de configuración.
///
/// Incluye los ajustes necesarios de la configuración, aplicando valores por defecto a las claves
/// que no definan los archivos TOML.
///
/// # Sintaxis
///
/// Se declara en el código de la extensión o de la aplicación así:
///
/// ```rust,ignore
/// include_config!(SETTINGS_NAME: SettingsType => [
///     "ruta.clave" => valor,
///     // ...
/// ]);
/// ```
///
/// donde:
///
/// * **`SETTINGS_NAME`** es el nombre de la variable global que se usará para referenciar los
///   ajustes. Se recomienda usar `SETTINGS`, aunque no es obligatorio.
/// * **`SettingsType`** es la estructura que define los tipos para deserializar la configuración.
///   Debe implementar `Deserialize` (derivable con `#[derive(Deserialize)]`).
/// * **Lista de pares** con las claves TOML que requieran valores por defecto. Siguen la notación
///   del árbol TOML, con un punto por nivel (`"blog.title"`, `"blog.comments.enabled"`...). La
///   lista se muestra además en la documentación generada de la variable, con todos los valores
///   por defecto de un vistazo.
///
/// # Ejemplo básico
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use serde::Deserialize;
///
/// include_config!(SETTINGS: BlogSettings => [
///     // [blog]
///     "blog.title" => "Mi Blog",
///     "blog.port"  => 8080,
/// ]);
///
/// #[derive(Debug, Deserialize)]
/// pub struct BlogSettings {
///     pub blog: Blog,
/// }
///
/// #[derive(Debug, Deserialize)]
/// pub struct Blog {
///     pub title: String,
///     pub description: Option<String>,
///     pub port: u16,
/// }
///
/// fn print_title() {
///     // Lectura en tiempo de ejecución.
///     println!("Título: {}", SETTINGS.blog.title);
/// }
/// ```
///
/// # Buenas prácticas
///
/// * **Valores por defecto**. Declara un valor por defecto para cada clave obligatoria. Las claves
///   opcionales pueden ser `Option<T>`. Los valores de esta lista se aplican con `set_default()`
///   antes de deserializar, y sólo en las claves que no defina ningún TOML, así que las estructuras
///   de ajustes no necesitan implementar `Default` ni derivar `AutoDefault`. Si falta una clave
///   obligatoria en los TOML y en la lista, la deserialización falla.
///
/// * **Secciones únicas**. Agrupa tus claves dentro de una sección exclusiva (p. ej. `[blog]`) para
///   evitar colisiones con otras extensiones.
///
/// * **Sólo lectura**. La variable generada es inmutable durante toda la vida del programa. Para
///   configurar distintos entornos (*dev*, *staging*, *production*) usa los archivos TOML descritos
///   en la documentación de [`config`](crate::config).
///
/// * **Errores explícitos**. Si la configuración no puede cargarse, el primer acceso a la variable
///   lanza un `panic!` con un mensaje que nombra la estructura de ajustes y el error de origen
///   (normalmente la clave problemática).
///
/// # Requisitos
///
/// * Cada nivel de las claves debe coincidir con un campo (*snake case*) de la estructura de
///   ajustes o de sus estructuras anidadas.
/// * Añade `serde` con la *feature* `derive` en *Cargo.toml*:
///
/// ```toml
/// [dependencies]
/// serde = { version = "1.0", features = ["derive"] }
/// ```
#[macro_export]
macro_rules! include_config {
    ( $SETTINGS_NAME:ident : $settings_type:ty => [ $( $k:literal => $v:expr ),* $(,)? ] ) => {
        #[doc = concat!(
            "Carga [`", stringify!($settings_type), "`] ",
            "(y aplica **valores por defecto** en claves no definidas)."
        )]
        #[doc = ""]
        #[doc = "Valores predeterminados que se aplican en ausencia de configuración:"]
        #[doc = "```text"]
        $(
            #[doc = concat!($k, " = ", stringify!($v))]
        )*
        #[doc = "```"]
        pub static $SETTINGS_NAME: std::sync::LazyLock<$settings_type> =
            std::sync::LazyLock::new(|| {
                let mut settings = $crate::config::CONFIG_VALUES.clone();
                $(
                    settings = settings.set_default($k, $v)
                        .expect(concat!("Failed to set default for key ", $k));
                )*
                settings
                    .build()
                    .expect(concat!("Failed to build config for ", stringify!($settings_type)))
                    .try_deserialize::<$settings_type>()
                    .expect(concat!("Error parsing settings for ", stringify!($settings_type)))
            });
    };
}
