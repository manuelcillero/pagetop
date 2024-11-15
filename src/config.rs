//! Retrieve and apply settings values from configuration files.
//!
//! Carga la configuración de la aplicación en forma de pares `clave = valor` recogidos en archivos
//! [TOML](https://toml.io).
//!
//! La metodología [The Twelve-Factor App](https://12factor.net/es/) define **la configuración de
//! una aplicación como todo lo que puede variar entre despliegues**, diferenciando entre entornos
//! de desarrollo, pre-producción, producción, etc.
//!
//! A veces las aplicaciones guardan configuraciones como constantes en el código, lo que implica
//! una violación de esta metodología. `PageTop` recomienda una **estricta separación entre código y
//! configuración**. La configuración variará en cada tipo de despliegue, y el código no.
//!
//!
//! # Cómo cargar los ajustes de configuración
//!
//! Si tu aplicación requiere archivos de configuración debes crear un directorio *config* al mismo
//! nivel del archivo *Cargo.toml* de tu proyecto (o del ejecutable binario de la aplicación).
//!
//! `PageTop` se encargará de cargar todos los ajustes de configuración de tu aplicación leyendo los
//! siguientes archivos TOML en este orden (todos los archivos son opcionales):
//!
//! 1. **config/common.toml**, útil para los ajustes comunes a cualquier entorno. Estos valores
//!    podrán ser sobrescritos al fusionar los archivos de configuración restantes.
//!
//! 2. **config/{file}.toml**, donde *{file}* se define con la variable de entorno
//!    `PAGETOP_RUN_MODE`:
//!
//!     * Si no está definida se asumirá *default* por defecto y `PageTop` intentará cargar el
//!       archivo *config/default.toml* si existe.
//!
//!     * De esta manera podrás tener diferentes ajustes de configuración para diferentes entornos
//!       de ejecución. Por ejemplo, para *devel.toml*, *staging.toml* o *production.toml*. O
//!       también para *server1.toml* o *server2.toml*. Sólo uno será cargado.
//!
//!     * Normalmente estos archivos suelen ser idóneos para incluir contraseñas o configuración
//!       sensible asociada al entorno correspondiente. Estos archivos no deberían ser publicados en
//!       el repositorio Git por razones de seguridad.
//!
//! 3. **config/local.toml**, para añadir o sobrescribir ajustes de los archivos anteriores.
//!
//!
//! # Cómo añadir ajustes de configuración
//!
//! Para proporcionar a tu **módulo** sus propios ajustes de configuración, añade
//! [*serde*](https://docs.rs/serde) en las dependencias de tu archivo *Cargo.toml* habilitando la
//! característica `derive`:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! Y luego inicializa con la macro [`config_defaults!`](crate::config_defaults) tus ajustes, usando
//! tipos seguros y asignando los valores predefinidos para la estructura asociada:
//!
//! ```
//! use pagetop::prelude::*;
//! use serde::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Settings {
//!    pub myapp: MyApp,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct MyApp {
//!     pub name: String,
//!     pub description: Option<String>,
//!     pub width: u16,
//!     pub height: u16,
//! }
//!
//! config_defaults!(SETTINGS: Settings => [
//!     // [myapp]
//!     "myapp.name" => "Value Name",
//!     "myapp.width" => 900,
//!     "myapp.height" => 320,
//! ]);
//! ```
//!
//! De hecho, así se declaran los ajustes globales de la configuración (ver [`SETTINGS`]).
//!
//! Puedes usar la [sintaxis TOML](https://toml.io/en/v1.0.0#table) para añadir tu nueva sección
//! `[myapp]` en los archivos de configuración, del mismo modo que se añaden `[log]` o `[server]` en
//! los ajustes globales (ver [`Settings`]).
//!
//! Se recomienda inicializar todos los ajustes con valores predefinidos, o utilizar la notación
//! `Option<T>` si van a ser tratados en el código como opcionales.
//!
//! Si no pueden inicializarse correctamente los ajustes de configuración, entonces la aplicación
//! ejecutará un panic! y detendrá la ejecución.
//!
//! Los ajustes de configuración siempre serán de sólo lectura.
//!
//!
//! # Cómo usar tus nuevos ajustes de configuración
//!
//! ```
//! use pagetop::prelude::*;
//! use crate::config;
//!
//! fn global_settings() {
//!     println!("App name: {}", &global::SETTINGS.app.name);
//!     println!("App description: {}", &global::SETTINGS.app.description);
//!     println!("Value of PAGETOP_RUN_MODE: {}", &global::SETTINGS.app.run_mode);
//! }
//!
//! fn package_settings() {
//!     println!("{} - {:?}", &config::SETTINGS.myapp.name, &config::SETTINGS.myapp.description);
//!     println!("{}", &config::SETTINGS.myapp.width);
//! }
//! ```

