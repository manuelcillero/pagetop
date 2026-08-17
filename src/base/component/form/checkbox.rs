use crate::prelude::*;

/// Componente para crear una **casilla de verificación** o un **interruptor** (*toggle switch*).
///
/// Renderiza un control binario (marcado/no marcado) en dos variantes, por defecto como casilla de
/// verificación estándar, y también como interruptor ([`Checkbox::switch()`]).
///
/// Se puede mostrar en línea con otros controles usando [`with_inline()`](Checkbox::with_inline), o
/// justificar a la derecha del contenedor invirtiendo el orden de la etiqueta y el control usando
/// [`with_reverse()`](Checkbox::with_reverse).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let accept_terms = form::Checkbox::new()
///     .with_name("terms_accepted")
///     .with_label(Lc::n("I accept the terms and conditions"))
///     .with_required(true);
///
/// let notifications = form::Checkbox::switch()
///     .with_name("notifications_enabled")
///     .with_label(Lc::n("Receive email notifications"))
///     .with_checked(true);
/// ```
///
/// Cuando el control está activo, el navegador envía `name=true`; si no lo está, no envía nada.
/// En el servidor el campo se deserializa como `bool` con `#[serde(default)]`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     #[serde(default)]
///     terms_accepted: bool,        // true = marcada, false = no marcada.
///     #[serde(default)]
///     notifications_enabled: bool, // true = activo, false = inactivo.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Checkbox {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la variante visual del control.
    checkbox_kind: form::CheckboxKind,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve la etiqueta del control.
    label: Lc,
    /// Devuelve si el control debe estar marcado/activo por defecto.
    checked: bool,
    /// Devuelve si el control recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el campo es obligatorio.
    required: bool,
    /// Devuelve si el control está deshabilitado.
    disabled: bool,
    /// Devuelve si el control se muestra en línea con otros controles.
    inline: bool,
    /// Devuelve si el control y su etiqueta se justifican a la derecha del contenedor.
    reverse: bool,
}

#[async_trait]
impl Component for Checkbox {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &Context) {
        // Asegura `name` e `id`.
        // Si falta uno se deriva del otro; si faltan ambos se genera un valor único.
        let name = self
            .name()
            .get()
            .unwrap_or_else(|| cx.required_id::<Self>(self.id(), 1));
        self.alter_name(&name);
        let container_id = self.id().unwrap_or_else(|| util::join!("edit-", &name));
        self.alter_prop(PropsOp::ensure_id(container_id));

        // Clases CSS del contenedor de la casilla de verificación.
        let mut classes = String::from("form-field form-check");
        if *self.checkbox_kind() == form::CheckboxKind::Switch {
            classes.push_str(" form-switch");
        }
        if *self.inline() {
            classes.push_str(" form-check-inline");
        }
        if *self.reverse() {
            classes.push_str(" form-check-reverse");
        }
        self.alter_prop(PropsOp::prepend_classes(classes));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // En `setup()` se garantiza que `name` e `id` están definidos antes del renderizado.
        let name = self.name().get().unwrap();
        let container_id = self.id().unwrap();

        let checkbox_id = util::join!(&container_id, "-checkbox");
        let is_switch = *self.checkbox_kind() == form::CheckboxKind::Switch;

        Ok(html! {
            div (self.props()) {
                input
                    type="checkbox"
                    role=[is_switch.then_some("switch")]
                    id=(&checkbox_id)
                    class="form-check-input"
                    name=(&name)
                    value="true"
                    checked[*self.checked()]
                    autofocus[*self.autofocus()]
                    required[*self.required()]
                    disabled[*self.disabled()];
                @if let Some(label) = self.label().lookup(cx) {
                    label class="form-check-label" for=(&checkbox_id) {
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
            }
        })
    }
}

impl Checkbox {
    /// Crea una casilla de verificación estándar.
    pub fn check() -> Self {
        Self::default()
    }

    /// Crea un interruptor de encendido/apagado (*toggle switch*).
    pub fn switch() -> Self {
        Self {
            checkbox_kind: form::CheckboxKind::Switch,
            ..Self::default()
        }
    }

    // **< Checkbox BUILDER >***********************************************************************

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

    /// Establece la variante visual del control.
    #[builder_fn]
    pub fn with_kind(mut self, kind: form::CheckboxKind) -> Self {
        self.checkbox_kind = kind;
        self
    }

    /// Establece el nombre del campo (atributo `name`).
    ///
    /// Si se omite, se asigna un identificador generado automáticamente. Para deserializar el campo
    /// en el servidor es recomendable establecer un `name` explícito.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece la etiqueta visible del control (usa [`Lc::none()`] para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece si el control debe aparecer marcado/activo por defecto.
    #[builder_fn]
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Establece si el control recibe el foco automáticamente al cargar la página.
    #[builder_fn]
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Establece si el campo es obligatorio.
    #[builder_fn]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Establece si el control está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Establece si el control se muestra en línea con otros controles.
    ///
    /// Al activar este modo, se añade la clase `form-check-inline` al contenedor, lo que permite
    /// alinear varios controles horizontalmente.
    #[builder_fn]
    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    /// Establece si el control y su etiqueta se justifican a la derecha del contenedor.
    ///
    /// Al activar este modo, se añade la clase `form-check-reverse` al contenedor.
    #[builder_fn]
    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }
}
