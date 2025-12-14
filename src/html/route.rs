use crate::{builder_fn, AutoDefault};

use std::borrow::Cow;
use std::fmt;

/// Representa una ruta como un *path* inicial más una lista opcional de parámetros.
///
/// Modela rutas del estilo `/path/to/resource?foo=bar&debug` o `https://example.com/path?foo=bar`,
/// pensadas para usarse en atributos HTML como `href`, `action` o `src`.
///
/// `RoutePath` no valida ni interpreta la estructura del *path*; simplemente concatena los
/// parámetros de consulta sobre el valor proporcionado.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Ruta relativa con parámetros y una *flag* sin valor.
/// let route = RoutePath::new("/search")
///     .with_param("q", "rust")
///     .with_param("page", "2")
///     .with_flag("debug");
/// assert_eq!(route.to_string(), "/search?q=rust&page=2&debug");
///
/// // Ruta absoluta a un recurso externo.
/// let external = RoutePath::new("https://example.com/export").with_param("format", "csv");
/// assert_eq!(external.to_string(), "https://example.com/export?format=csv");
/// ```
#[derive(AutoDefault)]
pub struct RoutePath {
    // *Path* inicial sobre el que se añadirán los parámetros.
    //
    // Puede ser relativo (p. ej. `/about`) o una ruta completa (`https://example.com/about`).
    // `RoutePath` no realiza ninguna validación ni normalización.
    //
    // Se almacena como `Cow<'static, str>` para reutilizar literales estáticos sin asignación
    // adicional y, al mismo tiempo, aceptar rutas dinámicas representadas como `String`.
    path: Cow<'static, str>,

    // Conjunto de parámetros asociados a la ruta.
    //
    // Cada clave es única y se mantiene el orden de inserción. El valor vacío se utiliza para
    // representar *flags* sin valor explícito (por ejemplo `?debug`).
    query: indexmap::IndexMap<String, String>,
}

impl RoutePath {
    /// Crea un `RoutePath` a partir de un *path* inicial.
    ///
    /// Por ejemplo: `RoutePath::new("/about")`.
    pub fn new(path: impl Into<Cow<'static, str>>) -> Self {
        Self {
            path: path.into(),
            query: indexmap::IndexMap::new(),
        }
    }

    /// Añade o sustituye un parámetro `key=value`. Si la clave ya existe, el valor se sobrescribe.
    #[builder_fn]
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
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

impl From<&'static str> for RoutePath {
    fn from(path: &'static str) -> Self {
        RoutePath::new(path)
    }
}

impl From<String> for RoutePath {
    fn from(path: String) -> Self {
        RoutePath::new(path)
    }
}
