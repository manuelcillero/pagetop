//! Useful functions and macros.

pub mod config;

use crate::trace;

use std::io;
use std::path::PathBuf;

// USEFUL FUNCTIONS ********************************************************************************

pub enum TypeInfo {
    FullName,
    ShortName,
    NameFrom(isize),
    NameTo(isize),
    PartialName(isize, isize),
}

impl TypeInfo {
    pub fn of<T: ?Sized>(&self) -> &'static str {
        let type_name = std::any::type_name::<T>();
        match self {
            TypeInfo::FullName => type_name,
            TypeInfo::ShortName => Self::partial(type_name, -1, None),
            TypeInfo::NameFrom(start) => Self::partial(type_name, *start, None),
            TypeInfo::NameTo(end) => Self::partial(type_name, 0, Some(*end)),
            TypeInfo::PartialName(start, end) => Self::partial(type_name, *start, Some(*end)),
        }
    }

    fn partial(type_name: &'static str, start: isize, end: Option<isize>) -> &'static str {
        let maxlen = type_name.len();
        let mut segments = Vec::new();
        let mut segment_start = 0; // Start position of the current segment.
        let mut angle_brackets = 0; // Counter for tracking '<' and '>'.
        let mut previous_char = '\0'; // Initializes to a null character, no previous character.

        for (idx, c) in type_name.char_indices() {
            match c {
                ':' if angle_brackets == 0 => {
                    if previous_char == ':' {
                        if segment_start < idx - 1 {
                            segments.push((segment_start, idx - 1)); // Do not include last '::'.
                        }
                        segment_start = idx + 1; // Next segment starts after '::'.
                    }
                }
                '<' => angle_brackets += 1,
                '>' => angle_brackets -= 1,
                _ => {}
            }
            previous_char = c;
        }

        // Include the last segment if there's any.
        if segment_start < maxlen {
            segments.push((segment_start, maxlen));
        }

        // Calculates the start position.
        let start_pos = segments
            .get(if start >= 0 {
                start as usize
            } else {
                segments.len() - start.unsigned_abs()
            })
            .map_or(0, |&(s, _)| s);

        // Calculates the end position.
        let end_pos = segments
            .get(if let Some(end) = end {
                if end >= 0 {
                    end as usize
                } else {
                    segments.len() - end.unsigned_abs()
                }
            } else {
                segments.len() - 1
            })
            .map_or(maxlen, |&(_, e)| e);

        // Returns the partial string based on the calculated positions.
        &type_name[start_pos..end_pos]
    }
}

/// Calculates the absolute directory given a root path and a relative path.
///
/// # Arguments
///
/// * `root_path` - A string slice that holds the root path.
/// * `relative_path` - A string slice that holds the relative path.
///
/// # Returns
///
/// * `Ok` - If the operation is successful, returns the absolute directory as a `String`.
/// * `Err` - If an I/O error occurs, returns an `io::Error`.
///
/// # Errors
///
/// This function will return an error if:
/// - The root path or relative path are invalid.
/// - There is an issue with file system operations, such as reading the directory.
///
/// # Examples
///
/// ```
/// let root = "/home/user";
/// let relative = "documents";
/// let abs_dir = absolute_dir(root, relative).unwrap();
/// println!("{}", abs_dir);
/// ```
pub fn absolute_dir(
    root_path: impl Into<String>,
    relative_path: impl Into<String>,
) -> Result<String, io::Error> {
    let root_path = PathBuf::from(root_path.into());
    let full_path = root_path.join(relative_path.into());
    let absolute_dir = full_path.to_string_lossy().into();

    if !full_path.is_absolute() {
        let message = format!("Path \"{absolute_dir}\" is not absolute");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
    }

    if !full_path.exists() {
        let message = format!("Path \"{absolute_dir}\" does not exist");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::NotFound, message));
    }

    if !full_path.is_dir() {
        let message = format!("Path \"{absolute_dir}\" is not a directory");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
    }

    Ok(absolute_dir)
}

// USEFUL MACROS ***********************************************************************************

#[macro_export]
/// Macro para construir grupos de pares clave-valor.
///
/// ```rust#ignore
/// let args = kv![
///     "userName" => "Roberto",
///     "photoCount" => 3,
///     "userGender" => "male",
/// ];
/// ```
macro_rules! kv {
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key.into(), $value.into());
        )*
        a
    }};
}

