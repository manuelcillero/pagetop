use crate::prelude::*;

/// Texto de ayuda de un campo de formulario, ya traducido, con el identificador que lo enlaza a su
/// control.
///
/// Lo usan los campos de formulario (y los temas que pintan su propio marcado) para mostrar el
/// texto de ayuda bajo el control y enlazarlo con `aria-describedby`, de modo que los lectores de
/// pantalla lo anuncien al entrar en el campo. Se inserta en [`html!`] como cualquier otro valor:
/// genera `<div id="..." class="form-text">...</div>`, o nada si no hay texto.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn render(help_text: &Lc, input_id: Option<&str>, cx: &Context) -> Markup {
/// let help = form::FieldHelp::new(help_text, input_id, cx);
/// html! {
///     input type="text" id=[input_id] aria-describedby=[help.id()];
///     (help)
/// }
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct FieldHelp {
    text: Option<String>,
    id: Option<String>,
}

impl FieldHelp {
    /// Traduce `help_text` y, si hay texto, le asigna el identificador `{anchor_id}-help`, donde
    /// `anchor_id` es el identificador del control (o del contenedor, si el texto describe a un
    /// grupo de controles). Sin `anchor_id` el texto se muestra igual, pero sin identificador.
    pub fn new(help_text: &Lc, anchor_id: Option<&str>, language: &impl LangId) -> Self {
        Self::suffixed(help_text, anchor_id, "-help", language)
    }

    /// Igual que [`new()`](Self::new), pero con otro sufijo para el identificador; útil cuando un
    /// mismo control tiene más de un texto que lo describe.
    pub fn suffixed(
        help_text: &Lc,
        anchor_id: Option<&str>,
        suffix: &str,
        language: &impl LangId,
    ) -> Self {
        let text = help_text.lookup(language);
        // Sin texto no hay nada que enlazar, y sin identificador del control no hay de qué derivar
        // uno: en ambos casos se omite el identificador para no dejar una referencia rota.
        let id = text
            .as_ref()
            .and(anchor_id)
            .map(|anchor_id| util::join!(anchor_id, suffix));
        Self { text, id }
    }

    /// Devuelve el identificador para `aria-describedby`, o `None` si no hay texto o si el control
    /// no tiene identificador.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

impl Render for FieldHelp {
    fn render(&self) -> Markup {
        html! {
            @if let Some(text) = &self.text {
                div id=[self.id()] class="form-text" { (text) }
            }
        }
    }
}
