//! Carga las opciones de configuración de la aplicación.
//!
//! Estos ajustes se obtienen de archivos [TOML](https://toml.io) como pares `clave = valor` que se
//! deserializan en estructuras **fuertemente tipadas**, con valores por defecto para las claves que
//! no se definan.
//!
//! Separar la configuración del código permite que cada despliegue (*dev*, *staging*, *production*,
//! etc.) tenga su propia configuración sin recompilar. Los ajustes se leen de archivos y pueden
//! sobrescribirse con [variables de entorno](#variables-de-entorno), útiles para pasar secretos o
//! ajustes de un despliegue concreto (Docker, systemd, Kubernetes...) sin escribirlos en archivos.
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
//! # Variables de entorno
//!
//! Por último, las variables de entorno con el prefijo `PAGETOP_` sobrescriben los valores de los
//! archivos. El nombre de la variable se forma con la sección y la clave en mayúsculas, separando
//! cada nivel con dos guiones bajos (`__`), porque las claves ya llevan guiones bajos simples:
//!
//! ```sh
//! PAGETOP_DATABASE__DB_PASS=secreto      # [database] db_pass
//! PAGETOP_MAIL__SMTP__PASSWORD=secreto   # [mail.smtp] password
//! PAGETOP_SERVER__BIND_PORT=80           # [server] bind_port
//! ```
//!
//! El valor se convierte al tipo del ajuste: un número se escribe en decimal (`80`), un booleano
//! como `true` o `false`, y una cadena tal cual, sin comillas. Como en los archivos, un valor no
//! válido o una clave desconocida detienen el arranque. Además:
//!
//! * Una variable vacía equivale a no definirla, así que no sirve para vaciar un valor de los
//!   archivos.
//!
//! * Una variable de ajustes cuyo valor no es Unicode válido también detiene el arranque.
//!
//! * No sirven para las listas ni para las claves que llevan un punto, como `mail.queue` en
//!   `[cron.jobs."mail.queue"]`. Ésas se configuran en los archivos. Un trabajo con una clave sin
//!   punto, como `cleanup`, sí: `PAGETOP_CRON__JOBS__CLEANUP__ENABLED=false`.
//!
//! * Con la *feature* `testing` se ignoran, como `PAGETOP_RUN_MODE`, para que las variables de la
//!   terminal no cambien el resultado de las pruebas.
//!
//! Al arrancar, las trazas indican qué ajustes vienen de variables de entorno (las claves, nunca
//! los valores), para que se sepa de dónde sale un valor que no aparece en los archivos.
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
//! formado, o la sección y el valor no válido o la **clave desconocida** (típicamente una errata),
//! junto con los nombres de las variables de entorno que sobrescriben esa sección, si las hay.
//! También avisa al arrancar de las secciones que no corresponden a PageTop ni a ninguna extensión,
//! vengan de los archivos o de variables de entorno.
//!
//! Fuera de una aplicación en marcha (p. ej. en pruebas unitarias) los ajustes se cargan en el
//! primer acceso a la variable, con las mismas reglas.
//!
//! Los ajustes globales de PageTop están en [`global::SETTINGS`](crate::global::SETTINGS).

use crate::{AutoDefault, BoxError, trace, util};

use config::{Config, ConfigError, Environment, File, FileFormat, Map};
use serde::de::{DeserializeOwned, IgnoredAny};

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock};

// Nombre del directorio de configuración por defecto.
const DEFAULT_CONFIG_DIR: &str = "config";

// Modo de ejecución por defecto.
const DEFAULT_RUN_MODE: &str = "default";

// Prefijo de las variables de entorno que sobrescriben ajustes.
const ENV_PREFIX: &str = "PAGETOP_";

// Separador de niveles (sección, subsección y clave) en el nombre de esas variables.
const ENV_SEPARATOR: &str = "__";

// Variables de entorno que sobrescriben ajustes, o el texto del error si alguna no se puede leer.
// Con la *feature* `testing` no se lee ninguna.
static ENV_VARS: LazyLock<Result<Map<String, String>, String>> = LazyLock::new(|| {
    if cfg!(feature = "testing") {
        return Ok(Map::new());
    }
    env_overrides(env::vars_os())
});

