use crate::prelude::*;

/// Componente que muestra los **mensajes de usuario** acumulados en el contexto.
///
/// No recibe ningún contenido de quien lo construye. Lo obtiene directamente del [`Context`] (ver
/// [`Context::push_message()`](Context::push_message)) en el momento de renderizarse (ver
/// [`Context::messages()`]). Si no hay ninguno acumulado, no se renderiza nada.
///
/// # Clases CSS
///
/// Cada nivel de severidad renderiza siempre las mismas clases fijas: `.message.message-info`,
/// `.message.message-warning` o `.message.message-error`. Será el tema activo quien decide el
/// aspecto visual de cada nivel, proporcionando las reglas CSS para esas clases.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// async fn save_settings(cx: &mut Context) {
///     cx.push_message(MessageLevel::Info, Lc::n("Settings saved."));
/// }
///
/// async fn homepage(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     Page::new(request)
///         .with_child(Messages::new())
///         // ... resto del contenido de la página ...
///         .render()
///         .await
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Messages {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
}

#[async_trait]
impl Component for Messages {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes("messages"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        if !cx.has_messages() {
            return Ok(html! {});
        }
        Ok(html! {
            div (self.props().unpack(cx)) role="alert" {
                @for message in cx.messages() {
                    div class=(match message.level() {
                        MessageLevel::Info => "message message-info",
                        MessageLevel::Warning => "message message-warning",
                        MessageLevel::Error => "message message-error",
                    }) {
                        (message.text().using(cx))
                    }
                }
            }
        })
    }
}

#[builder_impl]
impl Messages {
    // **< Messages BUILDER >***********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }
}
