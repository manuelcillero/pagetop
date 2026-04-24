//! Definiciones para crear grupos de casillas de verificación (*check buttons*).

use pagetop::prelude::*;

// **< Item >***************************************************************************************

/// Casilla de verificación individual de un [`form::check::Group`](Group).
///
/// Representa cada casilla de un grupo de casillas de verificación, con una etiqueta localizable
/// visible. Puede marcarse como seleccionada o deshabilitada de forma independiente al resto.
///
/// El parámetro `name` de [`form::check::Item::new()`](Item::new) se combina con el `name` del
/// grupo para componer el atributo `name` de la casilla. Por ejemplo, si el grupo tiene
/// `name=interests` y el ítem se crea con `name=tech`, la casilla tendrá `name=interests_tech`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let item = form::check::Item::new("apple", L10n::n("Apple")).with_checked(true);
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve el nombre que se combina con el del grupo para componer el atributo `name`.
    name: AttrValue,
    /// Devuelve la etiqueta de la casilla.
    label: L10n,
    /// Devuelve si la casilla debe aparecer marcada por defecto.
    checked: bool,
    /// Devuelve si la casilla está deshabilitada.
    disabled: bool,
}

impl Item {
    /// Crea una nueva casilla con el nombre y la etiqueta indicados.
    ///
    /// El parámetro `name` se combina con el del grupo para componer el atributo `name` de la
    /// casilla.
    pub fn new(name: impl AsRef<str>, label: L10n) -> Self {
        Self {
            name: AttrValue::new(name),
            label,
            checked: false,
            disabled: false,
        }
    }

    // **< Item BUILDER >***************************************************************************

    /// Establece si la casilla debe aparecer marcada por defecto.
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Establece si la casilla está deshabilitada.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// **< Group >**************************************************************************************

/// Componente para crear un **grupo de casillas de verificación**.
///
/// Renderiza un conjunto de casillas de verificación donde, a diferencia de un grupo de botones
/// [`form::radio::Group`](crate::theme::form::radio::Group), cada casilla puede marcarse de forma
/// independiente.
///
/// Las casillas se añaden mediante [`with_item()`](Group::with_item) usando instancias de
/// [`form::check::Item`](Item). Si se activa el modo en línea con
/// [`with_inline()`](Group::with_inline), las casillas se disponen horizontalmente.
///
/// El atributo `name` de cada casilla se construye automáticamente combinando el `name` del grupo
/// y el `name` del [`form::check::Item`](Item) con un guion bajo. Por ejemplo, para el grupo con
/// `name=interests` y casillas con `name=art` y `name=tech`, se genera `name=interests_art` y
/// `name=interests_tech`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::prelude::*;
/// let interests = form::check::Group::new()
///     .with_name("interests")
///     .with_label(L10n::n("Areas of interest"))
///     .with_item(form::check::Item::new("art", L10n::n("Art")))
///     .with_item(form::check::Item::new("tech", L10n::n("Technology")))
///     .with_item(form::check::Item::new("science", L10n::n("Science")).with_checked(true));
/// ```
///
/// Cada `name` debe ser único y válido como identificador de campo. Cuando el usuario marca una
/// casilla, el navegador envía algo como `interests_tech=true`; mientras que si no la marca, no
/// envía nada. En el servidor cada campo se deserializa como `bool` con `#[serde(default)]`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     #[serde(default)]
///     interests_art: bool,
///     #[serde(default)]
///     interests_tech: bool,
///     #[serde(default)]
///     interests_science: bool,
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Group {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del contenedor del grupo.
    classes: Classes,
    /// Devuelve el nombre base compartido por todas las casillas del grupo.
    name: AttrName,
    /// Devuelve la etiqueta del grupo.
    label: Attr<L10n>,
    /// Devuelve el texto de ayuda del grupo.
    help_text: Attr<L10n>,
    /// Devuelve las casillas del grupo.
    items: Vec<Item>,
    /// Devuelve si todo el grupo está deshabilitado.
    disabled: bool,
    /// Devuelve si las casillas se muestran en línea horizontalmente.
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
        self.alter_classes(ClassesOp::Prepend, "form-item form-item-checkboxes");
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
                    label class="form-label" { (label) }
                }
                @let item_classes = if *self.inline() {
                    "form-check form-check-inline"
                } else {
                    "form-check"
                };
                @for (item, i) in self.items().iter().zip(1..) {
                    @let i = i.to_string();
                    @let item_id = util::join!(&container_id, "-check-", &i);
                    @let item_name = if let Some(item_name) = item.name().get() {
                        util::join!(&name, "_", &item_name)
                    } else {
                        util::join!(&name, "_", &i)
                    };
                    div class=(item_classes) {
                        input
                            type="checkbox"
                            id=(&item_id)
                            class="form-check-input"
                            name=(&item_name)
                            value="true"
                            checked[*item.checked()]
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

    /// Establece el identificador único (`id`) del grupo de casillas.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor del grupo de casillas.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece el nombre base para el grupo de casillas.
    ///
    /// Se combina con el `name` de cada [`form::check::Item`](Item) para generar el atributo `name`
    /// de cada casilla de verificación. Por ejemplo, con `name=interests` en el grupo y `name=tech`
    /// en el ítem, se genera `name=interests_tech`.
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

    /// Añade una casilla al grupo. Las casillas se muestran en el orden en que se añaden.
    #[builder_fn]
    pub fn with_item(mut self, item: Item) -> Self {
        self.items.push(item);
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

    /// Establece si las casillas se muestran en línea horizontalmente.
    ///
    /// Al activar este modo, se añade la clase `form-check-inline` al contenedor de cada casilla.
    #[builder_fn]
    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
}
