use crate::html::assets::Asset;
use crate::html::{html, Context, Markup, PreEscaped};
use crate::{join_pair, AutoDefault, Weight};

// Define el origen del recurso CSS y cómo se incluye en el documento.
//
// Los estilos pueden cargarse desde un archivo externo o estar embebidos directamente en una
// etiqueta `<style>`.
//
// - [`From`]   – Carga la hoja de estilos desde un archivo externo, insertándola mediante una
//                etiqueta `<link>` con `rel="stylesheet"`.
// - [`Inline`] – Inserta directamente el contenido CSS dentro de una etiqueta `<style>`.
#[derive(AutoDefault)]
enum Source {
    #[default]
    From(String),
    // `name`, `closure(Context) -> String`.
    Inline(String, Box<dyn Fn(&mut Context) -> String + Send + Sync>),
}

/// Define el medio objetivo para la hoja de estilos.
///
/// Permite especificar en qué contexto se aplica el CSS, adaptándose a diferentes dispositivos o
/// situaciones de impresión.
#[derive(AutoDefault)]
pub enum TargetMedia {
    /// Se aplica en todos los casos (el atributo `media` se omite).
    #[default]
    Default,
    /// Se aplica cuando el documento se imprime.
    Print,
    /// Se aplica en pantallas.
    Screen,
    /// Se aplica en dispositivos que convierten el texto a voz.
    Speech,
}

/// Devuelve el valor para el atributo `media` (`Some(...)`) o `None` para `Default`.
#[rustfmt::skip]
impl TargetMedia {
    fn as_str_opt(&self) -> Option<&str> {
        match self {
            TargetMedia::Default => None,
            TargetMedia::Print   => Some("print"),
            TargetMedia::Screen  => Some("screen"),
            TargetMedia::Speech  => Some("speech"),
        }
    }
}

/// Define un recurso **StyleSheet** para incluir en un documento HTML.
///
/// Este tipo permite incluir hojas de estilo CSS externas o embebidas, con soporte para medios
/// específicos (`screen`, `print`, etc.) y [pesos](crate::Weight) que determinan el orden de
/// inserción en el documento.
///
/// > **Nota**
/// > Las hojas de estilo CSS deben estar disponibles en el servidor web de la aplicación. Pueden
/// > servirse usando [`static_files_service!`](crate::static_files_service).
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Crea una hoja de estilos externa con control de versión y medio específico (`screen`).
/// let stylesheet = StyleSheet::from("/assets/css/main.css")
///     .with_version("2.0.1")
///     .for_media(TargetMedia::Screen)
///     .with_weight(-10);
///
/// // Crea una hoja de estilos embebida en el documento HTML.
/// let embedded = StyleSheet::inline("custom_theme", |_| r#"
///     body {
///         background-color: #f5f5f5;
///         font-family: 'Segoe UI', sans-serif;
///     }
/// "#.to_string());
/// ```
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct StyleSheet {
    source : Source,      // Fuente y modo de inclusión del CSS.
    version: String,      // Versión del recurso para la caché del navegador.
    media  : TargetMedia, // Medio objetivo para los estilos (`print`, `screen`, ...).
    weight : Weight,      // Peso que determina el orden.
}

impl StyleSheet {
    /// Crea una hoja de estilos externa.
    ///
    /// Equivale a `<link rel="stylesheet" href="...">`.
    pub fn from(path: impl Into<String>) -> Self {
        StyleSheet {
            source: Source::From(path.into()),
            ..Default::default()
        }
    }

    /// Crea una hoja de estilos embebida directamente en el documento HTML.
    ///
    /// Equivale a `<style>...</style>`. El parámetro `name` se usa como identificador interno del
    /// recurso.
    ///
    /// La función *closure* recibirá el [`Context`] por si se necesita durante el renderizado.
    pub fn inline<F>(name: impl Into<String>, f: F) -> Self
    where
        F: Fn(&mut Context) -> String + Send + Sync + 'static,
    {
        StyleSheet {
            source: Source::Inline(name.into(), Box::new(f)),
            ..Default::default()
        }
    }

    // StyleSheet BUILDER **************************************************************************

    /// Asocia una versión al recurso (usada para control de la caché del navegador).
    ///
    /// Si `version` está vacío, no se añade ningún parámetro a la URL.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Modifica el peso del recurso.
    ///
    /// Los recursos se renderizan de menor a mayor peso. Por defecto es `0`, que respeta el orden
    /// de creación.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    // StyleSheet EXTRAS ***************************************************************************

    /// Especifica el medio donde se aplican los estilos.
    ///
    /// Según el argumento `media`:
    ///
    /// - `TargetMedia::Default` - Se aplica en todos los casos (medio por defecto).
    /// - `TargetMedia::Print`   - Se aplica cuando el documento se imprime.
    /// - `TargetMedia::Screen`  - Se aplica en pantallas.
    /// - `TargetMedia::Speech`  - Se aplica en dispositivos que convierten el texto a voz.
    pub fn for_media(mut self, media: TargetMedia) -> Self {
        self.media = media;
        self
    }
}

impl Asset for StyleSheet {
    /// Devuelve el nombre del recurso, utilizado como clave única.
    ///
    /// Para hojas de estilos externas es la ruta del recurso; para las embebidas, un identificador.
    fn name(&self) -> &str {
        match &self.source {
            Source::From(path) => path,
            Source::Inline(name, _) => name,
        }
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn render(&self, cx: &mut Context) -> Markup {
        match &self.source {
            Source::From(path) => html! {
                link
                    rel="stylesheet"
                    href=(join_pair!(path, "?v=", self.version.as_str()))
                    media=[self.media.as_str_opt()];
            },
            Source::Inline(_, f) => html! {
                style { (PreEscaped((f)(cx))) };
            },
        }
    }
}
