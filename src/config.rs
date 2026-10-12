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
//! # Cómo declarar ajustes
//!
//! Cada extensión declara sus ajustes en una estructura con el atributo
//! [`#[settings]`](crate::settings), que corresponde a una sección propia del archivo de
//! configuración. Los valores por defecto se escriben en los propios campos con `#[default = ...]`,
//! así que el compilador comprueba sus tipos, y la documentación de cada campo muestra el suyo.
//!
//! Lo habitual es definirlos en un archivo `config.rs` de la extensión con algo como esto:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//!
//! /// Ajustes de la sección `[blog]`, cargados al arrancar la aplicación.
//! pub static SETTINGS: LazySettings<BlogSettings> = LazySettings::new();
//!
//! /// Sección **`[blog]`** de la configuración.
//! #[settings(section = "blog")]
//! pub struct BlogSettings {
//!     /// Título del blog.
//!     #[default = "Mi Blog"]
//!     pub title: String,
//!     /// Descripción opcional (sin valor por defecto: `None`).
//!     pub description: Option<String>,
//!     /// Comentarios, en la subsección `[blog.comments]`.
//!     pub comments: CommentsSettings,
//! }
//!
//! /// Subsección **`[blog.comments]`** de la configuración.
//! #[settings]
//! pub struct CommentsSettings {
//!     /// Si se admiten comentarios.
//!     #[default = true]
//!     pub enabled: bool,
//!     /// Número máximo de comentarios por página.
//!     #[default = 20]
//!     pub page_size: u32,
//! }
//! ```
//!
//! [`LazySettings`] lee la sección indicada en `section`; las claves que falten toman su valor por
//! defecto, y si falta la sección entera se usan los valores por defecto de todos los campos.
//!
//! Para que los ajustes se carguen al arrancar, la extensión los devuelve en
//! [`Extension::settings()`](crate::core::extension::Extension::settings):
//!
//! ```rust,ignore
//! pub mod config;
//!
//! impl Extension for Blog {
//!     fn settings(&self) -> Option<&'static dyn LoadSettings> {
//!         Some(&config::SETTINGS)
//!     }
//! }
//! ```
//!
//! Desde cualquier parte del código se leen como `config::SETTINGS.title` o
//! `config::SETTINGS.comments.enabled`. Son de **sólo lectura** durante la ejecución.
//!
//! # Carga y errores
//!
//! PageTop carga los ajustes de todas las extensiones durante el arranque, en el orden en que se
//! registran y antes de inicializarlas. Si unos ajustes no pueden cargarse, la aplicación se
//! detiene con un mensaje que indica el problema: el archivo y la línea si un TOML está mal
//! formado, o la sección y el valor no válido o la **clave desconocida** (típicamente una errata).
//! También avisa al arrancar de las secciones de los archivos que no corresponden a PageTop ni a
//! ninguna extensión.
//!
//! Fuera de una aplicación en marcha (p. ej. en pruebas unitarias) los ajustes se cargan en el
//! primer acceso a la variable, con las mismas reglas.
//!
//! Los ajustes globales de PageTop están en [`global::SETTINGS`](crate::global::SETTINGS).

use crate::{trace, util};

use config::{Config, ConfigError, File, FileFormat};
use serde::de::{DeserializeOwned, IgnoredAny};

use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::LazyLock;

// Nombre del directorio de configuración por defecto.
const DEFAULT_CONFIG_DIR: &str = "config";

// Modo de ejecución por defecto.
const DEFAULT_RUN_MODE: &str = "default";

// Configuración combinada de los archivos TOML en el orden de carga, más el modo de ejecución
// (`app.run_mode`). Se construye una sola vez, en el primer acceso, y sólo la leen las funciones de
// este módulo: así ningún ajuste se salta las comprobaciones de `#[settings]`.
static CONFIG_VALUES: LazyLock<Config> = LazyLock::new(|| {
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
        .and_then(|builder| builder.build())
        .unwrap_or_else(|err| panic!("Failed to read the configuration files: {err}"))
});

// Secciones del primer nivel de la configuración.
pub(crate) fn sections() -> Vec<String> {
    CONFIG_VALUES
        .clone()
        .try_deserialize::<HashMap<String, IgnoredAny>>()
        .map(|sections| sections.into_keys().collect())
        .unwrap_or_default()
}

