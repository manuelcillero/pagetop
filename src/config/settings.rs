use crate::Lazy;
use crate::config::CONFIG_DIR;

use config_rs::{Config, File};
use serde::Deserialize;

use std::env;

/// Carga los ajustes globales "clave = valor" al arrancar la aplicación.
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    // Establece el modo de ejecución según el valor de la variable de entorno
    // PAGETOP_RUN_MODE. Asume "default" por defecto.
    let run_mode = env::var("PAGETOP_RUN_MODE").unwrap_or("default".into());

    // Inicializa los ajustes.
    let mut settings = Config::default();

    // Lee los ajustes combinando los archivos de configuración disponibles y
    // asigna el modo de ejecución.
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
/// Usar esta macro para obtener el valor de cualquier ajuste global, donde
/// clave y valor son cadenas de caracteres. Devuelve la cadena vacía si no
/// encuentra un ajuste para la clave.
macro_rules! config_get {
    ( $key:expr ) => {
        $crate::config::CONFIG.get_str($key).unwrap_or("".to_string())
    };
}

#[macro_export]
/// Carga los ajustes específicos de tu módulo o aplicación en una estructura
/// similar a [`SETTINGS`] con tipos de variables seguros. Genera un *panic!*
/// en caso de asignaciones no válidas.
macro_rules! config_map {
    ( $COMMENT:expr, $CONF:ident, $TYPE:tt $(, $key:expr => $value:expr)* ) => {
        $crate::doc_comment! {
            concat!($COMMENT),

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
    pub name         : String,
    pub description  : String,
    pub language     : String,
    pub theme        : String,
    pub run_mode     : String,
}

#[derive(Debug, Deserialize)]
pub struct Webserver {
    pub bind_address : String,
    pub bind_port    : u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app          : App,
    pub webserver    : Webserver,
}

config_map!(r#"
Ajustes globales y valores predeterminados para las secciones *\[app\]* y
*\[webserver\]* específicas de PageTop.
"#,
    SETTINGS, Settings,

    // [app]
    "app.name"               => "PageTop Application",
    "app.description"        => "Developed with the amazing PageTop framework.",
    "app.language"           => "en-US",
    "app.theme"              => "Minimal",

    // [webserver]
    "webserver.bind_address" => "localhost",
    "webserver.bind_port"    => 8088
);
