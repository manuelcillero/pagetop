use crate::{AutoDefault, CowStr, builder_fn};

use std::fmt::{self, Write as _};

/// Representa una ruta como un *path* inicial más una lista opcional de parámetros.
///
/// Modela rutas del estilo `/path/to/resource?foo=bar&debug` o `https://example.com/path?foo=bar`,
/// pensadas para usarse en atributos HTML como `href`, `action` o `src`.
///
/// `RoutePath` no valida ni interpreta la estructura del *path*; simplemente concatena los
/// parámetros de consulta sobre el valor proporcionado. El *path* tampoco se codifica: se asume
/// que ya es válido (rutas propias de la aplicación, normalmente literales o formadas a partir de
/// identificadores conocidos).
///
/// # Codificación de los valores
///
/// El método [`with_param()`](Self::with_param) codifica el **valor** (no la clave) según RFC 3986
/// antes de insertarlo. Así, cualquier valor (como una búsqueda de usuario, un destino con su
/// propia *query string*, etc.) puede pasarse tal cual, sin que quien llama tenga que codificarlo
/// primero.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Ruta relativa con parámetros y un *flag* sin valor.
/// let route = RoutePath::new("/search")
///     .with_param("q", "rust")
///     .with_param("page", "2")
///     .with_flag("debug");
/// assert_eq!(route.to_string(), "/search?q=rust&page=2&debug");
///
/// // Ruta absoluta a un recurso externo.
/// let external = RoutePath::new("https://example.com/export").with_param("format", "csv");
/// assert_eq!(external.to_string(), "https://example.com/export?format=csv");
///
/// // Un valor con espacios o símbolos se codifica automáticamente.
/// let search = RoutePath::new("/search").with_param("q", "rust & htmx");
/// assert_eq!(search.to_string(), "/search?q=rust%20%26%20htmx");
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct RoutePath {
    /// *Path* inicial sobre el que se añadirán los parámetros.
    ///
    /// Puede ser relativo (p. ej. `/about`) o una ruta completa (`https://example.com/about`).
    /// `RoutePath` no realiza ninguna validación ni normalización.
    path: CowStr,

    /// Conjunto de parámetros asociados a la ruta.
    ///
    /// Cada clave es única y se mantiene el orden de inserción. El valor vacío se utiliza para
    /// representar *flags* sin valor explícito (por ejemplo `?debug`).
    query: indexmap::IndexMap<String, String>,
}

impl RoutePath {
    /// Crea un `RoutePath` a partir de un *path* inicial.
    ///
    /// Por ejemplo: `RoutePath::new("/about")`.
    pub fn new(path: impl Into<CowStr>) -> Self {
        Self {
            path: path.into(),
            query: indexmap::IndexMap::new(),
        }
    }

    /// Añade o sustituye un parámetro `key=value`. Si la clave ya existe, el valor se sobrescribe.
    ///
    /// El valor se codifica según RFC 3986, los caracteres no reservados (alfanuméricos ASCII, `-`,
    /// `_`, `.`, `~`) quedan intactos, el resto se sustituye por su secuencia `%XX`. La clave se
    /// inserta tal cual, sin codificar.
    ///
    /// Un `value` vacío no se distingue de [`with_flag()`](Self::with_flag): ambos se renderizan
    /// como `?key`, sin `=`.
    #[builder_fn]
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query
            .insert(key.into(), Self::encode_query_value(&value.into()));
        self
    }

    /// Añade o sustituye un *flag* sin valor, por ejemplo `?debug`.
    #[builder_fn]
    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.query.insert(flag.into(), String::new());
        self
    }

    /// Devuelve el *path* inicial tal y como se pasó a [`RoutePath::new`], sin parámetros.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Indica si el *path* **parece** una URL externa por su prefijo (ver
    /// [`util::url_looks_external()`](crate::util::url_looks_external)).
    pub fn is_external(&self) -> bool {
        crate::util::url_looks_external(&self.path)
    }

    /// Indica si la ruta no tiene *path* ni parámetros, es decir, si su representación textual
    /// sería una cadena vacía.
    pub fn is_empty(&self) -> bool {
        self.path.is_empty() && self.query.is_empty()
    }

    // **< RoutePath HELPERS >**********************************************************************

    // Codifica un valor para su uso seguro como parte de una *query string* según RFC 3986: los
    // caracteres no reservados quedan intactos y el resto se codifica como `%XX`.
    fn encode_query_value(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        for byte in value.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char);
                }
                _ => write!(out, "%{byte:02X}").unwrap(),
            }
        }
        out
    }
}

impl fmt::Display for RoutePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.path)?;
        if !self.query.is_empty() {
            f.write_str("?")?;
            for (i, (key, value)) in self.query.iter().enumerate() {
                if i > 0 {
                    f.write_str("&")?;
                }
                f.write_str(key)?;
                if !value.is_empty() {
                    f.write_str("=")?;
                    f.write_str(value)?;
                }
            }
        }
        Ok(())
    }
}

// Cualquier `&str`, sea cual sea su vida, se acepta copiándolo a un `String` propio: así, por
// ejemplo, una función hipotética que devuelva un `&str` con una vida atada a una petición, no
// `'static` (del estilo `fn resolve_target<'a>(next: &'a str, fallback: &'a str) -> &'a str`),
// sigue pudiendo construir un `RoutePath` sin que quien llama tenga que convertir el valor a mano.
impl From<&str> for RoutePath {
    fn from(path: &str) -> Self {
        RoutePath::new(path.to_owned())
    }
}

impl From<String> for RoutePath {
    fn from(path: String) -> Self {
        RoutePath::new(path)
    }
}
