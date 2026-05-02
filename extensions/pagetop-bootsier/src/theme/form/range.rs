use pagetop::prelude::*;

/// Componente para crear un **control deslizante** de rango.
///
/// Renderiza una barra deslizante con una etiqueta opcional y un texto de ayuda. Permite
/// seleccionar un valor de entre una lista de valores posibles, acotados por un valor mínimo y
/// máximo, con un paso opcional entre valores.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let volume = form::Range::new()
///     .with_name("volume")
///     .with_label(L10n::n("Volume"))
///     .with_min(Some(0.0))
///     .with_max(Some(100.0))
///     .with_step(Some(5.0))
///     .with_value(Some(50.0));
/// ```
///
/// Al enviar el formulario el navegador transmite `name=valor`. Un control deslizante siempre
/// envía su valor. En el servidor se deserializa como `f64`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     volume: f64, // Siempre presente con el valor numérico seleccionado.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Range {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del contenedor del control deslizante.
    classes: Classes,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve la etiqueta del campo.
    label: Attr<L10n>,
    /// Devuelve el texto de ayuda del campo.
    help_text: Attr<L10n>,
    /// Devuelve el valor mínimo permitido.
    min: Attr<f64>,
    /// Devuelve el valor máximo permitido.
    max: Attr<f64>,
    /// Devuelve el incremento entre valores del campo.
    step: Attr<f64>,
    /// Devuelve el valor inicial del campo.
    value: Attr<f64>,
    /// Devuelve si el control recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el control está deshabilitado.
    disabled: bool,
}

impl Component for Range {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_classes(ClassesOp::Prepend, "form-field form-field-range");
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self
            .id()
            .or_else(|| self.name().get().map(|n| util::join!("edit-", n)));
        let range_id = container_id.as_deref().map(|id| util::join!(id, "-range"));
        Ok(html! {
            div id=[container_id.as_deref()] class=[self.classes().get()] {
                @if let Some(label) = self.label().lookup(cx) {
                    label for=[range_id.as_deref()] class="form-label" { (label) }
                }
                input
                    type="range"
                    id=[range_id.as_deref()]
                    class="form-range"
                    name=[self.name().get()]
                    min=[self.min().get()]
                    max=[self.max().get()]
                    step=[self.step().get()]
                    value=[self.value().get()]
                    autofocus[*self.autofocus()]
                    disabled[*self.disabled()];
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Range {
    // **< Range BUILDER >**************************************************************************

    /// Establece el identificador único (`id`) del contenedor del control deslizante.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor del control deslizante.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
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

    /// Establece el valor mínimo del rango.
    ///
    /// Pasar `None` omite el atributo `min` y deja que el navegador aplique su valor por defecto.
    #[builder_fn]
    pub fn with_min(mut self, min: Option<f64>) -> Self {
        self.min.alter_opt(min);
        self
    }

    /// Establece el valor máximo del rango.
    ///
    /// Pasar `None` omite el atributo `max` y deja que el navegador aplique su valor por defecto.
    #[builder_fn]
    pub fn with_max(mut self, max: Option<f64>) -> Self {
        self.max.alter_opt(max);
        self
    }

    /// Establece el incremento entre valores del campo.
    ///
    /// Pasar `None` omite el atributo `step` y deja que el navegador aplique su valor por defecto
    /// (normalmente `1`).
    #[builder_fn]
    pub fn with_step(mut self, step: Option<f64>) -> Self {
        self.step.alter_opt(step);
        self
    }

    /// Establece el valor inicial del campo.
    ///
    /// Pasar `None` omite el atributo `value` y deja que el navegador aplique su valor por defecto
    /// (normalmente el punto medio del rango).
    #[builder_fn]
    pub fn with_value(mut self, value: Option<f64>) -> Self {
        self.value.alter_opt(value);
        self
    }

    /// Establece si el control recibe el foco automáticamente al cargar la página.
    #[builder_fn]
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el control está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
