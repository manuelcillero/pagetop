//! Definiciones para crear campos de selección de archivos en formularios.

use crate::prelude::*;

/// Componente para crear un **campo de selección de archivos** de formulario (`type="file"`).
///
/// Permite elegir uno o varios archivos del equipo para enviarlos con el formulario, con una
/// etiqueta opcional, un texto de ayuda y los tipos de archivo aceptados (`accept`).
///
/// El formulario debe usar la codificación [`form::Enctype::Multipart`]; sin ella, el navegador
/// envía sólo el nombre del archivo.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let form = Form::new()
///     .with_action("/documents")
///     .with_enctype(form::Enctype::Multipart)
///     .with_child(
///         form::File::new()
///             .with_name("attachments")
///             .with_label(Lc::n("Attachments"))
///             .with_accept(".pdf,image/*")
///             .with_multiple(true)
///             .with_help_text(Lc::n("PDF documents or images.")),
///     )
///     .with_child(Button::submit(Lc::n("Send")));
/// ```
///
/// Al enviar el formulario, cada archivo llega en una parte propia del cuerpo con el `name` del
/// campo (varias con el mismo nombre si se eligen varios archivos). Si no se elige ninguno, el
/// navegador envía igualmente una parte vacía, sin nombre de archivo. En el servidor,
/// [`web::Upload::receive()`] recibe el envío completo, guarda los archivos e ignora esa parte
/// vacía.
///
/// `accept` es sólo una ayuda para el usuario al elegir el archivo: no impide enviar otros tipos,
/// así que el servidor debe comprobarlos igualmente con [`web::Upload::with_extensions()`].
/// [`web::Upload::accept()`] devuelve esas mismas extensiones con el formato de `accept`.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct File {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve la etiqueta del campo.
    label: Lc,
    /// Devuelve el texto de ayuda del campo.
    help_text: Lc,
    /// Devuelve los tipos de archivo aceptados.
    accept: AttrValue,
    /// Devuelve si se pueden elegir varios archivos.
    multiple: bool,
    /// Devuelve si el campo recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el campo es obligatorio.
    required: bool,
    /// Devuelve si el campo está deshabilitado.
    disabled: bool,
}

#[async_trait]
impl Component for File {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &mut Context) {
        if let Some(container_id) = self
            .id()
            .or_else(|| self.name().as_deref().map(|n| util::join!("edit-", n)))
        {
            self.alter_prop(PropsOp::ensure_id(container_id));
        }

        // Clases CSS del contenedor del campo.
        self.alter_prop(PropsOp::prepend_classes("form-field form-field-file"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self.id();
        let input_id = container_id.as_deref().map(|id| util::join!(id, "-file"));
        let help = form::parts::Help::new(self.help_text(), input_id.as_deref());
        let label = form::parts::Label::new(self.label(), input_id.as_deref())
            .with_required(*self.required());

        Ok(html! {
            div (self.props().unpack(cx)) {
                (label.render(cx))
                input
                    type="file"
                    id=[input_id.as_deref()]
                    aria-describedby=[help.id()]
                    class="form-control"
                    name=[self.name().as_deref()]
                    accept=[self.accept().as_deref()]
                    multiple[*self.multiple()]
                    autofocus[*self.autofocus()]
                    required[*self.required()]
                    disabled[*self.disabled()];
                (help.render(cx))
            }
        })
    }
}

#[builder_impl]
impl File {
    // **< File BUILDER >***************************************************************************

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

    /// Establece el nombre del campo (atributo `name`).
    ///
    /// Sin él, los archivos no se transmiten al servidor al enviar el formulario. Es el nombre que
    /// devuelve `name()` en cada parte recibida con [`web::Multipart`].
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece la etiqueta visible del campo (usa [`Lc::none()`] para quitarla).
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece el texto de ayuda del campo (usa [`Lc::none()`] para quitarlo).
    pub fn with_help_text(mut self, help_text: Lc) -> Self {
        self.help_text = help_text;
        self
    }

    /// Establece los tipos de archivo aceptados (atributo `accept`), separados por comas.
    ///
    /// Admite extensiones (`.pdf`), tipos MIME (`application/pdf`) y familias de tipos (`image/*`),
    /// p. ej. `".pdf,image/*"`. Una cadena vacía quita el atributo y se acepta cualquier archivo.
    pub fn with_accept(mut self, accept: impl AsRef<str>) -> Self {
        self.accept.alter_str(accept);
        self
    }

    /// Establece si se pueden elegir varios archivos a la vez.
    pub fn with_multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Establece si el campo recibe el foco automáticamente al cargar la página.
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el campo es obligatorio.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Establece si el campo está deshabilitado.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
