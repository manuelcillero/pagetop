//! Global settings, functions and macro helpers.

use crate::{config_defaults, trace};

use serde::Deserialize;

use std::io;
use std::path::PathBuf;

// *************************************************************************************************
// SETTINGS.
// *************************************************************************************************

#[derive(Debug, Deserialize)]
/// Configuration settings for global [`[app]`](App), [`[dev]`](Dev), [`[log]`](Log), and
/// [`[server]`](Server) sections (see [`SETTINGS`]).
pub struct Settings {
    pub app: App,
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
    /// Por defecto: *"My App"*.
    pub name: String,
    /// Una descripción breve de la aplicación.
    /// Por defecto: *"Developed with the amazing PageTop framework."*.
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

config_defaults!(SETTINGS: Settings => [
    // [app]
    "app.name"                => "My App",
    "app.description"         => "Developed with the amazing PageTop framework.",
    "app.theme"               => "Default",
    "app.language"            => "en-US",
    "app.direction"           => "ltr",
    "app.startup_banner"      => "Slant",

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
    "server.session_lifetime" => 604_800,
]);

// *************************************************************************************************
// FUNCTIONS HELPERS.
// *************************************************************************************************

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

// *************************************************************************************************
// MACRO HELPERS.
// *************************************************************************************************

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
