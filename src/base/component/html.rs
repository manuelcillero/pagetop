use crate::prelude::*;

/// Componente básico para renderizar directamente código HTML.
#[derive(AutoDefault)]
pub struct Html(Markup);

impl ComponentTrait for Html {
    fn new() -> Self {
        Html::default()
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! { (self.0) })
    }
}

impl Html {
    /// Crear una instancia con el código HTML del argumento.
    pub fn with(html: Markup) -> Self {
        Html(html)
    }

    /// Modifica el código HTML de la instancia con el nuevo código del argumento.
    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.0 = html;
        self
    }
}
