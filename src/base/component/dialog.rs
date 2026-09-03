use crate::prelude::*;

/// Componente para crear un **diálogo modal**.
///
/// Renderiza un cuadro de diálogo con cabecera (título opcional más el botón de cierre), cuerpo y
/// un pie opcional para botones de acción (`Ok`, `Cancelar`, u otros botones personalizados).
///
/// Sin cuerpo ni pie, el componente **no se renderiza**.
///
/// Cualquier [`Button`] que deba abrir o cerrar el diálogo debe usar el siguiente vocabulario de
/// atributos `data-dialog-*`:
///
/// - `data-dialog-toggle="modal"` más `data-dialog-target="#id"` en el `Button` que abre el
///   diálogo.
/// - `data-dialog-dismiss="modal"` en cualquier `Button` que deba cerrarlo (el botón de cierre de
///   la cabecera ya lo lleva).
///
/// Sólo en los componentes `Button` **podrán renombrarse automáticamente estos atributos al
/// vocabulario propio de cada tema**; cualquier otro elemento deberá usar directamente el nombre
/// final que el tema en cuestión espera, perdiendo así la portabilidad entre temas que sí tiene un
/// `Button`.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let dialog = Dialog::new()
///     .with_id("confirm-delete")
///     .with_title(Lc::n("Delete record"))
///     .with_child(
///         Html::with(|_| html! { p { "Are you sure you want to delete this record?" } })
///     )
///     .with_footer(
///         Button::plain(Lc::n("Cancel")).with_prop(PropsOp::set("data-dialog-dismiss", "modal")),
///     )
///     .with_footer(Button::submit(Lc::n("Delete")));
///
/// let opener = Button::plain(Lc::n("Delete"))
///     .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
///     .with_prop(PropsOp::set("data-dialog-target", "#confirm-delete"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Dialog {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el título del diálogo.
    title: Lc,
    /// Devuelve la lista de componentes (`children`) del cuerpo del diálogo.
    body: Children,
    /// Devuelve la lista de componentes (`children`) del pie del diálogo.
    footer: Children,
}

#[async_trait]
impl Component for Dialog {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &Context) {
        // Asegura que el diálogo tiene un identificador único con el que abrirlo.
        self.alter_prop(PropsOp::ensure_id(cx.build_id::<Self>(1)));
        self.alter_prop(PropsOp::prepend_classes("dialog"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let body = self.body().render(cx).await;
        let footer = self.footer().render(cx).await;
        if body.is_empty() && footer.is_empty() {
            return Ok(html! {});
        }

        let title = self.title().using(cx);
        // `setup()` garantiza que habrá un `id` antes de renderizar. Sin título no hay elemento
        // que etiquete el diálogo, así que `aria-labelledby` se omite en vez de apuntar a un `id`
        // inexistente.
        let id_label = (!title.is_empty()).then(|| util::join!(self.id().unwrap(), "-label"));

        Ok(html! {
            dialog (self.props()) aria-labelledby=[id_label.as_deref()] {
                div class="dialog-header" {
                    @if let Some(id_label) = &id_label {
                        h2 id=(id_label) class="dialog-title" { (title) }
                    }
                    button
                        type="button"
                        class="dialog-close"
                        data-dialog-dismiss="modal"
                        aria-label=[Lc::l("dialog_close").lookup(cx)]
                    {}
                }
                div class="dialog-body" { (body) }
                @if !footer.is_empty() {
                    div class="dialog-footer" { (footer) }
                }
            }
        })
    }
}

#[builder_impl]
impl Dialog {
    // **< Dialog BUILDER >*************************************************************************

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

    /// Establece el título del diálogo.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Añade un nuevo componente al cuerpo del diálogo o modifica la lista de componentes
    /// (`children`) del cuerpo con una operación [`ChildOp`].
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.body.alter_child(op.into());
        self
    }

    /// Añade un nuevo componente al pie del diálogo (normalmente un [`Button`]) o modifica la
    /// lista de componentes (`children`) del pie con una operación [`ChildOp`].
    ///
    /// El pie ya se maqueta en fila y alineado a la derecha por su propia clase CSS
    /// (`dialog-footer`); no requiere ninguna configuración adicional para alinear los botones.
    pub fn with_footer(mut self, op: impl Into<ChildOp>) -> Self {
        self.footer.alter_child(op.into());
        self
    }
}