mod data;
mod de;
mod error;
mod file;
mod path;
mod source;
mod value;

use crate::concat_string;
use crate::config::data::ConfigData;
use crate::config::file::File;

use std::sync::LazyLock;

use std::env;

/// Directorio donde se encuentran los archivos de configuración.
const CONFIG_DIR: &str = "config";

/// Valores originales de la configuración en forma de pares `clave = valor` recogidos de los
/// archivos de configuración.

#[rustfmt::skip]
pub static CONFIG_DATA: LazyLock<ConfigData> = LazyLock::new(|| {
    // Modo de ejecución según la variable de entorno PAGETOP_RUN_MODE. Por defecto 'default'.
    let run_mode = env::var("PAGETOP_RUN_MODE").unwrap_or_else(|_| "default".into());

    // Inicializa los ajustes.
    let mut settings = ConfigData::default();

    // Combina los archivos (opcionales) de configuración y asigna el modo de ejecución.
    settings
        // Primero añade la configuración común a todos los entornos. Por defecto 'common.toml'.
        .merge(
            File::with_name(&concat_string!(CONFIG_DIR, "/common.toml"))
                .required(false)
        ).unwrap()
        // Añade la configuración específica del entorno. Por defecto 'default.toml'.
        .merge(
            File::with_name(&concat_string!(CONFIG_DIR, "/", run_mode, ".toml"))
                .required(false)
        ).unwrap()
        // Añade la configuración local reservada del entorno. Por defecto 'local.default.toml'.
        .merge(
            File::with_name(&concat_string!(CONFIG_DIR, "/local.", run_mode, ".toml"))
                .required(false),
        ).unwrap()
        // Añade la configuración local reservada general. Por defecto 'local.toml'.
        .merge(
            File::with_name(&concat_string!(CONFIG_DIR, "/local.toml"))
                .required(false)
        ).unwrap()
        // Salvaguarda el modo de ejecución.
        .set("app.run_mode", run_mode)
        .unwrap();

    settings
});

#[macro_export]
/// Define un conjunto de ajustes de configuración usando tipos seguros y valores predefinidos.
///
/// Detiene la aplicación con un panic! si no pueden asignarse los ajustes de configuración.
///
/// Ver [`Cómo añadir ajustes de configuración`](config/index.html#cómo-añadir-ajustes-de-configuración).
macro_rules! config_defaults {
    ( $SETTINGS:ident: $Settings:ty => [ $($key:literal => $value:literal),* $(,)? ] ) => {
        #[doc = concat!(
            "Assigned or predefined values for configuration settings associated to the ",
            "[`", stringify!($Settings), "`] type."
        )]
        pub static $SETTINGS: std::sync::LazyLock<$Settings> = std::sync::LazyLock::new(|| {
            let mut settings = $crate::config::CONFIG_DATA.clone();
            $(
                settings.set_default($key, $value).unwrap();
            )*
            match settings.try_into() {
                Ok(s) => s,
                Err(e) => panic!("Error parsing settings: {}", e),
            }
        });
    };
}
