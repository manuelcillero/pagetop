//! Definiciones para crear grupos de casillas de verificación (*check buttons*).

use crate::prelude::*;

// **< Item >***************************************************************************************

/// Casilla de verificación individual de un [`Field`].
///
/// Representa cada casilla de un grupo de casillas de verificación, con una etiqueta localizable
/// visible. Puede marcarse como seleccionada o deshabilitada de forma independiente al resto.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let item = form::check::Item::new("apple", Lc::n("Apple")).with_checked(true);
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve el valor enviado al servidor cuando la casilla está marcada.
    value: AttrValue,
    /// Devuelve la etiqueta de la casilla.
    label: Lc,
    /// Devuelve si la casilla debe aparecer marcada por defecto.
    checked: bool,
    /// Devuelve si la casilla está deshabilitada.
    disabled: bool,
}

impl Item {
    /// Crea una nueva casilla con el valor y la etiqueta indicados.
    pub fn new(value: impl AsRef<str>, label: Lc) -> Self {
        Self {
            value: AttrValue::new(value),
            label,
            checked: false,
            disabled: false,
        }
    }

    // **< Item BUILDER >***************************************************************************

    /// Establece si la casilla debe aparecer marcada por defecto.
    #[builder_fn]
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Establece si la casilla está deshabilitada.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// **< Field >**************************************************************************************

/// Componente para crear un **grupo de casillas de verificación**.
///
/// Renderiza una lista de opciones de la que el usuario puede marcar cero, una o varias. Todas las
/// casillas comparten el mismo `name` (el del grupo); cada una envía como valor el de su
/// [`form::check::Item`] cuando está marcada. Las opciones se añaden con [`with_item()`]. Si se
/// activa el modo en línea con [`with_inline()`], las casillas se disponen horizontalmente.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let interests = form::check::Field::new()
///     .with_name("interests")
///     .with_label(Lc::n("Areas of interest"))
///     .with_item(form::check::Item::new("art", Lc::n("Art")))
///     .with_item(form::check::Item::new("tech", Lc::n("Technology")))
///     .with_item(form::check::Item::new("science", Lc::n("Science")).with_checked(true));
/// ```
///
/// El navegador envía una entrada por cada casilla marcada, todas bajo la misma clave (por ejemplo,
/// si el usuario marca "Technology" y "Science", `interests=tech&interests=science`) y ninguna si
/// no marca ninguna. El servidor no necesita conocer de antemano qué opciones existían, lo que hace
/// de `Field` la opción adecuada también para listas de opciones dinámicas (por ejemplo, cargadas
/// de una base de datos). `axum::extract::Form` (basado en `serde_urlencoded`) no deserializa
/// claves repetidas en un `Vec<T>`; hace falta un extractor que sí lo haga, como [`serde_qs`]:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     #[serde(default)]
///     interests: Vec<String>, // ["tech", "science"], o [] si no se marcó ninguna.
/// }
/// ```
///
/// [`with_item()`]: Field::with_item
/// [`with_inline()`]: Field::with_inline
/// [`serde_qs`]: https://docs.rs/serde_qs
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Field {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el nombre compartido por todas las casillas del grupo.
    name: AttrName,
    /// Devuelve la etiqueta del grupo.
    label: Attr<Lc>,
    /// Devuelve el texto de ayuda del grupo.
    help_text: Attr<Lc>,
    /// Devuelve las casillas del grupo.
    items: Vec<Item>,
    /// Devuelve si todo el grupo está deshabilitado.
    disabled: bool,
    /// Devuelve si las casillas se muestran en línea horizontalmente.
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

        // Clases CSS del contenedor del grupo de casillas.
        self.alter_prop(PropsOp::prepend_classes("form-field form-field-checkboxes"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // En `setup()` se garantiza que `name` e `id` están definidos antes del renderizado.
        let name = self.name().get().unwrap();
        let container_id = self.id().unwrap();

        Ok(html! {
            div (self.props()) {
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
                    div class=(item_classes) {
                        input
                            type="checkbox"
                            id=(&item_id)
                            class="form-check-input"
                            name=(&name)
                            value=[item.value().get()]
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

impl Field {
    // **< Field BUILDER >**************************************************************************

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

    /// Establece el nombre compartido por todas las casillas del grupo.
    ///
    /// Todas las casillas [`form::check::Item`](Item) del grupo llevarán este mismo `name`. Si se
    /// omite, se asigna un nombre generado automáticamente. Para deserializar los campos en el
    /// servidor es recomendable establecer un `name` explícito.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    /// Establece o elimina la etiqueta visible del grupo (basta pasar `None` para quitarla).
    #[builder_fn]
    pub fn with_label(mut self, label: impl Into<Option<Lc>>) -> Self {
        self.label.alter_opt(label.into());
        self
    }

    /// Establece o elimina el texto de ayuda del grupo (basta pasar `None` para quitarlo).
    #[builder_fn]
    pub fn with_help_text(mut self, help_text: impl Into<Option<Lc>>) -> Self {
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
