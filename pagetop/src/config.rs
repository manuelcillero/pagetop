//! Gestión de la configuración.
//!
//! Comprueba el modo de ejecución activo y carga la configuración asociada en forma de pares
//! `clave = valor` incluidos en archivos [TOML](https://toml.io).
//!
//! PageTop aplica los principios de [The Twelve-Factor App](https://12factor.net/es/).
//!
//! # ¿Cómo usar los archivos de configuración?
//!
//! Si tu aplicación requiere ajustes de configuración debes crear un directorio llamado *config*
//! (ver [`CONFIG_DIR`]) al mismo nivel del archivo *Cargo.toml* de tu proyecto (o del ejecutable
//! binario de la aplicación).
//!
//! Guarda la configuración usando archivos TOML asumiendo el siguiente orden de lectura (todos los
//! archivos son opcionales):
//!
//! 1. *config/common.toml*, útil para asignar valores comunes para cualquier entorno. Estos valores
//!    pueden ser modificados al fusionar los siguientes archivos de configuración.
//!
//! 2. *config/{archivo}.toml*, donde *{archivo}* puede definirse mediante la variable de entorno
//!    PAGETOP_RUN_MODE:
//!
//!     * Si no está definida, se asumirá *default* como nombre predeterminado y PageTop cargará el
//!       archivo de configuración *config/default.toml* si existe.
//!
//!     * De esta manera, se pueden tener diferentes ajustes de configuración para diferentes
//!       entornos de ejecución. Por ejemplo, para *devel.toml*, *staging.toml* o *production.toml*.
//!       O también para *server1.toml* o *server2.toml*. Sólo uno será cargado.
//!
//! 3. *config/local.toml*, para añadir o sobrescribir ajustes.
//!
//! # ¿Cómo leer los valores de configuración?
//!
//! ```
//! use pagetop::config;
//!
//! // Obtiene el valor (String) de una clave.
//! let app_name: String = config::get("app.name");
//!
//! // Obtiene el valor (del tipo indicado) de una clave.
//! let db_port: u16 = config::get_value::<u16>("database.db_port");
//! ```

use crate::{trace, LazyStatic};

use config_rs::{Config, ConfigError, File};

use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::RwLock;

#[macro_export]
macro_rules! default_settings {
    ( $($key:literal => $value:literal),* ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key, $value);
        )*
        a
    }};
}

/// Directorio donde se encuentran los archivos de configuración.
pub const CONFIG_DIR: &str = "config";

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

static DEFAULTS: LazyStatic<RwLock<HashMap<&str, &str>>> = LazyStatic::new(||
    RwLock::new(default_settings![
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

pub fn add_defaults(defaults: HashMap<&'static str, &'static str>) {
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

/// Devuelve el valor (del tipo indicado) de una clave.
pub fn get_value<T: FromStr + Default>(key: &str) -> T where <T as FromStr>::Err: Debug {
    match get(key).parse::<T>() {
        Ok(value) => value,
        _ => {
            trace::warn!("Failed to parse value for key \"{}\"! Return default empty value", key);
            Default::default()
        }
    }
}
