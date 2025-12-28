//! HTML en código.

use crate::AutoDefault;

mod maud;
pub use maud::{display, html, html_private, Escaper, Markup, PreEscaped, DOCTYPE};

mod route;
pub use route::RoutePath;

// **< HTML DOCUMENT ASSETS >***********************************************************************

mod assets;
pub use assets::favicon::Favicon;
pub use assets::javascript::JavaScript;
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::{Asset, Assets};

mod logo;
pub use logo::PageTopSvg;

// **< HTML ATTRIBUTES >****************************************************************************

mod attr;
pub use attr::{Attr, AttrId, AttrName, AttrValue};

mod classes;
pub use classes::{Classes, ClassesOp};

mod unit;
pub use unit::UnitValue;

// **< HTML PrepareMarkup >*************************************************************************

/// Prepara contenido HTML para su conversión a [`Markup`].
///
/// Este tipo encapsula distintos orígenes de contenido HTML (texto plano, HTML sin escapar o
/// fragmentos ya procesados) para renderizarlos de forma homogénea en plantillas, sin interferir
/// con el uso estándar de [`Markup`].
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Texto normal, se escapa automáticamente para evitar inyección de HTML.
/// let fragment = PrepareMarkup::Escaped("Hola <b>mundo</b>".to_string());
/// assert_eq!(fragment.into_string(), "Hola &lt;b&gt;mundo&lt;/b&gt;");
///
/// // HTML literal, se inserta directamente, sin escapado adicional.
/// let raw_html = PrepareMarkup::Raw("<b>negrita</b>".to_string());
/// assert_eq!(raw_html.into_string(), "<b>negrita</b>");
///
/// // Fragmento ya preparado con la macro `html!`.
/// let prepared = PrepareMarkup::With(html! {
///     h2 { "Título de ejemplo" }
///     p { "Este es un párrafo con contenido dinámico." }
/// });
/// assert_eq!(
///     prepared.into_string(),
///     "<h2>Título de ejemplo</h2><p>Este es un párrafo con contenido dinámico.</p>"
/// );
/// ```
#[derive(AutoDefault, Clone)]
pub enum PrepareMarkup {
    /// No se genera contenido HTML (equivale a `html! {}`).
    #[default]
    None,
    /// Texto plano que se **escapará automáticamente** para que no sea interpretado como HTML.
    ///
    /// Úsalo con textos que provengan de usuarios u otras fuentes externas para garantizar la
    /// seguridad contra inyección de código.
    Escaped(String),
    /// HTML literal que se inserta **sin escapado adicional**.
    ///
    /// Úsalo únicamente para contenido generado de forma confiable o controlada, ya que cualquier
    /// etiqueta o script incluido será renderizado directamente en el documento.
    Raw(String),
    /// Fragmento HTML ya preparado como [`Markup`], listo para insertarse directamente.
    ///
    /// Normalmente proviene de expresiones `html! { ... }`.
    With(Markup),
}

impl PrepareMarkup {
    /// Devuelve `true` si el contenido está vacío y no generará HTML al renderizar.
    pub fn is_empty(&self) -> bool {
        match self {
            PrepareMarkup::None => true,
            PrepareMarkup::Escaped(text) => text.is_empty(),
            PrepareMarkup::Raw(string) => string.is_empty(),
            PrepareMarkup::With(markup) => markup.is_empty(),
        }
    }

    /// Convierte el contenido en una cadena HTML renderizada. Usar sólo para pruebas o depuración.
    pub fn into_string(&self) -> String {
        self.render().into_string()
    }

    /// Integra el renderizado fácilmente en la macro [`html!`].
    pub(crate) fn render(&self) -> Markup {
        match self {
            PrepareMarkup::None => html! {},
            PrepareMarkup::Escaped(text) => html! { (text) },
            PrepareMarkup::Raw(string) => html! { (PreEscaped(string)) },
            PrepareMarkup::With(markup) => html! { (markup) },
        }
    }
}
