//! Carga las opciones de configuración.
//!
//! Estos ajustes se obtienen de archivos [TOML](https://toml.io) como pares `clave = valor` que se
//! mapean a estructuras **fuertemente tipadas** y valores predefinidos.
//!
//! Siguiendo la metodología [Twelve-Factor App](https://12factor.net/config), `PageTop` separa el
//! **código** de la **configuración**, lo que permite tener configuraciones diferentes para cada
//! despliegue, como *dev*, *staging* o *production*, sin modificar el código fuente.
//!
//!
//! # Orden de carga
//!
//! Si tu aplicación necesita archivos de configuración, crea un directorio `config` en la raíz del
//! proyecto, al mismo nivel que el archivo *Cargo.toml* o que el binario de la aplicación.
//!
//! `PageTop` carga en este orden, y siempre de forma opcional, los siguientes archivos TOML:
//!
//! 1. **config/common.toml**, para ajustes comunes a todos los entornos. Este enfoque simplifica el
//!    mantenimiento al centralizar los valores de configuración comunes.
//!
//! 2. **config/{rm}.toml**, donde `{rm}` es el valor de la variable de entorno `PAGETOP_RUN_MODE`:
//!
//!    * Si `PAGETOP_RUN_MODE` no está definida, se asume el valor `default`, y `PageTop` intentará
//!      cargar *config/default.toml* si el archivo existe.
//!
//!    * Útil para definir configuraciones específicas por entorno, garantizando que cada uno (p.e.
//!      *dev*, *staging* o *production*) disponga de sus propias opciones, como claves de API,
//!      URLs o ajustes de rendimiento, sin afectar a los demás.
//!
//! 3. **config/local.{rm}.toml**, útil para configuraciones locales específicas de la máquina o de
//!    la ejecución:
//!
//!    * Permite añadir o sobrescribir ajustes propios del entorno. Por ejemplo, `local.dev.toml`
//!      para desarrollo o `local.production.toml` para retoques en producción.
//!
//!    * Facilita que cada desarrollador adapte la configuración a su equipo en un entorno dado. Por
//!      lo general no se comparte ni se sube al sistema de control de versiones.
//!
//! 4. **config/local.toml**, para ajustes locales válidos en cualquier entorno, ideal para cambios
//!    rápidos o valores temporales que no dependan de un entorno concreto.
//!
//! Los archivos se combinan en el orden anterior, cada archivo sobrescribe a los anteriores en caso
//! de conflicto.
//!
//!
//! # Cómo añadir opciones de configuración a tu código
//!
//! Añade [*serde*](https://docs.rs/serde) en tu archivo *Cargo.toml* con la *feature* `derive`:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! Y usa la macro [`include_config!`](crate::include_config) para inicializar tus ajustes en una
//! estructura con tipos seguros. Por ejemplo:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//! use serde::Deserialize;
//!
//! include_config!(SETTINGS: Settings => [
//!     // [myapp]
//!     "myapp.name"   => "Value Name",
//!     "myapp.width"  => 900,
//!     "myapp.height" => 320,
//! ]);
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Settings {
//!     pub myapp: MyApp,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct MyApp {
//!     pub name: String,
//!     pub description: Option<String>,
//!     pub width: u16,
//!     pub height: u16,
//! }
//! ```
//!
//! De esta forma estás añadiendo una nueva sección `[myapp]` a la configuración, igual que existen
//! `[app]` o `[server]` en las opciones globales de [`Settings`](crate::global::Settings).
//!
//! Se recomienda proporcionar siempre valores por defecto o usar `Option<T>` para los ajustes
//! opcionales.
//!
//! Si la configuración no se inicializa correctamente, la aplicación lanzará *panic* y detendrá la
//! ejecución.
//!
//! Las estructuras de configuración son de **sólo lectura** durante la ejecución.
//!
//!
//! # Usando tus opciones de configuración
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//! use crate::config;
//!
//! fn global_settings() {
//!     println!("Nombre de la app: {}", &global::SETTINGS.app.name);
//!     println!("Descripción: {}",      &global::SETTINGS.app.description);
//!     println!("Run mode: {}",         &global::SETTINGS.app.run_mode);
//! }
//!
//! fn extension_settings() {
//!     println!("{} - {:?}", &config::SETTINGS.myapp.name, &config::SETTINGS.myapp.description);
//!     println!("{}", &config::SETTINGS.myapp.width);
//! }
//! ```

use config::builder::DefaultState;
use config::{Config, ConfigBuilder, File};

use std::env;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

// Nombre del directorio de configuración por defecto.
const DEFAULT_CONFIG_DIR: &str = "config";

// Modo de ejecución por defecto.
const DEFAULT_RUN_MODE: &str = "default";

/// Valores originales cargados desde los archivos de configuración como pares `clave = valor`.
pub static CONFIG_VALUES: LazyLock<ConfigBuilder<DefaultState>> = LazyLock::new(|| {
    // Determina el directorio de configuración:
    // - Usa CONFIG_DIR si está definido en el entorno (p.e.: CONFIG_DIR=/etc/myapp ./myapp).
    // - Si no, intenta DEFAULT_CONFIG_DIR dentro del proyecto (en CARGO_MANIFEST_DIR).
    // - Si nada de esto aplica, entonces usa DEFAULT_CONFIG_DIR relativo al ejecutable.
    let config_dir: PathBuf = if let Ok(env_dir) = env::var("CONFIG_DIR") {
        env_dir.into()
    } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let manifest_config = Path::new(&manifest_dir).join(DEFAULT_CONFIG_DIR);
        if manifest_config.exists() {
            manifest_config
        } else {
            DEFAULT_CONFIG_DIR.into()
        }
    } else {
        DEFAULT_CONFIG_DIR.into()
    };

    // Determina el modo de ejecución según la variable de entorno PAGETOP_RUN_MODE. Por defecto usa
    // DEFAULT_RUN_MODE si no está definida (p.e.: PAGETOP_RUN_MODE=production ./myapp).
    let rm = env::var("PAGETOP_RUN_MODE").unwrap_or_else(|_| DEFAULT_RUN_MODE.into());

    Config::builder()
        // 1. Configuración común para todos los entornos (common.toml).
        .add_source(File::from(config_dir.join("common.toml")).required(false))
        // 2. Configuración específica del entorno (p.e.: default.toml, production.toml).
        .add_source(File::from(config_dir.join(format!("{rm}.toml"))).required(false))
        // 3. Configuración local reservada para cada entorno (p.e.: local.default.toml).
        .add_source(File::from(config_dir.join(format!("local.{rm}.toml"))).required(false))
        // 4. Configuración local común (local.toml).
        .add_source(File::from(config_dir.join("local.toml")).required(false))
        // Guarda el modo de ejecución explícitamente.
        .set_override("app.run_mode", rm)
        .expect("Failed to set application run mode")
});

#[macro_export]
macro_rules! include_config {
    ( $SETTINGS:ident : $Settings:ty => [ $( $key:expr => $value:expr ),* $(,)? ] ) => {
        /// Valores asignados o predefinidos para la configuración de [`$Settings`].
        pub static $SETTINGS: std::sync::LazyLock<$Settings> = std::sync::LazyLock::new(|| {
            let mut settings = $crate::config::CONFIG_VALUES.clone();
            $(
                settings = settings.set_default($key, $value).unwrap();
            )*
            settings
                .build()
                .expect(concat!("Failed to build config for ", stringify!($Settings)))
                .try_deserialize::<$Settings>()
                .expect(concat!("Error parsing settings for ", stringify!($Settings)))
        });
    };
}
