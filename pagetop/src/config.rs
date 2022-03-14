use crate::Lazy;

use config_rs::{Config, File};
use serde::Deserialize;

use std::env;

/// Nombre del directorio donde se encuentra la configuración.
const CONFIG_DIR: &'static str = "config";

/// Al arrancar la aplicación, carga los valores originales "clave = valor" de
/// los archivos de configuración. Con [`config_map`] se asignarán los ajustes
/// globales ([`SETTINGS`]); y se podrán asignar los ajustes específicos de la
/// aplicación, o también de un tema, módulo o componente.
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    // Establece el modo de ejecución según el valor de la variable de entorno
    // PAGETOP_RUN_MODE. Asume "default" por defecto.
    let run_mode = env::var("PAGETOP_RUN_MODE").unwrap_or("default".into());

    // Inicializa los ajustes.
    let mut settings = Config::default();

    // Combina los archivos de configuración y asigna el modo de ejecución.
    settings
        .merge(
            File::with_name(
                &format!("{}/{}.toml", CONFIG_DIR, "common")
            ).required(false)).unwrap()
        .merge(
            File::with_name(
                &format!("{}/{}.toml", CONFIG_DIR, run_mode)
            ).required(false)).unwrap()
        .merge(
            File::with_name(
                &format!("{}/{}.toml", CONFIG_DIR, "local")
            ).required(false)).unwrap()
        .set("app.run_mode", run_mode).unwrap();

    settings
});

#[macro_export]
/// Asigna los ajustes específicos de la aplicación, o de un tema, módulo o
/// componente, en una estructura similar a [`SETTINGS`] con tipos de variables
/// seguros. Produce un *panic!* en caso de asignaciones no válidas.
macro_rules! config_map {
    (
        $COMM:expr,
        $CONF:ident,
        $TYPE:tt
        $(, $key:expr => $value:expr)*
    ) => {
        $crate::doc_comment! {
            concat!($COMM),

            pub static $CONF: $crate::Lazy<$TYPE> = $crate::Lazy::new(|| {
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

#[derive(Debug, Deserialize)]
pub struct Log {
    pub tracing       : String,
    pub rolling       : String,
    pub path          : String,
    pub prefix        : String,
    pub format        : String,
}

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

#[derive(Debug, Deserialize)]
pub struct Webserver {
    pub bind_address  : String,
    pub bind_port     : u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app           : App,
    pub log           : Log,
    pub database      : Database,
    pub webserver     : Webserver,
}

config_map!(r#"
Ajustes globales y valores predeterminados para las secciones *\[app\]*,
*\[log\]* y *\[webserver\]* de PageTop.
"#,
    SETTINGS, Settings,

    // [app]
    "app.name"               => "PageTop Application",
    "app.description"        => "Developed with the amazing PageTop framework.",
    "app.theme"              => "Bootsier",
    "app.language"           => "en-US",
    "app.direction"          => "ltr",
    "app.startup_banner"     => "Small",

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
    "webserver.bind_port"    => 8088
);
