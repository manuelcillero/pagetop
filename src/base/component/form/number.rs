use crate::prelude::*;

/// Componente para crear un **campo numérico**.
///
/// Renderiza un campo de entrada numérica (`input type="number"`) con una etiqueta opcional y un
/// texto de ayuda. El navegador restringe la entrada a valores numéricos y ofrece controles nativos
/// para incrementar o decrementar el valor.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let quantity = form::Number::new()
///     .with_name("quantity")
///     .with_label(Lc::n("Quantity"))
///     .with_min(Some(1))
///     .with_max(Some(100))
///     .with_value(Some(1));
/// ```
///
/// Al enviar el formulario el navegador transmite `name=valor`. Un campo numérico siempre envía su
/// valor. En el servidor se deserializa como `u64`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     quantity: u64, // Siempre presente con el valor numérico introducido.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Number {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve el valor inicial del campo.
    #[getters(copy)]
    value: Option<u64>,
    /// Devuelve la etiqueta del campo.
    label: Lc,
    /// Devuelve el texto de ayuda del campo.
    help_text: Lc,
    /// Devuelve el valor mínimo permitido.
    #[getters(copy)]
    min: Option<u64>,
    /// Devuelve el valor máximo permitido.
    #[getters(copy)]
    max: Option<u64>,
    /// Devuelve el incremento entre valores del campo.
    #[getters(copy)]
    step: Option<u64>,
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
impl Component for Number {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        if let Some(container_id) = self
            .id()
            .or_else(|| self.name().as_deref().map(|n| util::join!("edit-", n)))
        {
            self.alter_prop(PropsOp::ensure_id(container_id));
        }

        // Clases CSS del contenedor del campo numérico.
        self.alter_prop(PropsOp::prepend_classes("form-field form-field-number"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self.id();
        let input_id = container_id.as_deref().map(|id| util::join!(id, "-input"));
        Ok(html! {
            div (self.props()) {
                @if let Some(label) = self.label().lookup(cx) {
                    label for=[input_id.as_deref()] class="form-label" {
                        (label)
                        @if *self.required() {
                            span
                                class="form-required"
                                title=[Lc::l("field_required").lookup(cx)]
                            {
                                "*"
                            }
                        }
                    }
                }
                input
                    type="number"
                    id=[input_id.as_deref()]
                    class="form-control"
                    name=[self.name().as_deref()]
                    min=[self.min()]
                    max=[self.max()]
                    step=[self.step()]
                    value=[self.value()]
                    autofocus[*self.autofocus()]
                    readonly[*self.readonly()]
                    required[*self.required()]
                    disabled[*self.disabled()];
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Number {
    // **< Number BUILDER >************************************************************************

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

    /// Establece el valor inicial del campo.
    #[builder_fn]
    pub fn with_value(mut self, value: impl Into<Option<u64>>) -> Self {
        self.value = value.into();
        self
    }

    /// Establece la etiqueta visible del campo (usa [`Lc::none()`] para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece el texto de ayuda del campo (usa [`Lc::none()`] para quitarlo).
    #[builder_fn]
    pub fn with_help_text(mut self, help_text: Lc) -> Self {
        self.help_text = help_text;
        self
    }

    /// Establece el valor mínimo permitido (`None` para no imponer mínimo).
    #[builder_fn]
    pub fn with_min(mut self, min: impl Into<Option<u64>>) -> Self {
        self.min = min.into();
        self
    }

    /// Establece el valor máximo permitido (`None` para no imponer máximo).
    #[builder_fn]
    pub fn with_max(mut self, max: impl Into<Option<u64>>) -> Self {
        self.max = max.into();
        self
    }

    /// Establece el incremento entre valores del campo.
    ///
    /// Pasar `None` omite el atributo `step` y deja que el navegador aplique su valor por defecto
    /// (normalmente `1`).
    #[builder_fn]
    pub fn with_step(mut self, step: impl Into<Option<u64>>) -> Self {
        self.step = step.into();
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
