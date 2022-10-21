//! Gestión de la configuración.
//!
//! Carga la configuración de la aplicación en forma de pares `clave = valor` recogidos en archivos
//! [TOML](https://toml.io).
//!
//! La metodología [The Twelve-Factor App](https://12factor.net/es/) define **la configuración de
//! una aplicación como todo lo que puede variar entre despliegues**, diferenciando entre entornos
//! de desarrollo, pre-producción, producción, etc.
//!
//! A veces las aplicaciones guardan configuraciones como constantes en el código, lo que implica
//! una violación de esta metodología. PageTop recomienda una **estricta separación entre código y
//! configuración**. La configuración variará en cada tipo de despliegue, y el código no.
//!
//!
//! # Cómo usar archivos de configuración
//!
//! Si tu aplicación requiere archivos de configuración debes crear un directorio llamado *config*
//! al mismo nivel del archivo *Cargo.toml* de tu proyecto (o del ejecutable binario de la
//! aplicación).
//!
//! Guarda la configuración usando archivos TOML asumiendo el siguiente orden de lectura secuencial
//! (todos los archivos son opcionales):
//!
//! 1. **config/common.toml**, útil para los ajustes comunes para cualquier entorno. Estos valores
//!    podrán ser sobrescritos al fusionar los archivos de configuración siguientes.
//!
//! 2. **config/{archivo}.toml**, donde *{archivo}* puede definirse mediante la variable de entorno
//!    PAGETOP_RUN_MODE:
//!
//!     * Si no lo está, se asumirá *default* por defecto, y PageTop cargará el archivo de
//!       configuración *config/default.toml* si existe.
//!
//!     * De esta manera se pueden tener diferentes ajustes de configuración para diferentes
//!       entornos de ejecución. Por ejemplo, para *devel.toml*, *staging.toml* o *production.toml*.
//!       O también para *server1.toml* o *server2.toml*. Sólo uno será cargado.
//!
//!     * Normalmente estos archivos suelen ser idóneos para incluir contraseñas o configuración
//!       sensible asociada al entorno correspondiente. Estos archivos no deberían ser publicados en
//!       el repositorio Git por razones de seguridad.
//!
//! 3. **config/local.toml**, para añadir o sobrescribir ajustes previos.
//!
//! Los ajustes de configuración siempre son de sólo lectura.
//!
//!
//! # Cómo añadir ajustes de configuración
//!
//! Puedes usar la sintaxis de TOML para crear nuevas secciones en los archivos de configuración,
//! del mismo modo que *\[app\]* o *\[webserver\]* existen en la configuración predeterminada (ver
//! [`SETTINGS`]).
//!
//! Para cargar y usar esta nueva configuración desde tu **aplicación** o **módulo** tienes que
//! incluir [*serde*](https://docs.rs/serde) en las dependencias de tu archivo *Cargo.toml*:
//!
//! ```
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! y añadir en tu código una declaración similar a la que utiliza [`SETTINGS`] para instanciar
//! ([`LazyStatic`]) e inicializar ([`init_settings()`]) los nuevos ajustes con tipos seguros y
//! valores predefinidos ([`predefined_settings!`](crate::predefined_settings)):
//!
//! ```
//! use pagetop::prelude::*;
//! use serde::Deserialize;
//! use std::fmt::Debug;
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Id {
//!     pub name: String,
//!     pub desc: String,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Size {
//!     pub width: u16,
//!     pub height: u16,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct MyApp {
//!    pub id: Id,
//!    pub size: Size,
//! }
//!
//! pub static MY_APP: LazyStatic<MyApp> = LazyStatic::new(|| {
//!     init_settings::<MyApp>(predefined_settings!(
//!         // [id]
//!         "id.name" => "Value Name",
//!         "id.desc" => "Value Description",
//!
//!         // [size]
//!         "size.width" => "900",
//!         "size.height" => "320"
//!     ))
//! });
//! ```
//!
//! Es importante inicializar todos los ajustes con valores predefinidos (aunque sea con valores
//! vacíos como *""* o *"0"*, por ejemplo) para evitar *panic!*'s no deseados.
//!
//!
//! # Cómo obtener los valores de configuración
//!
//! Basta con acceder directamente a la variable estática. Por ejemplo, con [`SETTINGS`]:
//!
//! ```
//! use pagetop::prelude::*;
//!
//! fn demo() {
//!     println!("App name: {}", &SETTINGS.app.name);
//!     println!("App description: {}", &SETTINGS.app.description);
//!     println!("Value of PAGETOP_RUN_MODE: {}", &SETTINGS.app.run_mode);
//! }
//! ```
//! O a valores específicos de la configuración de tu **aplicación** o **módulo**:
//!
//! ```
//! fn demo() {
//!     println!("{}", &MY_APP.id.name);
//!     println!("{}", &MY_APP.size.width);
//! }
//! ```

mod data;
mod de;
mod error;
mod file;
mod path;
mod source;
mod value;

use crate::LazyStatic;

use crate::config::data::ConfigData;
use crate::config::file::File;

use std::collections::HashMap;
use std::env;
use std::fmt::Debug;

use serde::Deserialize;

/// Un *HashMap* con una lista de literales `"clave" => "valor"` para asignar ajustes de
/// configuración predefinidos.
///
/// Ver [`cómo añadir ajustes de configuración`](index.html#cómo-añadir-ajustes-de-configuración).
pub type PredefinedSettings = HashMap<&'static str, &'static str>;

#[macro_export]
/// Macro para crear e inicializar un *HashMap* ([`PredefinedSettings`]) con una lista de literales
/// `"clave" => "valor"` para asignar ajustes de configuración predefinidos.
///
/// Ver [`cómo añadir ajustes de configuración`](config/index.html#cómo-añadir-ajustes-de-configuración).
macro_rules! predefined_settings {
    ( $($key:literal => $value:literal),* ) => {{
        #[allow(unused_mut)]
        let mut a = PredefinedSettings::new();
        $(
            a.insert($key, $value);
        )*
        a
    }};
}

/// Directorio donde se encuentran los archivos de configuración.
const CONFIG_DIR: &str = "config";

/// Todos los valores originales de la configuración en forma de pares `clave = valor` recogidos de
/// los archivos de configuración.
static CONFIG_DATA: LazyStatic<ConfigData> = LazyStatic::new(|| {
    // Modo de ejecución según la variable de entorno PAGETOP_RUN_MODE. Por defecto 'default'.
    let run_mode = env::var("PAGETOP_RUN_MODE").unwrap_or_else(|_| "default".into());

    // Inicializa los ajustes.
    let mut settings = ConfigData::default();

    // Combina los archivos de configuración y asigna el modo de ejecución.
    settings
        // Primero añade la configuración común a todos los entornos. Opcional.
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

/// Carga ajustes con tipos seguros y valores predefinidos para tu aplicación o módulo en una
/// estructura similiar a [`SETTINGS`].
///
/// Ver [`Cómo añadir ajustes de configuración`](index.html#cómo-añadir-ajustes-de-configuración).
pub fn init_settings<T>(values: PredefinedSettings) -> T
where
    T: Deserialize<'static>,
{
    let mut settings = CONFIG_DATA.clone();
    for (key, value) in values.iter() {
        settings.set_default(*key, *value).unwrap();
    }
    match settings.try_into() {
        Ok(c) => c,
        Err(e) => panic!("Error parsing settings: {}", e),
    }
}

#[derive(Debug, Deserialize)]
/// Sección *\[app\]* de la configuración global.
pub struct App {
    /// Valor predefinido: *"PageTop Application"*
    pub name: String,
    /// Valor predefinido: *"Developed with the amazing PageTop framework."*
    pub description: String,
    /// Valor predefinido: *"Bootsier"*
    pub theme: String,
    /// Valor predefinido: *"en-US"*
    pub language: String,
    /// Valor predefinido: *"ltr"*
    pub direction: String,
    /// Valor predefinido: *"Slant"*
    pub startup_banner: String,
    /// Valor predefinido: según variable de entorno PAGETOP_RUN_MODE, o *"default"* si no lo está
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Sección *\[log\]* de la configuración global.
pub struct Log {
    /// Valor predefinido: *"Info"*
    pub tracing: String,
    /// Valor predefinido: *"Stdout"*
    pub rolling: String,
    /// Valor predefinido: *"log"*
    pub path: String,
    /// Valor predefinido: *"tracing.log"*
    pub prefix: String,
    /// Valor predefinido: *"Full"*
    pub format: String,
}

#[derive(Debug, Deserialize)]
/// Sección *\[database\]* de la configuración global.
pub struct Database {
    /// Valor predefinido: *""*
    pub db_type: String,
    /// Valor predefinido: *""*
    pub db_name: String,
    /// Valor predefinido: *""*
    pub db_user: String,
    /// Valor predefinido: *""*
    pub db_pass: String,
    /// Valor predefinido: *"localhost"*
    pub db_host: String,
    /// Valor predefinido: *"0"*
    pub db_port: u16,
    /// Valor predefinido: *"5"*
    pub max_pool_size: u32,
}

#[derive(Debug, Deserialize)]
/// Sección *\[webserver\]* de la configuración global.
pub struct Webserver {
    /// Valor predefinido: *"localhost"*
    pub bind_address: String,
    /// Valor predefinido: *"8088"*
    pub bind_port: u16,
}

#[derive(Debug, Deserialize)]
/// Sección *\[dev\]* de la configuración global.
pub struct Dev {
    /// Valor predefinido: *""*
    pub static_files: String,
}

#[derive(Debug, Deserialize)]
/// Ajustes globales para las secciones *\[app\]*, *\[log\]*, *\[database\]*, *\[webserver\]* y
/// *\[dev\]* requeridas por PageTop (ver [`SETTINGS`]).
pub struct Settings {
    pub app: App,
    pub log: Log,
    pub database: Database,
    pub webserver: Webserver,
    pub dev: Dev,
}

/// Instancia los ajustes globales para la estructura [`Settings`].
///
/// Ver [`Cómo obtener los valores de configuración`](index.html#cómo-obtener-los-valores-de-configuración).
pub static SETTINGS: LazyStatic<Settings> = LazyStatic::new(|| {
    init_settings::<Settings>(predefined_settings!(
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
    ))
});
