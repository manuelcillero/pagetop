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
//! 2. **config/{file}.toml**, donde *{file}* se define con la variable de entorno
//!    `PAGETOP_RUN_MODE`:
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
//! Y luego inicializa con la macro [`default_settings!`](crate::default_settings) tus ajustes,
//! usando tipos seguros y asignando los valores predefinidos para la estructura asociada:
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
//! default_settings!(
//!     // [myapp]
//!     "myapp.name" => "Value Name",
//!     "myapp.width" => 900,
//!     "myapp.height" => 320,
//! );
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
//!
//! fn global_settings() {
//!     println!("App name: {}", &config::SETTINGS.app.name);
//!     println!("App description: {}", &config::SETTINGS.app.description);
//!     println!("Value of PAGETOP_RUN_MODE: {}", &config::SETTINGS.app.run_mode);
//! }
//!
//! fn package_settings() {
//!     println!("{} - {:?}", &SETTINGS.myapp.name, &SETTINGS.myapp.description);
//!     println!("{}", &SETTINGS.myapp.width);
//! }
//! ```

mod data;
mod de;
mod error;
mod file;
mod path;
mod source;
mod value;

use crate::config::data::ConfigData;
use crate::config::file::File;
use crate::{concat_string, LazyStatic};

use serde::Deserialize;

use std::env;

/// Directorio donde se encuentran los archivos de configuración.
const CONFIG_DIR: &str = "config";

/// Valores originales de la configuración en forma de pares `clave = valor` recogidos de los
/// archivos de configuración.

