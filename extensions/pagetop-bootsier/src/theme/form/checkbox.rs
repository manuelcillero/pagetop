use pagetop::prelude::*;

use crate::theme::form;
use crate::LOCALES_BOOTSIER;

/// Componente para crear una **casilla de verificación** o un **interruptor** (*toggle switch*).
///
/// Renderiza un control binario (marcado/no marcado) en dos variantes visuales, por defecto se
/// muestra como una casilla de verificación estándar, pero también puede renderizarse como un
/// interruptor de encendido/apagado ([`Checkbox::switch()`]).
///
/// Se puede mostrar en línea con otros controles usando [`with_inline()`](Checkbox::with_inline), o
/// justificar a la derecha del contenedor invirtiendo el orden de la etiqueta y el control usando
/// [`with_reverse()`](Checkbox::with_reverse).
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let accept_terms = form::Checkbox::check() // También sirve new() o default().
///     .with_name("terms_accepted")
///     .with_label(L10n::n("I accept the terms and conditions"))
///     .with_required(true);
///
/// let notifications = form::Checkbox::switch()
///     .with_name("notifications_enabled")
///     .with_label(L10n::n("Receive email notifications"))
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
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del contenedor del control.
    classes: Classes,
    /// Devuelve la variante visual del control.
    checkbox_kind: form::CheckboxKind,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve la etiqueta del control.
    label: Attr<L10n>,
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

impl Component for Checkbox {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        let mut classes = "form-item form-check".to_string();
        if *self.checkbox_kind() == form::CheckboxKind::Switch {
            classes.push_str(" form-switch");
        }
        if *self.inline() {
            classes.push_str(" form-check-inline");
        }
        if *self.reverse() {
            classes.push_str(" form-check-reverse");
        }
        self.alter_classes(ClassesOp::Prepend, classes);
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let name = self
            .name()
            .get()
            .unwrap_or_else(|| cx.required_id::<Self>(self.id(), 1));
        let container_id = self.id().unwrap_or_else(|| util::join!("edit-", &name));
        let checkbox_id = util::join!(&container_id, "-checkbox");
        let is_switch = *self.checkbox_kind() == form::CheckboxKind::Switch;
        Ok(html! {
            div id=(&container_id) class=[self.classes().get()] {
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
                                title=(L10n::t("input_required", &LOCALES_BOOTSIER).using(cx))
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

    /// Establece el identificador único (`id`) del control.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor del control.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
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

    /// Establece o elimina la etiqueta visible del control (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: impl Into<Option<L10n>>) -> Self {
        self.label.alter_opt(label.into());
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
