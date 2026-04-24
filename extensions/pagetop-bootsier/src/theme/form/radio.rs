//! Definiciones para crear grupos de botones de opción (*radio buttons*).

use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;

// **< Item >***************************************************************************************

/// Botón de opción individual de un [`form::radio::Group`](Group).
///
/// Representa cada opción de un grupo de opciones exclusivas entre sí, con un valor (el que se
/// envía al servidor), una etiqueta localizable visible y puede marcarse como seleccionada o
/// inicialmente deshabilitada de forma independiente.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let item = form::radio::Item::new("monthly", L10n::n("Monthly")).with_checked(true);
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve el valor enviado al servidor cuando la opción está seleccionada.
    value: AttrValue,
    /// Devuelve la etiqueta de la opción.
    label: L10n,
    /// Devuelve si la opción debe aparecer seleccionada por defecto.
    checked: bool,
    /// Devuelve si la opción está deshabilitada.
    disabled: bool,
}

impl Item {
    /// Crea una nueva opción con el valor y la etiqueta indicados.
    pub fn new(value: impl AsRef<str>, label: L10n) -> Self {
        Self {
            value: AttrValue::new(value),
            label,
            checked: false,
            disabled: false,
        }
    }

    // **< Item BUILDER >***************************************************************************

    /// Establece si la opción aparece seleccionada por defecto.
    ///
    /// Si varias opciones del grupo tienen `checked` activo, sólo la primera se renderizará como
    /// seleccionada; las demás se ignorarán.
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Establece si la opción está inicialmente deshabilitada.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// **< Group >**************************************************************************************

/// Componente para crear un **grupo de botones de opción**.
///
/// Renderiza un grupo de botones de opción [`form::radio::Item`](Item) que comparten el mismo
/// atributo `name`, por lo que sólo puede seleccionarse uno a la vez. Las opciones se añaden con
/// [`with_item()`](Group::with_item).
///
/// Si se activa el modo en línea [`with_inline()`](Group::with_inline), los botones se
/// disponen horizontalmente. El atributo `required` se propaga a todos los botones del grupo para
/// cumplir con la especificación HTML.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let plan = form::radio::Group::new()
///     .with_name("plan")
///     .with_label(L10n::n("Subscription plan"))
///     .with_item(form::radio::Item::new("monthly", L10n::n("Monthly")))
///     .with_item(form::radio::Item::new("annual", L10n::n("Annual")).with_checked(true))
///     .with_required(true);
/// ```
///
/// Cuando el usuario selecciona un botón, el navegador envía algo como `plan=monthly`; si no
/// selecciona ninguno, no envía nada. En el servidor el campo se deserializa como `Option<String>`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     plan: Option<String>, // Some("monthly"), Some("annual"), ..., o None si no se seleccionó.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Group {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del contenedor del grupo.
    classes: Classes,
    /// Devuelve el nombre compartido por todos los botones de opción del grupo.
    name: AttrName,
    /// Devuelve la etiqueta del grupo.
    label: Attr<L10n>,
    /// Devuelve el texto de ayuda del grupo.
    help_text: Attr<L10n>,
    /// Devuelve las opciones del grupo.
    items: Vec<Item>,
    /// Devuelve si la selección de alguna opción del grupo es obligatoria.
    required: bool,
    /// Devuelve si todo el grupo está deshabilitado.
    disabled: bool,
    /// Devuelve si los botones se muestran en línea horizontalmente.
    inline: bool,
}

impl Component for Group {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_classes(ClassesOp::Prepend, "form-item form-item-radios");
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let name = self
            .name()
            .get()
            .unwrap_or_else(|| cx.required_id::<Self>(self.id(), 3));
        let container_id = self.id().unwrap_or_else(|| util::join!("edit-", &name));
        Ok(html! {
            div id=(&container_id) class=[self.classes().get()] {
                @if let Some(label) = self.label().lookup(cx) {
                    label class="form-label" {
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
                @let item_classes = if *self.inline() {
                    "form-check form-check-inline"
                } else {
                    "form-check"
                };
                @let mut do_check = true;
                @for (item, i) in self.items().iter().zip(1..) {
                    @let checked = {
                        let c = *item.checked() && do_check;
                        if c { do_check = false; }
                        c
                    };
                    @let i = i.to_string();
                    @let item_id = util::join!(&container_id, "-radio-", &i);
                    div class=(item_classes) {
                        input
                            type="radio"
                            id=(&item_id)
                            class="form-check-input"
                            name=(&name)
                            value=[item.value().get()]
                            checked[checked]
                            required[*self.required()]
                            disabled[*item.disabled() || *self.disabled()];
                        label class="form-check-label" for=(&item_id) {
                            (item.label().using(cx))
                        }
                    }
                }
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Group {
    // **< Group BUILDER >**************************************************************************

    /// Establece el identificador único (`id`) del grupo de opciones.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor del grupo de opciones.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece el nombre compartido por todos los botones de opción del grupo.
    ///
    /// Todas las opciones [`form::radio::Item`](Item) del grupo llevarán este mismo `name`, lo que
    /// garantiza la exclusividad de la selección. Es imprescindible establecer un `name`; sin él
    /// los botones no se envían al servidor.
    ///
    /// Si se omite, se asigna un nombre generado automáticamente. Para deserializar los campos en
    /// el servidor es recomendable establecer un `name` explícito.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece o elimina la etiqueta visible del grupo (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: impl Into<Option<L10n>>) -> Self {
        self.label.alter_opt(label.into());
        self
    }

    /// Establece o elimina el texto de ayuda del grupo (basta pasar `None` para quitarlo).
    #[builder_fn]
    pub fn with_help_text(mut self, help_text: impl Into<Option<L10n>>) -> Self {
        self.help_text.alter_opt(help_text.into());
        self
    }

    /// Añade una opción al grupo. Las opciones se muestran en el orden en que se añaden.
    #[builder_fn]
    pub fn with_item(mut self, item: Item) -> Self {
        self.items.push(item);
        self
    }

    /// Establece si la selección de alguna opción del grupo es obligatoria.
    ///
    /// El atributo `required` se propaga a todos los botones del grupo para cumplir con la
    /// especificación HTML.
    #[builder_fn]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Establece si todo el grupo está deshabilitado.
    ///
    /// Cuando está activo, se combina con el estado `disabled` de cada [`Item`].
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Establece si los botones se muestran en línea horizontalmente.
    ///
    /// Al activar este modo, se añade la clase `form-check-inline` al contenedor de cada opción.
    #[builder_fn]
    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
}
