use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup, PreEscaped, Render};
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
    Inline(String, String),
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

/// Devuelve el texto asociado al punto de interrupción usado por Bootstrap.
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
/// > incluirse en el proyecto utilizando [`include_files!`](crate::include_files) y servirse con
/// > [`include_files_service!`](crate::include_files_service).
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
/// let embedded = StyleSheet::inline("custom_theme", r#"
///     body {
///         background-color: #f5f5f5;
///         font-family: 'Segoe UI', sans-serif;
///     }
/// "#);
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
    pub fn inline(name: impl Into<String>, styles: impl Into<String>) -> Self {
        StyleSheet {
            source: Source::Inline(name.into(), styles.into()),
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
    /// - `TargetMedia::Print`   - Se aplican cuando el documento se imprime.
    /// - `TargetMedia::Screen`  - Se aplican en pantallas.
    /// - `TargetMedia::Speech`  - Se aplican en dispositivos que convierten el texto a voz.
    pub fn for_media(mut self, media: TargetMedia) -> Self {
        self.media = media;
        self
    }
}

impl AssetsTrait for StyleSheet {
    // Para hojas de estilos externas es la ruta; para las embebidas, un identificador.
    fn name(&self) -> &str {
        match &self.source {
            Source::From(path) => path,
            Source::Inline(name, _) => name,
        }
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl Render for StyleSheet {
    fn render(&self) -> Markup {
        match &self.source {
            Source::From(path) => html! {
                link
                    rel="stylesheet"
                    href=(join_pair!(path, "?v=", self.version.as_str()))
                    media=[self.media.as_str_opt()];
            },
            Source::Inline(_, code) => html! {
                style { (PreEscaped(code)) };
            },
        }
    }
}