#[macro_export]
/// Define un conjunto de ajustes de configuración usando tipos seguros y valores predefinidos.
///
/// Detiene la aplicación con un panic! si no pueden asignarse los ajustes de configuración.
///
/// Carga la configuración de la aplicación en forma de pares `clave = valor` recogidos en archivos
/// [TOML](https://toml.io).
///
/// La metodología [The Twelve-Factor App](https://12factor.net/es/) define **la configuración de
/// una aplicación como todo lo que puede variar entre despliegues**, diferenciando entre entornos
/// de desarrollo, pre-producción, producción, etc.
///
/// A veces las aplicaciones guardan configuraciones como constantes en el código, lo que implica
/// una violación de esta metodología. `PageTop` recomienda una **estricta separación entre código y
/// configuración**. La configuración variará en cada tipo de despliegue, y el código no.
///
///
/// # Cómo cargar los ajustes de configuración
///
/// Si tu aplicación requiere archivos de configuración debes crear un directorio *config* al mismo
/// nivel del archivo *Cargo.toml* de tu proyecto (o del ejecutable binario de la aplicación).
///
/// `PageTop` se encargará de cargar todos los ajustes de configuración de tu aplicación leyendo los
/// siguientes archivos TOML en este orden (todos los archivos son opcionales):
///
/// 1. **config/common.toml**, útil para los ajustes comunes a cualquier entorno. Estos valores
///    podrán ser sobrescritos al fusionar los archivos de configuración restantes.
///
/// 2. **config/{file}.toml**, donde *{file}* se define con la variable de entorno
///    `PAGETOP_RUN_MODE`:
///
///     * Si no está definida se asumirá *default* por defecto y `PageTop` intentará cargar el
///       archivo *config/default.toml* si existe.
///
///     * De esta manera podrás tener diferentes ajustes de configuración para diferentes entornos
///       de ejecución. Por ejemplo, para *devel.toml*, *staging.toml* o *production.toml*. O
///       también para *server1.toml* o *server2.toml*. Sólo uno será cargado.
///
///     * Normalmente estos archivos suelen ser idóneos para incluir contraseñas o configuración
///       sensible asociada al entorno correspondiente. Estos archivos no deberían ser publicados en
///       el repositorio Git por razones de seguridad.
///
/// 3. **config/local.toml**, para añadir o sobrescribir ajustes de los archivos anteriores.
///
///
/// # Cómo añadir ajustes de configuración
///
/// Para proporcionar a tu **módulo** sus propios ajustes de configuración, añade
/// [*serde*](https://docs.rs/serde) en las dependencias de tu archivo *Cargo.toml* habilitando la
/// característica `derive`:
///
/// ```toml
/// [dependencies]
/// serde = { version = "1.0", features = ["derive"] }
/// ```
///
/// Y luego inicializa con la macro [`static_config!`](crate::static_config) tus ajustes, usando
/// tipos seguros y asignando los valores predefinidos para la estructura asociada:
///
/// ```
/// use pagetop::prelude::*;
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// pub struct Settings {
///    pub myapp: MyApp,
/// }
///
/// #[derive(Debug, Deserialize)]
/// pub struct MyApp {
///     pub name: String,
///     pub description: Option<String>,
///     pub width: u16,
///     pub height: u16,
/// }
///
/// static_config!(SETTINGS: Settings => [
///     // [myapp]
///     "myapp.name" => "Value Name",
///     "myapp.width" => 900,
///     "myapp.height" => 320,
/// ]);
/// ```
///
/// De hecho, así se declaran los ajustes globales de la configuración (ver [`SETTINGS`]).
///
/// Puedes usar la [sintaxis TOML](https://toml.io/en/v1.0.0#table) para añadir tu nueva sección
/// `[myapp]` en los archivos de configuración, del mismo modo que se añaden `[log]` o `[server]` en
/// los ajustes globales (ver [`Settings`]).
///
/// Se recomienda inicializar todos los ajustes con valores predefinidos, o utilizar la notación
/// `Option<T>` si van a ser tratados en el código como opcionales.
///
/// Si no pueden inicializarse correctamente los ajustes de configuración, entonces la aplicación
/// ejecutará un panic! y detendrá la ejecución.
///
/// Los ajustes de configuración siempre serán de sólo lectura.
///
///
/// # Cómo usar tus nuevos ajustes de configuración
///
/// ```
/// use pagetop::prelude::*;
/// use crate::config;
///
/// fn global_settings() {
///     println!("App name: {}", &global::SETTINGS.app.name);
///     println!("App description: {}", &global::SETTINGS.app.description);
///     println!("Value of PAGETOP_RUN_MODE: {}", &global::SETTINGS.app.run_mode);
/// }
///
/// fn package_settings() {
///     println!("{} - {:?}", &config::SETTINGS.myapp.name, &config::SETTINGS.myapp.description);
///     println!("{}", &config::SETTINGS.myapp.width);
/// }
/// ```
macro_rules! static_config {
    ( $SETTINGS:ident: $Settings:ty => [ $($key:literal => $value:literal),* $(,)? ] ) => {
        #[doc = concat!(
            "Assigned or predefined values for configuration settings associated to the ",
            "[`", stringify!($Settings), "`] type."
        )]
        pub static $SETTINGS: std::sync::LazyLock<$Settings> = std::sync::LazyLock::new(|| {
            let mut settings = $crate::util::config::CONFIG_DATA.clone();
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
