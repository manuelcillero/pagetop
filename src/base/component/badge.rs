use crate::prelude::*;

/// Componente para mostrar una **etiqueta corta informativa** (*badge*).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let badge = Badge::new().with_label(L10n::n("Admin"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Badge {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la etiqueta del badge.
    label: L10n,
}

#[async_trait]
impl Component for Badge {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes("badge"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            span (self.props()) {
                (self.label().using(cx))
            }
        })
    }
}

impl Badge {
    // **< Badge BUILDER >**************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece la etiqueta del badge.
    #[builder_fn]
    pub fn with_label(mut self, label: L10n) -> Self {
        self.label = label;
        self
    }
}
