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
//! # Cómo cargar los ajustes de configuración
//!
//! Si tu aplicación requiere archivos de configuración debes crear un directorio *config* al mismo
//! nivel del archivo *Cargo.toml* de tu proyecto (o del ejecutable binario de la aplicación).
//!
//! PageTop se encargará de cargar todos los ajustes de configuración de tu aplicación leyendo los
//! siguientes archivos TOML en este orden (todos los archivos son opcionales):
//!
//! 1. **config/common.toml**, útil para los ajustes comunes a cualquier entorno. Estos valores
//!    podrán ser sobrescritos al fusionar los archivos de configuración restantes.
//!
//! 2. **config/{file}.toml**, donde *{file}* se define con la variable de entorno PAGETOP_RUN_MODE:
//!
//!     * Si no está definida se asumirá *default* por defecto y PageTop intentará cargar el archivo
//!       *config/default.toml* si existe.
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
//! # Cómo usar los ajustes globales de la configuración
//!
//! PageTop incluye un conjunto de ajustes propios ([`Settings`]) accesibles vía [`SETTINGS`] para
//! las secciones reservadas [`[app]`](App), [`[log]`](Log), [`[database]`](Database),
//! [`[webserver]`](Webserver) y [`[dev]`](Dev) de la configuración:
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
//!
//! Los ajustes de configuración siempre son de sólo lectura.
//!
//!
//! # Cómo añadir ajustes de configuración
//!
//! Para proporcionar a tu **aplicación** o **módulo** sus propios ajustes de configuración, añade
//! [*serde*](https://docs.rs/serde) en las dependencias de tu archivo *Cargo.toml*:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! Incluye en tu código una asignación similar a la que usa [`SETTINGS`] para declarar
//! ([`LazyStatic`]) e inicializar tus nuevos ajustes ([`try_into()`]) con tipos seguros y
//! valores predefinidos ([`predefined_settings!`](crate::predefined_settings)):
//!
//! ```
//! use pagetop::prelude::*;
//! use serde::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! pub struct MySettings {
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
//! pub static MY_SETTINGS: LazyStatic<MySettings> = LazyStatic::new(|| {
//!     config::try_into::<MySettings>(predefined_settings!(
//!         // [myapp]
//!         "myapp.name" => "Value Name",
//!         "myapp.width" => "900",
//!         "myapp.height" => "320"
//!     ))
//! });
//! ```
//!
//! Y usa la sintaxis TOML para añadir tu nueva sección `[myapp]` en los archivos de configuración,
//! del mismo modo que `[app]` o `[webserver]` hacen para los ajustes globales ([`Settings`]).
//!
//! Se recomienda inicializar todos los ajustes con valores predefinidos, o utilizar la notación
//! `Option<T>` si van a ser tratados en el código como opcionales.
//!
//!
//! # Cómo usar los nuevos ajustes de configuración
//!
//! ```
//! fn demo() {
//!     println!("{} - {:?}", &MY_SETTINGS.myapp.name, &MY_SETTINGS.myapp.description);
//!     println!("{}", &MY_SETTINGS.myapp.width);
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

use serde::Deserialize;

/// Un *HashMap* con una lista de literales `"clave" => "valor"` para asignar ajustes de
/// configuración predefinidos.
///
/// Ver [`cómo añadir ajustes de configuración`](index.html#cómo-añadir-ajustes-de-configuración).
pub type PredefinedSettings = HashMap<&'static str, &'static str>;

#[macro_export]
/// Macro para crear e inicializar un *HashMap* ([`PredefinedSettings`]) con una lista de literales
/// `"clave" => "valor"` para asignar los ajustes de configuración predefinidos.
///
/// Ver [`cómo añadir ajustes de configuración`](config/index.html#cómo-añadir-ajustes-de-configuración).
macro_rules! predefined_settings {
    ( $($key:literal => $value:literal),* ) => {{
        #[allow(unused_mut)]
        let mut a = $crate::config::PredefinedSettings::new();
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
pub fn try_into<T>(values: PredefinedSettings) -> T
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
