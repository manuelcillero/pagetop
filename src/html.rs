//! HTML en código.

mod maud;
pub use maud::{display, html, html_private, Escaper, Markup, PreEscaped, Render, DOCTYPE};

mod assets;
pub use assets::favicon::Favicon;
pub use assets::javascript::JavaScript;
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub(crate) use assets::Assets;

mod context;
pub use context::{Context, ErrorParam};

use crate::AutoDefault;

/// Prepara contenido HTML para su conversión a [`Markup`].
///
/// Este tipo encapsula distintos orígenes de contenido HTML (texto plano, HTML escapado o marcado
/// ya procesado) para renderizar de forma homogénea en plantillas sin interferir con el uso
/// estándar de [`Markup`].
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// let fragment = PrepareMarkup::Text(String::from("Hola <b>mundo</b>"));
/// assert_eq!(fragment.render().into_string(), "Hola &lt;b&gt;mundo&lt;/b&gt;");
///
/// let raw_html = PrepareMarkup::Escaped(String::from("<b>negrita</b>"));
/// assert_eq!(raw_html.render().into_string(), "<b>negrita</b>");
///
/// let prepared = PrepareMarkup::With(html! {
///     h2 { "Título de ejemplo" }
///     p { "Este es un párrafo con contenido dinámico." }
/// });
/// assert_eq!(
///     prepared.render().into_string(),
///     "<h2>Título de ejemplo</h2><p>Este es un párrafo con contenido dinámico.</p>"
/// );
/// ```
#[derive(AutoDefault)]
pub enum PrepareMarkup {
    /// No se genera contenido HTML (devuelve `html! {}`).
    #[default]
    None,
    /// Texto estático que se escapará automáticamente para no ser interpretado como HTML.
    Text(String),
    /// Contenido sin escapado adicional, útil para HTML generado externamente.
    Escaped(String),
    /// Fragmento HTML ya preparado como [`Markup`], listo para insertarse directamente.
    With(Markup),
}

impl PrepareMarkup {
    /// Devuelve `true` si el contenido está vacío y no generará HTML al renderizar.
    pub fn is_empty(&self) -> bool {
        match self {
            PrepareMarkup::None => true,
            PrepareMarkup::Text(text) => text.is_empty(),
            PrepareMarkup::Escaped(string) => string.is_empty(),
            PrepareMarkup::With(markup) => markup.is_empty(),
        }
    }
}

impl Render for PrepareMarkup {
    /// Integra el renderizado fácilmente en la macro [`html!`].
    fn render(&self) -> Markup {
        match self {
            PrepareMarkup::None => html! {},
            PrepareMarkup::Text(text) => html! { (text) },
            PrepareMarkup::Escaped(string) => html! { (PreEscaped(string)) },
            PrepareMarkup::With(markup) => html! { (markup) },
        }
    }
}
