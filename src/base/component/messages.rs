use crate::prelude::*;

/// Componente que muestra los **mensajes de usuario** acumulados en el contexto.
///
/// No recibe ningún contenido de quien lo construye. Lo obtiene directamente del [`Context`] (ver
/// [`push_message()`](Contextual::push_message)) en el momento de renderizarse, y retira los
/// mensajes que muestra para que no se repitan. Si no hay ninguno acumulado, no renderiza nada.
///
/// [`Region`] añade uno al inicio de la región [`CoreRegions::Content`], así que no hace falta
/// incluirlo para ver los mensajes, tampoco los que llegan de la página anterior con
/// [`ResponseExt::with_message()`]. Ese componente está aplazado: se renderiza cuando ya se ha
/// renderizado el resto de la página, así que recoge también los mensajes que añadan los demás
/// componentes, aunque se muestra en su sitio.
///
/// Sólo hay que añadirlo a mano para mostrar los mensajes en un lugar concreto (por ejemplo, junto
/// a un formulario) o en un fragmento HTML que se renderiza sin página. Por defecto no se aplaza;
/// se renderiza en su turno, como cualquier otro componente, y muestra los mensajes acumulados
/// hasta ese momento, que ya no aparecerán al inicio de `Content`. También se puede aplazar con
/// [`with_deferred()`](Self::with_deferred) (ver en [`CoreRegions::Content`] cómo se reparten
/// entonces los mensajes).
///
/// [`Region`]: crate::base::component::layout::Region
///
/// # Clases CSS
///
/// Cada nivel de severidad renderiza siempre las mismas clases fijas: `.message.message-info`,
/// `.message.message-warning` o `.message.message-error`. Es el tema activo quien decide el aspecto
/// visual de cada nivel, proporcionando las reglas CSS para esas clases.
///
/// # Ejemplos
///
/// Los mensajes se muestran al inicio de `Content` sin añadir el componente:
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// async fn homepage(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     let mut page = Page::new(request);
///     page.push_message(MessageLevel::Info, Lc::n("Settings saved."));
///     page.with_child(Html::with(|_| html! { p { "..." } }))
///         .render()
///         .await
/// }
/// ```
///
/// Para mostrarlos junto antes de un formulario:
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// async fn contact(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     Page::new(request)
///         .with_child(Messages::new())
///         .with_child(
///             Form::new()
///                 .with_action("/contact")
///                 .with_child(Button::submit(Lc::n("Send"))),
///         )
///         .render()
///         .await
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Messages {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    #[getters(skip)]
    deferred: bool,
}

#[async_trait]
impl Component for Messages {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn is_deferred(&self) -> bool {
        self.deferred
    }

    fn setup(&mut self, _cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes("messages"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let messages = cx.take_messages();
        if messages.is_empty() {
            return Ok(html! {});
        }
        Ok(html! {
            div (self.props().unpack(cx)) role="alert" {
                @for message in &messages {
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
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece si el componente se aplaza (ver [`Component::is_deferred()`]).
    ///
    /// Dentro de una página, un componente aplazado se renderiza una vez renderizado el resto del
    /// `<body>`, aunque se muestre en su sitio, y así recoge también los mensajes que añadan los
    /// componentes que se renderizan después. Es lo que hace el que encabeza la región
    /// [`CoreRegions::Content`]. Por defecto no se aplaza, y muestra sólo los mensajes acumulados
    /// hasta el momento de renderizarse.
    pub fn with_deferred(mut self, deferred: bool) -> Self {
        self.deferred = deferred;
        self
    }
}
