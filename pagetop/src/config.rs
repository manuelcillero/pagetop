//! Gestión de la configuración.
//!
//! Carga durante el arranque la configuración de la aplicación en forma de pares `clave = valor`
//! recogidos en archivos [TOML](https://toml.io).
//!
//! La metodología [The Twelve-Factor App](https://12factor.net/es/) define **la configuración de
//! una aplicación como todo lo que puede variar entre despliegues**, diferenciando entre entornos
//! de desarrollo, pre-producción, producción, etc.
//!
//! A veces las aplicaciones guardan configuraciones como constantes en el código, lo que supone una
//! violación de esta metodología. Debe existir una **estricta separación entre la configuración y
//! el código**. La configuración variará sustancialmente en cada despliegue, el código no.
//!
//! # Cómo usar archivos de configuración
//!
//! Si tu aplicación requiere archivos de configuración debes crear un directorio llamado *config* al
//! mismo nivel del archivo *Cargo.toml* de tu proyecto (o del ejecutable binario de la aplicación).
//!
//! Guarda la configuración usando archivos TOML asumiendo el siguiente orden de lectura secuencial
//! (todos los archivos son opcionales):
//!
//! 1. *config/common.toml*, útil para los ajustes comunes para cualquier entorno. Estos valores
//!    podrán ser sobrescritos al fusionar los archivos de configuración siguientes.
//!
//! 2. *config/{archivo}.toml*, donde *{archivo}* puede definirse mediante la variable de entorno
//!    PAGETOP_RUN_MODE:
//!
//!     * Si no está definida, se asumirá *default* por defecto, y PageTop cargará el archivo de
//!       configuración *config/default.toml* si existe.
//!
//!     * De esta manera, se pueden tener diferentes ajustes de configuración para diferentes
//!       entornos de ejecución. Por ejemplo, para *devel.toml*, *staging.toml* o *production.toml*.
//!       O también para *server1.toml* o *server2.toml*. Sólo uno será cargado.
//!
//!     * Normalmente estos archivos suelen ser idóneos para incluir contraseñas o configuración
//!       sensible asociada al entorno correspondiente. Estos archivos no deberían ser publicados en
//!       el repositorio Git por razones de seguridad.
//!
//! 3. *config/local.toml*, para añadir o sobrescribir ajustes.
//!
//! # Cómo añadir valores predefinidos de configuración
//!
//! Si nuestra **aplicación** o **módulo** requiere sus propios ajustes de configuración, es
//! recomendable (aunque no imprescindible) inicializarlos antes de su uso.
//!
//! Sólo tienes que añadir el método [`settings()`](crate::core::module::ModuleTrait::settings) al
//! implementar [`ModuleTrait`](crate::core::module::ModuleTrait) para tu módulo, devolviendo los
//! nuevos valores predefinidos con la macro [`predefined_settings!`].
//!
//! Cuando se carga la configuración de la aplicación, estos valores podrán ser sobrescritos con los
//! ajustes personalizados del entorno. Y sólo será realmente necesario incluir en los archivos de
//! configuración los ajustes que difieran de los predefinidos.
//!
//! ```
//! use pagetop::prelude::*;
//!
//! pub_const_handler!(MY_MODULE_HANDLER);
//!
//! pub struct MyModule;
//!
//! impl ModuleTrait for MyModule {
//!     fn handler(&self) -> Handler {
//!         MY_MODULE_HANDLER
//!     }
//!
//!     fn settings(&self) -> PredefinedSettings {
//!         predefined_settings![
//!           // Valores predefinidos para "my_module".
//!           "my_module.name" => "Name",
//!           "my_module.desc" => "Description",
//!           // Valores predefinidos para "my_module.database".
//!           "my_module.database.db_port" => "3306"
//!         ]
//!     }
//! }
//! ```
//!
//! # Cómo obtener los valores de configuración
//!
//! ```
//! use pagetop::config;
//!
//! // Obtiene el valor (String) de una clave.
//! let name: String = config::get("my_module.name");
//!
//! // Obtiene el valor (del tipo especificado) de una clave.
//! let db_port: u16 = config::get_value::<u16>("my_module.database.db_port");
//! ```

