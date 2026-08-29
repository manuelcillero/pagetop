//! Definiciones para crear grupos de botones de opción (*radio buttons*).

use crate::prelude::*;

// **< Item >***************************************************************************************

/// Botón de opción individual de un [`Field`].
///
/// Representa cada opción de un grupo de opciones exclusivas entre sí, con un valor (el que se
/// envía al servidor), una etiqueta localizable visible y puede marcarse como seleccionada o
/// inicialmente deshabilitada de forma independiente.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let item = form::radio::Item::new("monthly", Lc::n("Monthly")).with_checked(true);
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve el valor enviado al servidor cuando la opción está seleccionada.
    value: AttrValue,
    /// Devuelve la etiqueta de la opción.
    label: Lc,
    /// Devuelve si la opción debe aparecer seleccionada por defecto.
    checked: bool,
    /// Devuelve si la opción está deshabilitada.
    disabled: bool,
}

#[builder_impl]
impl Item {
    /// Crea una nueva opción con el valor y la etiqueta indicados.
    pub fn new(value: impl AsRef<str>, label: Lc) -> Self {
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

// **< Field >**************************************************************************************

/// Componente para crear un **grupo de botones de opción**.
///
/// Renderiza un grupo de botones de opción [`form::radio::Item`] que comparten el mismo atributo
/// `name`, por lo que sólo puede seleccionarse uno a la vez. Las opciones se añaden con
/// [`with_item()`](Field::with_item).
///
/// Si se activa el modo en línea [`with_inline()`](Field::with_inline), los botones se disponen
/// horizontalmente. El atributo `required` se propaga a todos los botones del grupo para cumplir
/// con la especificación HTML.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let plan = form::radio::Field::new()
///     .with_name("plan")
///     .with_label(Lc::n("Subscription plan"))
///     .with_item(form::radio::Item::new("monthly", Lc::n("Monthly")))
///     .with_item(form::radio::Item::new("annual", Lc::n("Annual")).with_checked(true))
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
pub struct Field {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el nombre compartido por todos los botones de opción del grupo.
    name: AttrName,
    /// Devuelve la etiqueta del grupo.
    label: Lc,
    /// Devuelve el texto de ayuda del grupo.
    help_text: Lc,
    /// Devuelve las opciones del grupo.
    items: Vec<Item>,
    /// Devuelve si la selección de alguna opción del grupo es obligatoria.
    required: bool,
    /// Devuelve si todo el grupo está deshabilitado.
    disabled: bool,
    /// Devuelve si los botones se muestran en línea horizontalmente.
    inline: bool,
}

#[async_trait]
impl Component for Field {
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
            .unwrap_or_else(|| cx.required_id::<Self>(self.id(), 3));
        self.alter_name(&name);
        let container_id = self.id().unwrap_or_else(|| util::join!("edit-", &name));
        self.alter_prop(PropsOp::ensure_id(container_id));

        // Clases CSS del contenedor del grupo de opciones.
        self.alter_prop(PropsOp::prepend_classes("form-field form-field-radios"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // En `setup()` se garantiza que `name` e `id` están definidos antes del renderizado.
        let name = self.name().as_deref().unwrap();
        let container_id = self.id().unwrap();

        Ok(html! {
            div (self.props()) {
                @if let Some(label) = self.label().lookup(cx) {
                    label class="form-label" {
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
                            name=(name)
                            value=[item.value().as_deref()]
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

#[builder_impl]
impl Field {
    // **< Field BUILDER >**************************************************************************

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

    /// Establece el nombre compartido por todos los botones de opción del grupo.
    ///
    /// Todas las opciones [`form::radio::Item`](Item) del grupo llevarán este mismo `name`, lo que
    /// garantiza la exclusividad de la selección. Es imprescindible establecer un `name`; sin él
    /// los botones no se envían al servidor.
    ///
    /// Si se omite, se asigna un nombre generado automáticamente. Para deserializar los campos en
    /// el servidor es recomendable establecer un `name` explícito.
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece la etiqueta visible del grupo (usa [`Lc::none()`] para quitarla).
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece el texto de ayuda del grupo (usa [`Lc::none()`] para quitarlo).
    pub fn with_help_text(mut self, help_text: Lc) -> Self {
        self.help_text = help_text;
        self
    }

    /// Añade una opción al grupo. Las opciones se muestran en el orden en que se añaden.
    pub fn with_item(mut self, item: Item) -> Self {
        self.items.push(item);
        self
    }

    /// Establece si la selección de alguna opción del grupo es obligatoria.
    ///
    /// El atributo `required` se propaga a todos los botones del grupo para cumplir con la
    /// especificación HTML.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Establece si todo el grupo está deshabilitado.
    ///
    /// Cuando está activo, se combina con el estado `disabled` de cada [`Item`].
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Establece si los botones se muestran en línea horizontalmente.
    ///
    /// Al activar este modo, se añade la clase `form-check-inline` al contenedor de cada opción.
    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
}
