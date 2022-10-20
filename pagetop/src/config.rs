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
//! nuevos valores predefinidos con la macro [`predefined_settings!`](crate::predefined_settings).
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
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//! # Loading specific type-safe settings
//!
//! You can use the TOML syntax to create new sections in your configuration
//! files, just as *\[app\]*, *\[webserver\]* or *\[database\]* exist in global
//! settings. Or also add new settings in existing sections.
//!
//! Then you just have to load the configuration to use it from your module or
//! application.
//!
//! To do this, add [*serde*](https://docs.rs/serde) in your application's
//! *Cargo.toml*:
//!
//! ```
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! and use the [`config_map!`] macro to create a new static as follows:
//!
//! ```
//! use pagetop::config_map;
//! use serde::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Section1 {
//!     pub var1: String,
//!     pub var2: u16,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct MySettings {
//!     pub section1: Section1,
//! }
//!
//! config_map!("Application settings.", MYSETTINGS, MySettings);
//! ```
//!
//! Use the first argument of [`config_map!`] for documentation purposes.
//!
//! If `MYSETTINGS` contains variables that are not defined in the configuration
//! files, the application will *panic!*. To avoid this, you can initialize the
//! key=value settings with default values:
//!
//! ```
//! config_map!(r#"
//! My configuration settings for *section1* section.
//! "#,
//!     MYSETTINGS,
//!     MySettings,
//!     "section1.var1" => "seven",
//!     "section1.var2" => 7
//! );
//! ```
//!
//! # How to access configuration
//!
//!  * **Using the** [`config_get!`] **macro**
//!
//!    It will return the value assigned for a given key or an empty String ("")
//!    if it doesn't exist:
//!
//!    ```
//!    use pagetop::config_get;
//!
//!    fn demo() {
//!        println!("Address: {}", config_get!("webserver.bind_address"));
//!        println!("Port: {}", config_get!("webserver.bind_port"));
//!    }
//!    ```
//!
//!  * Or **using the static** [`SETTINGS`] **to get type-safe global settings**
//!
//!    ```
//!    use pagetop::config::SETTINGS;
//!
//!    fn demo() {
//!        println!("App name: {}", &SETTINGS.app.name);
//!        println!("App description: {}", &SETTINGS.app.description);
//!        println!("Value of PAGETOP_RUN_MODE: {}", &SETTINGS.app.run_mode);
//!    }
//!    ```
//!
//!  * Or **using statics to get specific type-safe settings**
//!
//!    Use this for your module or application specific configuration settings.
//!
//!    ```
//!    fn demo() {
//!        println!("{}", &MYSETTINGS.section1.var1);
//!        println!("{}", &MYSETTINGS.section1.var2);
//!    }
//!    ```

mod data;
mod de;
mod error;
mod file;
mod path;
mod source;
mod value;

use crate::{trace, LazyStatic};

use crate::config::data::ConfigData;
use crate::config::file::File;

use serde::Deserialize;

use std::env;
use std::fmt::Debug;
use std::str::FromStr;

/// Directorio donde se encuentran los archivos de configuración.
const CONFIG_DIR: &str = "config";

/// Original key=value settings loaded on application startup.
/// Asigna los ajustes específicos de la aplicación, o de un tema, módulo o componente, en una
/// estructura similar a [`SETTINGS`] con tipos de variables seguros. Produce un *panic!* en caso de
/// asignaciones no válidas.
pub static CONFIG: LazyStatic<ConfigData> = LazyStatic::new(|| {
    // Modo de ejecución según la variable de entorno PAGETOP_RUN_MODE. Por defecto 'default'.
    let run_mode = env::var("PAGETOP_RUN_MODE").unwrap_or_else(|_| "default".into());

    // Inicializa los ajustes.
    let mut settings = ConfigData::default();

    // Combina los archivos de configuración y asigna el modo de ejecución.
    settings
        // Primero añade configuración común a todos los entornos. Opcional.
        .merge(File::with_name(&format!("{}/{}.toml", CONFIG_DIR, "common")).required(false))
        .unwrap()

        // Combina la configuración específica del entorno. Por defecto 'default.toml'. Opcional.
        .merge(File::with_name(&format!("{}/{}.toml", CONFIG_DIR, run_mode)).required(false))
        .unwrap()

        // Combina la configuración local. Este archivo no debería incluirse en git. Opcional.
        .merge(File::with_name(&format!("{}/{}.toml", CONFIG_DIR, "local")).required(false))
        .unwrap()

        // Salvaguarda el modo de ejecución.
        .set("app.run_mode", run_mode)
        .unwrap();

    settings
});

#[macro_export]
/// Loads specific type-safe settings for your module or application in a structure similar to
/// [`SETTINGS`].
///
/// See [`How to load specific type-safe settings`](config/index.html#loading-specific-type-safe-settings).
macro_rules! pub_config_map {
    (
        $doc:expr,
        $SETTINGS:ident,
        $Type:tt
        $(, $key:expr => $value:expr)*
    ) => {
        $crate::doc_comment! {
            concat!($doc),

            pub static $SETTINGS: $crate::LazyStatic<$Type> = $crate::LazyStatic::new(|| {
                let mut settings = $crate::config::CONFIG.clone();
                $(
                    settings.set_default($key, $value).unwrap();
                )*
                match settings.try_into() {
                    Ok(c) => c,
                    Err(e) => panic!("Error parsing settings: {}", e),
                }
            });
        }
    };
}

/// Devuelve el valor (del tipo especificado) de una clave.
///
/// See [`How to access configuration`](config/index.html#how-to-access-configuration).
pub fn get<T: FromStr + Default>(key: &str) -> T where <T as FromStr>::Err: Debug {
    match CONFIG.get_str(key) {
        Ok(value) => match value.parse::<T>() {
            Ok(value) => value,
            _ => {
                trace::warn!("Failed to parse value for key \"{}\"! Return default empty value", key);
                Default::default()
            }
        },
        _ => {
            trace::warn!("Can't get config value for key \"{}\"! Return default empty value", key);
            Default::default()
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct App {
    pub name          : String,
    pub description   : String,
    pub theme         : String,
    pub language      : String,
    pub direction     : String,
    pub startup_banner: String,
    pub run_mode      : String,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct Log {
    pub tracing       : String,
    pub rolling       : String,
    pub path          : String,
    pub prefix        : String,
    pub format        : String,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct Database {
    pub db_type       : String,
    pub db_name       : String,
    pub db_user       : String,
    pub db_pass       : String,
    pub db_host       : String,
    pub db_port       : u16,
    pub max_pool_size : u32,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct Webserver {
    pub bind_address  : String,
    pub bind_port     : u16,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct Dev {
    pub static_files  : String,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app           : App,
    pub log           : Log,
    pub database      : Database,
    pub webserver     : Webserver,
    pub dev           : Dev,
}

pub_config_map!(r#"
Ajustes globales con tipos seguros y valores predefinidos para las secciones *\[app\]*, *\[log\]*,
*\[database\]*, *\[webserver\]* y *\[dev\]* de PageTop.

See [`How to access configuration`](index.html#how-to-access-configuration).
"#,
    SETTINGS, Settings,

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
    "database.db_port"       => 0,
    "database.max_pool_size" => 5,

    // [webserver]
    "webserver.bind_address" => "localhost",
    "webserver.bind_port"    => 8088,

    // [dev]
    "dev.static_files"       => ""
);