use crate::{trace, LazyStatic};

use config_rs::{Config, ConfigError, File};

use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::RwLock;

pub type PredefinedSettings = HashMap<&'static str, &'static str>;

#[macro_export]
macro_rules! predefined_settings {
    ( $($key:literal => $value:literal),* ) => {{
        #[allow(unused_mut)]
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key, $value);
        )*
        a
    }};
}

/// Directorio donde se encuentran los archivos de configuración.
const CONFIG_DIR: &str = "config";

/// Carga los valores originales "clave = valor" de los archivos de configuración. Con
/// [`config_map`] se asignarán los ajustes globales ([`SETTINGS`]); y se podrán asignar los ajustes
/// específicos de la aplicación, o también de un tema, módulo o componente.
static CONFIG: LazyStatic<Config> = LazyStatic::new(|| {
    // Modo de ejecución según la variable de entorno PAGETOP_RUN_MODE. Por defecto 'default'.
    let run_mode = env::var("PAGETOP_RUN_MODE").unwrap_or_else(|_| "default".into());

    // Inicializa los ajustes.
    let settings = Config::builder();

    // Combina los archivos de configuración y asigna el modo de ejecución.
    settings
        // Primero añade configuración común a todos los entornos. Opcional.
        .add_source(File::with_name(&format!("{}/{}.toml", CONFIG_DIR, "common")).required(false))

        // Combina la configuración específica del entorno. Por defecto 'default.toml'. Opcional.
        .add_source(File::with_name(&format!("{}/{}.toml", CONFIG_DIR, run_mode)).required(false))

        // Combina la configuración local. Este archivo no debería incluirse en git. Opcional.
        .add_source(File::with_name(&format!("{}/{}.toml", CONFIG_DIR, "local")).required(false))

        // Salvaguarda el modo de ejecución.
        .set_default("app.run_mode", run_mode)

        .unwrap()
        .build()
        .unwrap()
});

static DEFAULTS: LazyStatic<RwLock<PredefinedSettings>> = LazyStatic::new(||
    RwLock::new(predefined_settings![
        // [app]
        "app.name"               => "PageTop Application",
        "app.description"        => "Developed with the amazing PageTop framework.",
        "app.theme"              => "Bootsier",
        "app.language"           => "en-US",
        "app.direction"          => "ltr",
        "app.startup_banner"     => "Slant",

        // [log]
        "log.tracing"            => "Info",
        "log.rolling"            => "Stdout",
        "log.path"               => "log",
        "log.prefix"             => "tracing.log",
        "log.format"             => "Full",

        // [database]
        "database.db_type"       => "",
        "database.db_name"       => "",
        "database.db_user"       => "",
        "database.db_pass"       => "",
        "database.db_host"       => "localhost",
        "database.db_port"       => "0",
        "database.max_pool_size" => "5",

        // [webserver]
        "webserver.bind_address" => "localhost",
        "webserver.bind_port"    => "8088",

        // [dev]
        "dev.static_files"       => ""
    ])
);

/// Una aplicación o módulo podrá añadir nuevos valores predefinidos de configuración.
pub(crate) fn add_predefined_settings(defaults: PredefinedSettings) {
    DEFAULTS.write().unwrap().extend(defaults);
}

/// Devuelve el valor (String) de una clave.
pub fn get(key: &str) -> String {
    match CONFIG.get_string(key) {
        Ok(value) => value,
        Err(ConfigError::NotFound(_)) => match DEFAULTS.read().unwrap().get(key) {
            Some(value) => String::from(*value),
            _ => {
                trace::debug!("Config value not found for key \"{}\"! Return empty string", key);
                Default::default()
            }
        },
        _ => {
            trace::warn!("Can't read config value for key \"{}\"! Return empty string", key);
            Default::default()
        }
    }
}

/// Devuelve el valor (del tipo especificado) de una clave.
pub fn get_value<T: FromStr + Default>(key: &str) -> T where <T as FromStr>::Err: Debug {
    match get(key).parse::<T>() {
        Ok(value) => value,
        _ => {
            trace::warn!("Failed to parse value for key \"{}\"! Return default empty value", key);
            Default::default()
        }
    }
}
