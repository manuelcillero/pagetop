//! Definiciones para crear áreas de texto en formularios.

use crate::prelude::*;

/// Componente para crear un **área de texto** de formulario.
///
/// Permite escribir en un área de texto de más de una línea, con una etiqueta opcional y atributos
/// como el número de filas a presentar, longitud mínima (`minlength`) y máxima (`maxlength`), texto
/// indicativo (`placeholder`) o autocompletado (`autocomplete`).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let descripcion = form::Textarea::new()
///     .with_name("description")
///     .with_label(L10n::n("Description"))
///     .with_rows(Some(8))
///     .with_maxlength(Some(500))
///     .with_placeholder(L10n::n("Write here..."))
///     .with_required(true);
/// ```
///
/// Al enviar el formulario el navegador transmite `name=valor`. Un área de texto siempre envía su
/// valor, incluso si está vacía. En el servidor se deserializa como `String`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     description: String, // Siempre presente; cadena vacía si el usuario no escribió nada.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Textarea {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve el valor inicial del área de texto.
    value: AttrValue,
    /// Devuelve la etiqueta del campo.
    label: Attr<L10n>,
    /// Devuelve el texto de ayuda del campo.
    help_text: Attr<L10n>,
    /// Devuelve el número de filas visibles del área de texto.
    rows: Attr<u16>,
    /// Devuelve la longitud mínima permitida en caracteres.
    minlength: Attr<u16>,
    /// Devuelve la longitud máxima permitida en caracteres.
    maxlength: Attr<u16>,
    /// Devuelve el texto indicativo del área de texto.
    placeholder: Attr<L10n>,
    /// Devuelve la configuración de autocompletado del campo.
    autocomplete: Attr<form::Autocomplete>,
    /// Devuelve si el campo recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el campo es de sólo lectura.
    readonly: bool,
    /// Devuelve si el campo es obligatorio.
    required: bool,
    /// Devuelve si el campo está deshabilitado.
    disabled: bool,
}

#[async_trait]
impl Component for Textarea {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        if let Some(container_id) = self
            .id()
            .or_else(|| self.name().get().map(|n| util::join!("edit-", n)))
        {
            self.alter_prop(PropsOp::ensure_id(container_id));
        }

        // Clases CSS del contenedor del área de texto.
        self.alter_prop(PropsOp::prepend_classes("form-field form-field-textarea"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self.id();
        let textarea_id = container_id
            .as_deref()
            .map(|id| util::join!(id, "-textarea"));

        Ok(html! {
            div (self.props()) {
                @if let Some(label) = self.label().lookup(cx) {
                    label for=[textarea_id.as_deref()] class="form-label" {
                        (label)
                        @if *self.required() {
                            span
                                class="form-required"
                                title=[L10n::l("field_required").lookup(cx)]
                            {
                                "*"
                            }
                        }
                    }
                }
                textarea
                    id=[textarea_id.as_deref()]
                    class="form-control"
                    name=[self.name().get()]
                    rows=[self.rows().get()]
                    minlength=[self.minlength().get()]
                    maxlength=[self.maxlength().get()]
                    placeholder=[self.placeholder().lookup(cx)]
                    autocomplete=[self.autocomplete().get()]
                    autofocus[*self.autofocus()]
                    readonly[*self.readonly()]
                    required[*self.required()]
                    disabled[*self.disabled()]
                {
                    @if let Some(value) = self.value().get() {
                        (value)
                    }
                }
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Textarea {
    // **< Textarea BUILDER >***********************************************************************

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

    /// Establece el nombre del campo (atributo `name`).
    ///
    /// Sin él, el valor del campo no se transmite al servidor al enviar el formulario. Para
    /// deserializar el campo en el servidor es recomendable establecer un `name` explícito.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece el valor inicial del área de texto.
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }

    /// Establece o elimina la etiqueta visible del campo (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: impl Into<Option<L10n>>) -> Self {
        self.label.alter_opt(label.into());
        self
    }

    /// Establece o elimina el texto de ayuda del campo (basta pasar `None` para quitarlo).
    #[builder_fn]
    pub fn with_help_text(mut self, help_text: impl Into<Option<L10n>>) -> Self {
        self.help_text.alter_opt(help_text.into());
        self
    }

    /// Establece el número de filas visibles del área de texto.
    ///
    /// Sin valor o pasando `None`, el área muestra su altura predeterminada, dos filas según el
    /// estándar.
    #[builder_fn]
    pub fn with_rows(mut self, rows: Option<u16>) -> Self {
        self.rows.alter_opt(rows);
        self
    }

    /// Establece la longitud mínima permitida en caracteres.
    #[builder_fn]
    pub fn with_minlength(mut self, minlength: Option<u16>) -> Self {
        self.minlength.alter_opt(minlength);
        self
    }

    /// Establece la longitud máxima permitida en caracteres.
    #[builder_fn]
    pub fn with_maxlength(mut self, maxlength: Option<u16>) -> Self {
        self.maxlength.alter_opt(maxlength);
        self
    }

    /// Establece o elimina el texto indicativo del área de texto (`None` para quitarlo).
    ///
    /// Este texto aparece en el área de texto y desaparece en cuanto el usuario empieza a escribir.
    /// Al ser texto visible para el usuario se acepta [`L10n`] para poder localizarlo.
    #[builder_fn]
    pub fn with_placeholder(mut self, placeholder: impl Into<Option<L10n>>) -> Self {
        self.placeholder.alter_opt(placeholder.into());
        self
    }

    /// Establece la configuración de autocompletado del campo.
    ///
    /// Permite al navegador sugerir o rellenar automáticamente el contenido del área de texto
    /// con valores guardados. Es especialmente útil en áreas con contenido semántico predefinido.
    ///
    /// Usa los métodos de [`form::Autocomplete`] para los valores más habituales. Pasa `None` para
    /// omitir el atributo.
    #[builder_fn]
    pub fn with_autocomplete(mut self, autocomplete: Option<form::Autocomplete>) -> Self {
        self.autocomplete.alter_opt(autocomplete);
        self
    }

    /// Establece si el campo recibe el foco automáticamente al cargar la página.
    #[builder_fn]
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el campo es de sólo lectura.
    #[builder_fn]
    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Establece si el campo es obligatorio.
    #[builder_fn]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Establece si el campo está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
