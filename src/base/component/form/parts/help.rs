use crate::prelude::*;

/// Texto de ayuda de un campo de formulario.
///
/// Muestra el texto de ayuda de un control (o de un grupo de controles), con un identificador para
/// que el control (o el contenedor del grupo) lo enlace con `aria-describedby`, de modo que las
/// tecnologías de apoyo suelen anunciarlo al entrar en el campo. Se renderiza con
/// [`render()`](Self::render), que genera un `<div>`, o nada si el texto es [`Lc::none()`]. La
/// clase predeterminada (`form-text`) puede cambiarse con [`with_class()`](Self::with_class).
///
/// Se puede crear con:
///
/// - [`new()`](Self::new): para el texto de ayuda de un control, con el identificador
///   `{anchor_id}-help`.
/// - [`suffixed()`](Self::suffixed): para otro texto que describa al mismo control, con otro sufijo
///   en el identificador.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn render(help_text: &Lc, input_id: Option<&str>, cx: &Context) -> Markup {
/// let help = form::parts::Help::new(help_text, input_id);
/// html! {
///     input type="text" id=[input_id] aria-describedby=[help.id()];
///     (help.render(cx))
/// }
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Help<'a> {
    text: &'a Lc,
    id: Option<String>,
    class: CowStr,
}

impl<'a> Help<'a> {
    /// Texto de ayuda `help_text` con el identificador `{anchor_id}-help`, donde `anchor_id` es el
    /// identificador del control (o del contenedor, si el texto describe a un grupo de controles).
    /// Sin `anchor_id` el texto se muestra igual, pero sin identificador.
    pub fn new(help_text: &'a Lc, anchor_id: Option<&str>) -> Self {
        Self::suffixed(help_text, anchor_id, "-help")
    }

    /// Igual que [`new()`](Self::new), pero con otro sufijo para el identificador, que se añade tal
    /// cual (p. ej. `-format` da `{anchor_id}-format`). Es útil cuando un mismo control tiene más
    /// de un texto que lo describe: cada uno necesita un sufijo distinto, y el control los enlaza
    /// todos en `aria-describedby`, separados por espacios.
    pub fn suffixed(help_text: &'a Lc, anchor_id: Option<&str>, suffix: &str) -> Self {
        // Sin texto no hay nada que enlazar, y sin identificador del control no hay de qué derivar
        // uno: en ambos casos se omite el identificador para no dejar una referencia rota.
        let id = anchor_id
            .filter(|_| !help_text.is_none())
            .map(|anchor_id| util::join!(anchor_id, suffix));
        Self {
            text: help_text,
            id,
            class: CowStr::Borrowed("form-text"),
        }
    }

    /// Devuelve el identificador para `aria-describedby`, o `None` si el texto es [`Lc::none()`] o
    /// si no se indicó `anchor_id`.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Renderiza el texto de ayuda traducido al idioma del contexto.
    pub fn render(&self, cx: &Context) -> Markup {
        // Se genera el elemento aunque falte la traducción, para que `id()` no quede sin destino.
        html! {
            @if !self.text.is_none() {
                div id=[self.id()] class=(self.class) { (self.text.lookup(cx).unwrap_or_default()) }
            }
        }
    }
}

#[builder_impl]
impl<'a> Help<'a> {
    /// Sustituye la clase del texto de ayuda (por defecto, `form-text`) por otra u otras, separadas
    /// por espacios; útil para un tema cuyo marcado use otras clases.
    pub fn with_class(mut self, class: impl Into<CowStr>) -> Self {
        self.class = class.into();
        self
    }
}
