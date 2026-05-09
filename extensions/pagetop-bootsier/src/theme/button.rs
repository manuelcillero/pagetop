use pagetop::prelude::*;

use crate::theme::{ButtonAction, ButtonColor, ButtonSize};

/// Componente para crear un **botón**.
///
/// Renderiza un botón con soporte para las variantes disponibles en [`ButtonAction`] (`submit`,
/// `reset` y botón genérico) y con la variedad de estilos del tema a través de [`ButtonColor`] y
/// [`ButtonSize`].
///
/// El comportamiento del botón se establece al crearlo:
///
/// - [`Button::submit()`]: botón de envío (por defecto).
/// - [`Button::reset()`]: botón de restablecimiento de valores.
/// - [`Button::plain()`]: botón genérico sin comportamiento predeterminado.
///
/// El botón puede usarse dentro o fuera de un formulario.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let save = Button::submit(L10n::n("Save"))
///     .with_color(ButtonColor::Background(Color::Primary));
///
/// let cancel = Button::plain(L10n::n("Cancel"))
///     .with_color(ButtonColor::Outline(Color::Secondary));
///
/// let clear = Button::reset(L10n::n("Clear"))
///     .with_size(ButtonSize::Small);
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
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del botón.
    classes: Classes,
    /// Devuelve el comportamiento del botón al activarse.
    kind: ButtonAction,
    /// Devuelve el esquema de color del botón.
    color: ButtonColor,
    /// Devuelve el tamaño visual del botón.
    size: ButtonSize,
    /// Devuelve el nombre del botón.
    name: AttrName,
    /// Devuelve el valor del botón.
    value: AttrValue,
    /// Devuelve la etiqueta del botón.
    label: Attr<L10n>,
    /// Devuelve si el botón recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si el botón está deshabilitado.
    disabled: bool,
}

impl Component for Button {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        let mut classes = "btn".to_string();
        (*self.color()).push_class(&mut classes);
        (*self.size()).push_class(&mut classes);
        self.alter_classes(ClassesOp::Prepend, classes);
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! {
            button
                id=[self.id()]
                type=(self.kind())
                class=[self.classes().get()]
                name=[self.name().get()]
                value=[self.value().get()]
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
    pub fn submit(label: L10n) -> Self {
        Self {
            kind: ButtonAction::Submit,
            label: Attr::some(label),
            ..Default::default()
        }
    }

    /// Crea un botón de **restablecimiento** (`type="reset"`).
    ///
    /// Al pulsarlo, devuelve todos los campos del formulario a sus valores iniciales.
    pub fn reset(label: L10n) -> Self {
        Self {
            kind: ButtonAction::Reset,
            label: Attr::some(label),
            ..Default::default()
        }
    }

    /// Crea un **botón genérico** (`type="button"`).
    ///
    /// No tiene un comportamiento predeterminado sobre el formulario. Su comportamiento puede
    /// definirse mediante JavaScript.
    pub fn plain(label: L10n) -> Self {
        Self {
            kind: ButtonAction::Plain,
            label: Attr::some(label),
            ..Default::default()
        }
    }

    // **< Button BUILDER >*************************************************************************

    /// Establece el identificador único (`id`) del botón.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al botón.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece el esquema de color del botón.
    ///
    /// Usa [`ButtonColor::Background`] para botones sólidos o [`ButtonColor::Outline`] para
    /// variantes con contorno.
    #[builder_fn]
    pub fn with_color(mut self, color: ButtonColor) -> Self {
        self.color = color;
        self
    }

    /// Establece el tamaño visual del botón.
    #[builder_fn]
    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
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

    /// Establece o elimina la etiqueta visible del botón (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: impl Into<Option<L10n>>) -> Self {
        self.label.alter_opt(label.into());
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
