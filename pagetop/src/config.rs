//! Gestión de la configuración.
//!
//! Comprueba el modo de ejecución actual y carga la configuración asociada.
//!
//! PageTop aplica los principios de [The Twelve-Factor App](https://12factor.net/es/) cargando
//! archivos de configuración [TOML](https://toml.io) con pares `clave = valor` que pueden diferir
//! según el modo de ejecución del entorno actual, o al migrar a otros entornos (*desarrollo*,
//! *pre-producción*, *producción*, etc.).
//!
//! # ¿Cómo usar archivos de configuración?
//!
//! Si tu aplicación requiere opciones de configuración, primero debes crear un directorio llamado
//! *config* (ver [`CONFIG_DIR`]) al mismo nivel del archivo *Cargo.toml* de tu proyecto (o del
//! archivo binario ejecutable de la aplicación).
//!
//! Luego guarda la configuración usando archivos TOML asumiendo el siguiente orden de lectura
//! (todos los archivos son opcionales):
//!
//! 1. *config/common.toml*, útil para asignar valores comunes a cualquier entorno. Estos valores
//!    pueden ser modificados al fusionar los siguientes archivos de configuración.
//!
//! 2. *config/{archivo}.toml*, donde *{archivo}* puede definirse mediante la variable de entorno
//!    PAGETOP_RUN_MODE:
//!
//!     * Si no está definido, se asumirá *default* como nombre predeterminado y PageTop cargará el
//!       archivo de configuración *config/default.toml* si existe.
//!
//!     * De esta manera, se podrían tener diferentes opciones de configuración para diferentes
//!       entornos de ejecución. Por ejemplo, para *devel.toml*, *staging.toml* o *production.toml*.
//!       O también para *server1.toml* o *server2.toml*. Sólo uno será cargado.
//!
//! 3. *config/local.toml*, para añadir o sobrescribir ajustes.

use crate::LazyStatic;

use config_rs::{Config, File};

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

pub fn get(key: &str) -> String {
    match CONFIG.get_string(key) {
        Ok(value) => value,
        _ => match DEFAULTS.read().unwrap().get(key) {
            Some(value) => String::from(*value),
            _ => Default::default(),
        },
    }
}

pub fn get_value<T: FromStr + Default>(key: &str) -> T where <T as FromStr>::Err: Debug {
    get(key).parse::<T>().unwrap_or(Default::default())
}
