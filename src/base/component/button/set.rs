use crate::prelude::*;

/// Componente para mostrar un **conjunto de botones**.
///
/// Envuelve los botones en un contenedor que cada tema estiliza para separarlos visualmente. Sólo
/// admite componentes [`Button`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let actions = button::ButtonSet::new()
///     .with_button(Button::submit(Lc::n("Save")))
///     .with_button(Button::plain(Lc::n("Cancel")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct ButtonSet {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve los botones del conjunto.
    buttons: Children,
}

#[async_trait]
impl Component for ButtonSet {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes("button-set"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let buttons = self.buttons().render(cx).await;
        if buttons.is_empty() {
            return Ok(html! {});
        }
        Ok(html! {
            div (self.props()) { (buttons) }
        })
    }
}

impl ButtonSet {
    // **< ButtonSet BUILDER >*************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Añade un botón al conjunto, o modifica su lista de botones con una operación [`TypedOp`].
    #[builder_fn]
    pub fn with_button(mut self, op: impl Into<TypedOp<Button>>) -> Self {
        self.buttons.alter_child(op.into());
        self
    }
}