#[rustfmt::skip]
pub static CONFIG: LazyStatic<ConfigData> = LazyStatic::new(|| {
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
        // Añade la configuración local reservada del entorno. Por defecto 'default.local.toml'.
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
macro_rules! default_settings {
    ( $($key:literal => $value:literal),* $(,)? ) => {
        #[doc = concat!(
            "Assigned or predefined values for configuration settings associated to the ",
            "[`Settings`] type."
        )]
        pub static SETTINGS: $crate::LazyStatic<Settings> = $crate::LazyStatic::new(|| {
            let mut settings = $crate::config::CONFIG.clone();
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

#[derive(Debug, Deserialize)]
/// Configuration settings for the [`[app]`](App), [`[database]`](Database), [`[dev]`](Dev),
/// [`[log]`](Log), and [`[server]`](Server) sections (see [`SETTINGS`]).
pub struct Settings {
    pub app: App,
    pub database: Database,
    pub dev: Dev,
    pub log: Log,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
/// Section `[app]` of the configuration settings.
///
/// See [`Settings`].
pub struct App {
    /// El nombre de la aplicación.
    /// Por defecto: *"PageTop App"*.
    pub name: String,
    /// Una descripción breve de la aplicación.
    /// Por defecto: *"Developed with the awesome PageTop framework."*.
    pub description: String,
    /// Tema predeterminado.
    /// Por defecto: *"Default"*.
    pub theme: String,
    /// Idioma (localización) predeterminado.
    /// Por defecto: *"en-US"*.
    pub language: String,
    /// Dirección predeterminada para el texto: *"ltr"* (de izquierda a derecha), *"rtl"* (de
    /// derecha a izquierda) o *"auto"*.
    /// Por defecto: *"ltr"*.
    pub direction: String,
    /// Rótulo de texto ASCII al arrancar: *"Off"*, *"Slant"*, *"Small"*, *"Speed"* o *"Starwars"*.
    /// Por defecto: *"Slant"*.
    pub startup_banner: String,
    /// Por defecto: según variable de entorno `PAGETOP_RUN_MODE`, o *"default"* si no lo está.
    pub run_mode: String,
}

#[derive(Debug, Deserialize)]
/// Section `[database]` of the configuration settings.
///
/// See [`Settings`].
pub struct Database {
    /// Tipo de base de datos: *"mysql"*, *"postgres"* ó *"sqlite"*.
    /// Por defecto: *""*.
    pub db_type: String,
    /// Nombre (para mysql/postgres) o referencia (para sqlite) de la base de datos.
    /// Por defecto: *""*.
    pub db_name: String,
    /// Usuario de conexión a la base de datos (para mysql/postgres).
    /// Por defecto: *""*.
    pub db_user: String,
    /// Contraseña para la conexión a la base de datos (para mysql/postgres).
    /// Por defecto: *""*.
    pub db_pass: String,
    /// Servidor de conexión a la base de datos (para mysql/postgres).
    /// Por defecto: *"localhost"*.
    pub db_host: String,
    /// Puerto de conexión a la base de datos, normalmente 3306 (para mysql) ó 5432 (para postgres).
    /// Por defecto: *0*.
    pub db_port: u16,
    /// Número máximo de conexiones habilitadas.
    /// Por defecto: *5*.
    pub max_pool_size: u32,
}

#[derive(Debug, Deserialize)]
/// Section `[dev]` of the configuration settings.
///
/// See [`Settings`].
pub struct Dev {
    /// Los archivos estáticos requeridos por la aplicación se integran de manera predeterminada en
    /// el binario ejecutable. Sin embargo, durante el desarrollo puede resultar útil servir estos
    /// archivos desde su propio directorio para evitar recompilar cada vez que se modifican. En
    /// este caso bastaría con indicar la ruta completa al directorio raíz del proyecto.
    /// Por defecto: *""*.
    pub pagetop_project_dir: String,
}

#[derive(Debug, Deserialize)]
/// Section `[log]` of the configuration settings.
///
/// See [`Settings`].
pub struct Log {
    /// Filtro, o combinación de filtros separados por coma, para la traza de ejecución: *"Error"*,
    /// *"Warn"*, *"Info"*, *"Debug"* o *"Trace"*.
    /// Por ejemplo: "Error,actix_server::builder=Info,tracing_actix_web=Debug".
    /// Por defecto: *"Info"*.
    pub tracing: String,
    /// Muestra la traza en el terminal (*"Stdout"*) o queda registrada en archivos con rotación
    /// *"Daily"*, *"Hourly"*, *"Minutely"* o *"Endless"*.
    /// Por defecto: *"Stdout"*.
    pub rolling: String,
    /// Directorio para los archivos de traza (si `rolling` != *"Stdout"*).
    /// Por defecto: *"log"*.
    pub path: String,
    /// Prefijo para los archivos de traza (si `rolling` != *"Stdout"*).
    /// Por defecto: *"tracing.log"*.
    pub prefix: String,
    /// Presentación de las trazas. Puede ser *"Full"*, *"Compact"*, *"Pretty"* o *"Json"*.
    /// Por defecto: *"Full"*.
    pub format: String,
}

#[derive(Debug, Deserialize)]
/// Section `[server]` of the configuration settings.
///
/// See [`Settings`].
pub struct Server {
    /// Dirección del servidor web.
    /// Por defecto: *"localhost"*.
    pub bind_address: String,
    /// Puerto del servidor web.
    /// Por defecto: *8088*.
    pub bind_port: u16,
    /// Duración en segundos para la sesión (0 indica "hasta que se cierre el navegador").
    /// Por defecto: *604800* (7 días).
    pub session_lifetime: i64,
}

default_settings!(
    // [app]
    "app.name"                => "PageTop App",
    "app.description"         => "Developed with the awesome PageTop framework.",
    "app.theme"               => "Default",
    "app.language"            => "en-US",
    "app.direction"           => "ltr",
    "app.startup_banner"      => "Slant",

    // [database]
    "database.db_type"        => "",
    "database.db_name"        => "",
    "database.db_user"        => "",
    "database.db_pass"        => "",
    "database.db_host"        => "localhost",
    "database.db_port"        => 0,
    "database.max_pool_size"  => 5,

    // [dev]
    "dev.pagetop_project_dir" => "",

    // [log]
    "log.tracing"             => "Info",
    "log.rolling"             => "Stdout",
    "log.path"                => "log",
    "log.prefix"              => "tracing.log",
    "log.format"              => "Full",

    // [server]
    "server.bind_address"     => "localhost",
    "server.bind_port"        => 8088,
    "server.session_lifetime" => 604800,
);