// Selecciona las variables que sobrescriben ajustes: las que empiezan por `ENV_PREFIX`, con al
// menos un `ENV_SEPARATOR` tras el prefijo (sección y clave) y un valor no vacío. Un valor que no
// es Unicode válido es un error, como cualquier otro valor no válido; un nombre que no lo es no
// puede ser el de un ajuste y se ignora, como el resto de variables ajenas.
fn env_overrides(
    vars: impl IntoIterator<Item = (OsString, OsString)>,
) -> Result<Map<String, String>, String> {
    let mut overrides = Map::new();
    for (key, value) in vars {
        let Some(key) = key.to_str().filter(|key| {
            key.strip_prefix(ENV_PREFIX)
                .is_some_and(|rest| rest.contains(ENV_SEPARATOR))
        }) else {
            continue;
        };
        if value.is_empty() {
            continue;
        }
        let value = value
            .into_string()
            .map_err(|_| format!("Environment variable {key} is not valid Unicode"))?;
        overrides.insert(key.to_string(), value);
    }
    Ok(overrides)
}

// Nombres de las variables de entorno que sobrescriben ajustes. Ninguno si no se pudieron leer: en
// ese caso falla la lectura de la configuración, que devuelve el error.
fn env_names() -> impl Iterator<Item = &'static str> {
    ENV_VARS
        .iter()
        .flat_map(|vars| vars.keys())
        .map(String::as_str)
}

// Añade al error de una sección los nombres de las variables de entorno que la sobrescriben, si
// hay alguna. El error de una clave desconocida no dice de dónde viene, y una que llega de una
// variable no aparece en ningún archivo.
fn with_env_names<'a>(
    err: ConfigError,
    section: &str,
    names: impl Iterator<Item = &'a str>,
) -> ConfigError {
    let mut names: Vec<&str> = names
        .filter(|name| {
            name.strip_prefix(ENV_PREFIX)
                .and_then(|rest| rest.split(ENV_SEPARATOR).next())
                .is_some_and(|name_section| name_section.eq_ignore_ascii_case(section))
        })
        .collect();
    if names.is_empty() {
        return err;
    }
    names.sort_unstable();
    ConfigError::Message(format!(
        "{err} (environment variables in this section: {})",
        names.join(", ")
    ))
}

// Fuente de configuración con las variables ya filtradas. Sin `try_parsing`: los valores llegan
// como cadenas y se convierten al deserializar según el tipo de cada ajuste; con él, una
// contraseña `0123` llegaría como el número `123`.
fn env_source(vars: Map<String, String>) -> Environment {
    Environment::with_prefix(ENV_PREFIX.trim_end_matches('_'))
        .prefix_separator("_")
        .separator(ENV_SEPARATOR)
        .source(Some(vars))
}

// Clave de cada ajuste sobrescrito por una variable de entorno (p. ej. `database.db_pass`), en
// orden alfabético.
pub(crate) fn env_keys() -> Vec<String> {
    let mut keys: Vec<String> = env_names()
        .map(|name| {
            name[ENV_PREFIX.len()..]
                .to_lowercase()
                .replace(ENV_SEPARATOR, ".")
        })
        .collect();
    keys.sort();
    keys
}

// Configuración combinada de los archivos TOML en el orden de carga y de las variables de entorno,
// más el modo de ejecución (`app.run_mode`), o el texto del error si no se pudieron leer
// (`ConfigError` no implementa `Clone`). Se construye una sola vez, en el primer acceso, y sólo la
// leen las funciones de este módulo: así ningún ajuste se salta las comprobaciones de
// `#[settings]`.
static CONFIG_VALUES: LazyLock<Result<Config, String>> = LazyLock::new(|| {
    let env_vars = ENV_VARS.clone()?;

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
        // 5. Variables de entorno (p. ej. PAGETOP_DATABASE__DB_PASS).
        .add_source(env_source(env_vars))
        // Guarda el modo de ejecución explícitamente.
        .set_override("app.run_mode", rm)
        .and_then(|builder| builder.build())
        .map_err(|err| err.to_string())
});

// Configuración combinada, o el error que impidió leer los archivos o las variables de entorno.
fn config_values() -> Result<&'static Config, ConfigError> {
    CONFIG_VALUES
        .as_ref()
        .map_err(|message| ConfigError::Message(message.clone()))
}

// Comprueba que la configuración se puede leer (p. ej. que ningún TOML está mal formado), para
// detener el arranque con un error en lugar del `panic!` del primer acceso.
pub(crate) fn check_sources() -> Result<(), ConfigError> {
    config_values().map(|_| ())
}

