use crate::prelude::*;

/// Componente para crear un **botón**.
///
/// Renderiza un botón con soporte para las variantes:
///
/// - [`Button::submit()`]: botón de envío (por defecto).
/// - [`Button::reset()`]: botón de restablecimiento de valores.
/// - [`Button::plain()`]: botón genérico sin comportamiento predeterminado.
///
/// Un botón puede usarse dentro o fuera de un formulario.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let save   = Button::submit(Lc::n("Save"));
/// let cancel = Button::plain(Lc::n("Cancel"));
/// let clear  = Button::reset(Lc::n("Clear"));
/// ```
///
/// Cuando el botón activa el envío, el navegador incluye el par `name=value` en los datos del
/// formulario **sólo si** tiene el atributo `name` definido. Es la forma habitual de identificar
/// cuál de los botones de envío fue pulsado. En el servidor se deserializa como `Option<String>`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     #[serde(default)]
///     action: Option<String>, // p. ej., "save" o "delete"; `None` si el botón no tenía `name`.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Button {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el comportamiento del botón al activarse.
    kind: button::ButtonKind,
    /// Devuelve el estilo visual del botón.
    #[getters(copy)]
    style: button::ButtonStyle,
    /// Devuelve el nombre del botón.
    name: AttrName,
    /// Devuelve el valor del botón.
    value: AttrValue,
    /// Devuelve la etiqueta del botón.
    label: Lc,
    /// Devuelve el texto emergente del botón (atributo `title`).
    title: Lc,
    /// Devuelve si el botón recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el botón está deshabilitado.
    disabled: bool,
}

#[async_trait]
impl Component for Button {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        use button::ButtonStyle;

        self.alter_prop(PropsOp::prepend_classes(match self.style() {
            ButtonStyle::None => "button".to_string(),
            ButtonStyle::Solid(intent) => util::join!("button button-", intent.as_str()),
            ButtonStyle::Outline(intent) => util::join!("button button-outline-", intent.as_str()),
            ButtonStyle::Link => "button button-link".to_string(),
        }));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            button
                type=(self.kind())
                (self.props())
                name=[self.name().as_deref()]
                value=[self.value().as_deref()]
                title=[self.title().lookup(cx)]
                autofocus[*self.autofocus()]
                disabled[*self.disabled()]
            {
                @if let Some(label) = self.label().lookup(cx) {
                    (label)
                }
            }
        })
    }
}

impl Button {
    /// Crea un botón de **envío** (`type="submit"`).
    ///
    /// Es la acción predeterminada al pulsar un botón en la mayoría de los formularios: envía los
    /// datos al servidor.
    pub fn submit(label: Lc) -> Self {
        Self {
            kind: button::ButtonKind::Submit,
            label,
            ..Default::default()
        }
    }

    /// Crea un botón de **restablecimiento** (`type="reset"`).
    ///
    /// Al pulsarlo, devuelve todos los campos del formulario a sus valores iniciales.
    pub fn reset(label: Lc) -> Self {
        Self {
            kind: button::ButtonKind::Reset,
            label,
            ..Default::default()
        }
    }

    /// Crea un **botón genérico** (`type="button"`).
    ///
    /// No tiene un comportamiento predeterminado sobre el formulario. Su comportamiento puede
    /// definirse mediante JavaScript.
    pub fn plain(label: Lc) -> Self {
        Self {
            kind: button::ButtonKind::Plain,
            label,
            ..Default::default()
        }
    }

    // **< Button BUILDER >*************************************************************************

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

    /// Establece el comportamiento del botón al activarse.
    #[builder_fn]
    pub fn with_kind(mut self, kind: button::ButtonKind) -> Self {
        self.kind = kind;
        self
    }

    /// Establece el estilo visual del botón (usa [`button::ButtonStyle::None`] para quitarlo).
    #[builder_fn]
    pub fn with_style(mut self, style: button::ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Establece el nombre del botón (atributo `name`).
    ///
    /// Cuando el formulario tiene varios botones de envío, el navegador incluye en el envío el par
    /// `name=value` sólo del botón que activó el formulario. Permite identificar cuál fue pulsado.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece el valor del botón (atributo `value`).
    ///
    /// Es el dato que el navegador transmite al servidor junto con el `name` cuando este botón
    /// activa el envío. Útil para distinguir entre varios botones de envío en un mismo formulario.
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }

    /// Establece la etiqueta visible del botón (usa [`Lc::none()`] para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece el texto emergente del botón (usa [`Lc::none()`] para quitarlo).
    #[builder_fn]
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece si el botón recibe el foco automáticamente al cargar la página.
    #[builder_fn]
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el botón está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
