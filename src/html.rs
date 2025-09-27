//! HTML en código.

mod maud;
pub use maud::{display, html, html_private, Escaper, Markup, PreEscaped, DOCTYPE};

// HTML DOCUMENT ASSETS ****************************************************************************

mod assets;
pub use assets::favicon::Favicon;
pub use assets::javascript::JavaScript;
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::{Asset, Assets};

// HTML DOCUMENT CONTEXT ***************************************************************************

mod context;
pub use context::{AssetsOp, Context, Contextual, ErrorParam};

// HTML ATTRIBUTES *********************************************************************************

mod attr_id;
pub use attr_id::AttrId;
/// **Obsoleto desde la versión 0.4.0**: usar [`AttrId`] en su lugar.
#[deprecated(since = "0.4.0", note = "Use `AttrId` instead")]
pub type OptionId = AttrId;

mod attr_name;
pub use attr_name::AttrName;
/// **Obsoleto desde la versión 0.4.0**: usar [`AttrName`] en su lugar.
#[deprecated(since = "0.4.0", note = "Use `AttrName` instead")]
pub type OptionName = AttrName;

mod attr_value;
pub use attr_value::AttrValue;
/// **Obsoleto desde la versión 0.4.0**: usar [`AttrValue`] en su lugar.
#[deprecated(since = "0.4.0", note = "Use `AttrValue` instead")]
pub type OptionString = AttrValue;

mod attr_l10n;
pub use attr_l10n::AttrL10n;
/// **Obsoleto desde la versión 0.4.0**: usar [`AttrL10n`] en su lugar.
#[deprecated(since = "0.4.0", note = "Use `AttrL10n` instead")]
pub type OptionTranslated = AttrL10n;

mod attr_classes;
pub use attr_classes::{AttrClasses, ClassesOp};
/// **Obsoleto desde la versión 0.4.0**: usar [`AttrClasses`] en su lugar.
#[deprecated(since = "0.4.0", note = "Use `AttrClasses` instead")]
pub type OptionClasses = AttrClasses;

use crate::{core, AutoDefault};

/// **Obsoleto desde la versión 0.4.0**: usar [`Typed`](crate::core::component::Typed) en su lugar.
#[deprecated(
    since = "0.4.0",
    note = "Use `pagetop::core::component::Typed` instead"
)]
#[allow(type_alias_bounds)]
pub type OptionComponent<C: core::component::Component> = core::component::Typed<C>;

/// Prepara contenido HTML para su conversión a [`Markup`].
///
/// Este tipo encapsula distintos orígenes de contenido HTML (texto plano, HTML sin escapar o
/// fragmentos ya procesados) para renderizarlos de forma homogénea en plantillas, sin interferir
/// con el uso estándar de [`Markup`].
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Texto normal, se escapa automáticamente para evitar inyección de HTML.
/// let fragment = PrepareMarkup::Escaped("Hola <b>mundo</b>".to_string());
/// assert_eq!(fragment.render().into_string(), "Hola &lt;b&gt;mundo&lt;/b&gt;");
///
/// // HTML literal, se inserta directamente, sin escapado adicional.
/// let raw_html = PrepareMarkup::Raw("<b>negrita</b>".to_string());
/// assert_eq!(raw_html.render().into_string(), "<b>negrita</b>");
///
/// // Fragmento ya preparado con la macro `html!`.
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

    /// Integra el renderizado fácilmente en la macro [`html!`].
    pub fn render(&self) -> Markup {
        match self {
            PrepareMarkup::None => html! {},
            PrepareMarkup::Escaped(text) => html! { (text) },
            PrepareMarkup::Raw(string) => html! { (PreEscaped(string)) },
            PrepareMarkup::With(markup) => html! { (markup) },
        }
    }
}