// Comprueba, sin guardarlos, que los ajustes de la sección de `T` se pueden cargar. Sirve para los
// ajustes que se guardan en otra variable, como los globales de PageTop.
pub(crate) fn check<T: SettingsSection>() -> Result<(), ConfigError> {
    read_section::<T>(config_values()?).map(|_| ())
}

// Secciones del primer nivel de la configuración.
pub(crate) fn sections() -> Vec<String> {
    config_values()
        .ok()
        .and_then(|config| {
            config
                .clone()
                .try_deserialize::<HashMap<String, IgnoredAny>>()
                .ok()
        })
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
// valor por defecto, y si falta la sección entera se usan los de todos los campos. Si la
// configuración no se puede leer o la sección no puede deserializarse (un valor no válido o una
// clave desconocida), lanza `panic!` con un mensaje que indica el problema. Al arrancar no llega a
// ocurrir: la aplicación lo comprueba antes y se detiene con un error.
pub(crate) fn load<T: SettingsSection>() -> T {
    let config =
        config_values().unwrap_or_else(|err| panic!("Failed to read the configuration: {err}"));
    read_section(config)
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

// Lee la sección de `T` de la configuración combinada. Si no puede, el error nombra además las
// variables de entorno que sobrescriben la sección.
fn read_section<T: SettingsSection>(config: &Config) -> Result<T, ConfigError> {
    section_from(config).map_err(|err| with_env_names(err, T::SECTION, env_names()))
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
/// Se declara como `static`. Como buena práctica, se declara en el módulo `config` de la extensión
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
/// Si la extensión devuelve la variable en [`Extension::settings()`], los ajustes se cargan al
/// arrancar y una sección que no puede deserializarse (un valor no válido o una clave desconocida)
/// detiene el arranque. Si no, se cargan en el primer acceso.
///
/// # Panics
///
/// En el primer acceso a unos ajustes aún no cargados, si la sección no puede deserializarse, con
/// un mensaje que indica la sección y el problema. Ocurre si la extensión no los devuelve en
/// `Extension::settings()`, si se accede a ellos antes de cargarlos al arrancar (p. ej. desde
/// `Extension::dependencies()`) o si se usan fuera de una aplicación (p. ej. en una prueba).
///
/// [`Extension::settings()`]: crate::core::extension::Extension::settings
#[derive(AutoDefault)]
pub struct LazySettings<T: SettingsSection>(OnceLock<T>);

impl<T: SettingsSection> LazySettings<T> {
    /// Crea la variable de ajustes de la sección [`SettingsSection::SECTION`] de `T`.
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }
}

impl<T: SettingsSection> Deref for LazySettings<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.0.get_or_init(load::<T>)
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
    ///
    /// Devuelve un error si los ajustes no se pueden cargar, lo que detiene el arranque de la
    /// aplicación.
    fn load(&self) -> Result<(), BoxError>;

    /// Sección de la configuración a la que corresponden los ajustes.
    fn section(&self) -> &'static str;
}

impl<T: SettingsSection> LoadSettings for LazySettings<T> {
    // Devuelve el error en lugar del `panic!` del primer acceso. Si otro hilo guarda antes los
    // ajustes (las pruebas preparan aplicaciones en paralelo), se conservan los suyos, que son los
    // mismos.
    fn load(&self) -> Result<(), BoxError> {
        if self.0.get().is_none() {
            let _ = self.0.set(read_section::<T>(config_values()?)?);
        }
        Ok(())
    }

    fn section(&self) -> &'static str {
        T::SECTION
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Config, ConfigError, File, FileFormat, OsString, env_overrides, env_source, section_from,
        with_env_names,
    };
    use crate::settings;

    #[settings(section = "sample")]
    struct SampleSettings {
        #[default = "from-default"]
        name: String,
        #[default = "from-default"]
        password: String,
        #[default = 8080]
        port: u16,
        #[default = false]
        enabled: bool,
        limit: Option<u16>,
        nested: NestedSettings,
    }

    #[settings]
    struct NestedSettings {
        #[default = "from-default"]
        user: String,
    }

    const TOML: &str = r#"
        [sample]
        name = "from-file"
        password = "from-file"
        port = 3000
    "#;

    fn os_vars<'a>(vars: &'a [(&str, &str)]) -> impl Iterator<Item = (OsString, OsString)> + 'a {
        vars.iter().map(|(k, v)| (k.into(), v.into()))
    }

    fn load(vars: &[(&str, &str)]) -> Result<SampleSettings, ConfigError> {
        let config = Config::builder()
            .add_source(File::from_str(TOML, FileFormat::Toml))
            .add_source(env_source(env_overrides(os_vars(vars)).unwrap()))
            .build()?;
        section_from::<SampleSettings>(&config)
    }

    #[test]
    fn env_vars_override_file_values() {
        let s = load(&[
            ("PAGETOP_SAMPLE__PASSWORD", "from-env"),
            ("PAGETOP_SAMPLE__PORT", "80"),
            ("PAGETOP_SAMPLE__ENABLED", "true"),
            ("PAGETOP_SAMPLE__LIMIT", "5"),
            ("PAGETOP_SAMPLE__NESTED__USER", "from-env"),
        ])
        .unwrap();
        assert_eq!(s.name, "from-file");
        assert_eq!(s.password, "from-env");
        assert_eq!(s.port, 80);
        assert!(s.enabled);
        assert_eq!(s.limit, Some(5));
        assert_eq!(s.nested.user, "from-env");
    }

    #[test]
    fn env_values_are_not_parsed_before_deserializing() {
        // A numeric-looking string must keep its leading zeros.
        let s = load(&[("PAGETOP_SAMPLE__PASSWORD", "0123")]).unwrap();
        assert_eq!(s.password, "0123");
    }

    #[test]
    fn env_var_with_unknown_key_is_rejected() {
        let error = load(&[("PAGETOP_SAMPLE__PASWORD", "typo")]).err().unwrap();
        assert!(
            error.to_string().contains("pasword"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn env_var_with_invalid_value_is_rejected() {
        assert!(load(&[("PAGETOP_SAMPLE__PORT", "not-a-port")]).is_err());
    }

    #[test]
    fn empty_env_vars_are_ignored() {
        let s = load(&[("PAGETOP_SAMPLE__PASSWORD", "")]).unwrap();
        assert_eq!(s.password, "from-file");
    }

    #[test]
    fn only_prefixed_vars_with_section_and_key_are_taken() {
        let vars = env_overrides(os_vars(&[
            ("PAGETOP_RUN_MODE", "production"),
            ("PAGETOP_SAMPLE__NAME", "x"),
            ("OTHER_SAMPLE__NAME", "x"),
            ("pagetop_sample__name", "x"),
        ]))
        .unwrap();
        assert_eq!(vars.len(), 1);
        assert!(vars.contains_key("PAGETOP_SAMPLE__NAME"));
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_values_are_rejected_only_in_setting_vars() {
        use std::os::unix::ffi::OsStringExt;

        let invalid = || OsString::from_vec(vec![0xff]);
        let error = env_overrides([("PAGETOP_SAMPLE__NAME".into(), invalid())]).unwrap_err();
        assert!(
            error.contains("PAGETOP_SAMPLE__NAME"),
            "unexpected error: {error}"
        );
        // Foreign variables, and names that cannot be settings, are ignored.
        assert!(
            env_overrides([("OTHER_VAR".into(), invalid())])
                .unwrap()
                .is_empty()
        );
        assert!(env_overrides([(invalid(), "x".into())]).unwrap().is_empty());
    }

    #[test]
    fn section_errors_name_the_env_vars_of_that_section() {
        let names = [
            "PAGETOP_SAMPLE__PASWORD",
            "PAGETOP_OTHER__NAME",
            "PAGETOP_SAMPLE__NESTED__USER",
        ];
        let error = with_env_names(
            ConfigError::Message("unknown field `pasword`".into()),
            "sample",
            names.into_iter(),
        );
        assert_eq!(
            error.to_string(),
            "unknown field `pasword` (environment variables in this section: \
             PAGETOP_SAMPLE__NESTED__USER, PAGETOP_SAMPLE__PASWORD)"
        );
        // Without variables for the section, the error is left as it was.
        let error = with_env_names(
            ConfigError::Message("unknown field `x`".into()),
            "sample",
            ["PAGETOP_OTHER__NAME"].into_iter(),
        );
        assert_eq!(error.to_string(), "unknown field `x`");
    }
}