// **< Ajustes >************************************************************************************

/// Estructura de ajustes asociada a una sección del archivo de configuración.
///
/// La implementa el atributo [`#[settings(section = "...")]`](crate::settings); no es necesario
/// implementarla a mano.
pub trait SettingsSection: DeserializeOwned + Default + Send + Sync + 'static {
    /// Nombre de la sección en el archivo de configuración (p. ej. `"blog"` para `[blog]`).
    const SECTION: &'static str;
}

// Carga los ajustes de la sección `T::SECTION` de la configuración. Las claves ausentes toman su
// valor por defecto, y si falta la sección entera se usan los de todos los campos. Si la sección no
// puede deserializarse (un valor no válido o una clave desconocida), lanza `panic!` con un mensaje
// que indica la sección y el problema.
pub(crate) fn load<T: SettingsSection>() -> T {
    section_from(&CONFIG_VALUES)
        .unwrap_or_else(|err| panic!("Invalid settings in section [{}]: {err}", T::SECTION))
}

/// Carga los ajustes de la sección [`SettingsSection::SECTION`] a partir de un texto TOML, con las
/// mismas reglas que [`LazySettings`].
///
/// Pensada para pruebas, p. ej. para comprobar que un archivo de configuración de referencia sólo
/// documenta claves que existen. No forma parte de la API pública.
#[doc(hidden)]
pub fn load_from_str<T: SettingsSection>(toml: &str) -> Result<T, ConfigError> {
    let config = Config::builder()
        .add_source(File::from_str(toml, FileFormat::Toml))
        .build()?;
    section_from(&config)
}

// Lee la sección de `T` de una configuración; si falta entera, usa los valores por defecto.
fn section_from<T: SettingsSection>(config: &Config) -> Result<T, ConfigError> {
    match config.get::<T>(T::SECTION) {
        Err(ConfigError::NotFound(_)) => Ok(T::default()),
        result => result,
    }
}

// **< LazySettings >*******************************************************************************

/// Variable con los ajustes de una sección de la configuración.
///
/// Se declara como `static` y los ajustes se cargan al arrancar la aplicación (o, fuera de ella, en
/// el primer acceso). Como buena práctica, se declara en el módulo `config` de la extensión
/// (archivo `config.rs`), junto a las estructuras de sus ajustes:
///
/// ```rust,ignore
/// pub static SETTINGS: LazySettings<BlogSettings> = LazySettings::new();
/// ```
///
/// Y desde el resto del código se accede a los ajustes a través de la variable, cualificada con el
/// módulo:
///
/// ```rust,ignore
/// let title = &config::SETTINGS.title;
/// ```
///
/// Las claves ausentes toman su valor por defecto, y si falta la sección entera se usan los valores
/// por defecto de todos los campos.
///
/// # Panics
///
/// Al cargarse, si la sección no puede deserializarse (un valor no válido o una clave desconocida),
/// con un mensaje que indica la sección y el problema.
pub struct LazySettings<T: SettingsSection>(LazyLock<T>);

impl<T: SettingsSection> LazySettings<T> {
    /// Crea la variable de ajustes de la sección [`SettingsSection::SECTION`] de `T`.
    pub const fn new() -> Self {
        Self(LazyLock::new(load::<T>))
    }
}

// A mano y no derivado: el `Default` de `LazyLock` usaría `T::default()` en lugar de cargar los
// ajustes de la configuración.
impl<T: SettingsSection> Default for LazySettings<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: SettingsSection> Deref for LazySettings<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// **< LoadSettings >*******************************************************************************

/// Ajustes que PageTop carga durante el arranque.
///
/// Lo implementa [`LazySettings<T>`] con `T` declarado con
/// [`#[settings(section = "...")]`](crate::settings), y las extensiones devuelven esa variable en
/// [`Extension::settings()`](crate::core::extension::Extension::settings).
pub trait LoadSettings: Sync {
    /// Carga los ajustes si aún no se han cargado.
    fn load(&self);

    /// Sección de la configuración a la que corresponden los ajustes.
    fn section(&self) -> &'static str;
}

impl<T: SettingsSection> LoadSettings for LazySettings<T> {
    fn load(&self) {
        LazyLock::force(&self.0);
    }

    fn section(&self) -> &'static str {
        T::SECTION
    }
}
