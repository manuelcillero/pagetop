//! Definiciones para crear listas de selección.

use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;
use crate::theme::form;

// **< Item >***************************************************************************************

/// Elemento individual de [`form::select::Field`] o de [`form::select::Group`].
///
/// Representa un elemento dentro de una lista de selección o de un grupo de elementos de la lista.
/// Cada elemento tiene un valor que se envía al servidor y una etiqueta localizable visible para el
/// usuario.
///
/// Puede marcarse como seleccionado por defecto con [`with_selected()`](Self::with_selected) o
/// deshabilitado de forma independiente al resto usando [`with_disabled()`](Self::with_disabled).
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let item = form::select::Item::new("es", L10n::n("Spanish")).with_selected(true);
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve el valor enviado al servidor cuando se selecciona el elemento.
    value: AttrValue,
    /// Devuelve la etiqueta visible del elemento.
    label: L10n,
    /// Devuelve si el elemento debe aparecer seleccionado por defecto.
    selected: bool,
    /// Devuelve si el elemento está deshabilitado.
    disabled: bool,
}

impl Item {
    /// Crea un nuevo elemento con el valor y la etiqueta indicados.
    pub fn new(value: impl AsRef<str>, label: L10n) -> Self {
        Self {
            value: AttrValue::new(value),
            label,
            selected: false,
            disabled: false,
        }
    }

    // **< Item BUILDER >***************************************************************************

    /// Establece si el elemento aparece seleccionado por defecto.
    ///
    /// En una lista de selección única, el navegador aplica la selección al último elemento marcado
    /// si hay más de uno; mientras que en una lista múltiple se respetan todos los elementos
    /// marcados.
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Establece si el elemento está deshabilitado.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// **< Group >**************************************************************************************

/// Grupo de elementos dentro de [`form::select::Field`].
///
/// Agrupa un conjunto de elementos dentro de una lista de selección con una etiqueta visible. El
/// grupo completo puede deshabilitarse en bloque con [`with_disabled()`](Self::with_disabled).
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let group = form::select::Group::new(L10n::n("Europe"))
///     .with_item(form::select::Item::new("es", L10n::n("Spanish")))
///     .with_item(form::select::Item::new("fr", L10n::n("French")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Group {
    /// Devuelve la etiqueta visible del grupo de elementos.
    label: L10n,
    /// Devuelve los elementos del grupo.
    items: Vec<Item>,
    /// Devuelve si el grupo de elementos está deshabilitado.
    disabled: bool,
}

impl Group {
    /// Crea un nuevo grupo con la etiqueta indicada.
    pub fn new(label: L10n) -> Self {
        Self {
            label,
            ..Self::default()
        }
    }

    // **< Group BUILDER >**************************************************************************

    /// Añade un elemento al grupo. Los elementos se muestran en el orden en que se añaden.
    pub fn with_item(mut self, item: Item) -> Self {
        self.items.push(item);
        self
    }

    /// Establece si el grupo de elementos está deshabilitado en bloque.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// **< Entry >**************************************************************************************

/// Entrada de [`form::select::Field`] con un elemento o un grupo de elementos.
///
/// Cada entrada se crea implícitamente cuando se usa [`form::select::Field::with_item()`] para
/// añadir un elemento individual o [`form::select::Field::with_group()`] para añadir un grupo de
/// elementos a una lista de selección.
///
/// Con [`form::select::Field::entries()`] se pueden recuperar todas las entradas para su
/// renderizado.
#[derive(Clone, Debug)]
pub enum Entry {
    /// Elemento individual.
    Item(Item),
    /// Grupo de elementos.
    Group(Group),
}

// **< Field >**************************************************************************************

/// Componente para crear una **lista de selección**.
///
/// Renderiza un campo para mostrar una lista de elementos con una etiqueta opcional. Permite elegir
/// uno, o más de uno si se activa la selección múltiple con
/// [`with_multiple()`](Self::with_multiple).
///
/// Los elementos individuales se añaden con [`with_item()`](Self::with_item); los grupos de
/// elementos con un encabezado común se añaden con [`with_group()`](Self::with_group). Ambos
/// métodos pueden combinarse libremente.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_bootsier::theme::*;
/// let idioma = form::select::Field::new()
///     .with_name("language")
///     .with_label(L10n::n("Language"))
///     .with_item(form::select::Item::new("", L10n::n("— Choose —")).with_selected(true))
///     .with_group(
///         form::select::Group::new(L10n::n("Europe"))
///             .with_item(form::select::Item::new("es", L10n::n("Spanish")))
///             .with_item(form::select::Item::new("fr", L10n::n("French"))),
///     )
///     .with_group(
///         form::select::Group::new(L10n::n("Americas"))
///             .with_item(form::select::Item::new("en", L10n::n("English")))
///             .with_item(form::select::Item::new("pt", L10n::n("Portuguese"))),
///     )
///     .with_required(true);
/// ```
///
/// Cuando el usuario selecciona un elemento y envía el formulario, el navegador transmite
/// `name=valor`. Si el campo es obligatorio el valor siempre estará presente y puede deserializarse
/// como `String`; si es opcional, usa `Option<String>`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     language: String,            // Siempre presente (campo obligatorio).
///     // language: Option<String>, // None si no se selecciona ninguna opción.
/// }
/// ```
///
/// Con selección múltiple activa, el navegador envía un valor por cada elemento marcado; si no se
/// marca ninguno, no envía nada. Usa `Vec<String>` con `#[serde(default)]`:
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct FormData {
///     #[serde(default)]
///     interests: Vec<String>, // p. ej. ["art", "tech"] o [] si no se marcó ninguna.
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Field {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS del contenedor de la lista de selección.
    classes: Classes,
    /// Devuelve el nombre del campo.
    name: AttrName,
    /// Devuelve la etiqueta del campo.
    label: Attr<L10n>,
    /// Devuelve si la etiqueta se muestra flotante sobre el campo.
    floating_label: bool,
    /// Devuelve el texto de ayuda del campo.
    help_text: Attr<L10n>,
    /// Devuelve las entradas de la lista (elementos individuales y grupos de elementos).
    entries: Vec<Entry>,
    /// Devuelve si la lista permite selección múltiple.
    multiple: bool,
    /// Devuelve el número de filas visibles de la lista de selección.
    rows: Attr<u16>,
    /// Devuelve la configuración de autocompletado del campo.
    autocomplete: Attr<form::Autocomplete>,
    /// Devuelve si la lista recibe el foco automáticamente al cargar la página.
    autofocus: bool,
    /// Devuelve si la selección de un elemento es obligatoria.
    required: bool,
    /// Devuelve si la lista está deshabilitada.
    disabled: bool,
}

impl Component for Field {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup(&mut self, _cx: &Context) {
        if *self.floating_label() {
            self.multiple = false;
            self.rows.alter_opt(None::<u16>);
            self.alter_classes(ClassesOp::Prepend, "form-floating");
        }
        self.alter_classes(ClassesOp::Prepend, "form-field form-field-select");
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let container_id = self
            .id()
            .or_else(|| self.name().get().map(|n| util::join!("edit-", n)));
        let select_id = container_id.as_deref().map(|id| util::join!(id, "-select"));
        let label = match self.label().lookup(cx) {
            Some(text) => html! {
                label for=[select_id.as_deref()] class="form-label" {
                    (text)
                    @if *self.required() {
                        span
                            class="form-required"
                            title=(L10n::t("input_required", &LOCALES_BOOTSIER).using(cx))
                        {
                            "*"
                        }
                    }
                }
            },
            None => html! {},
        };
        Ok(html! {
            div id=[container_id.as_deref()] class=[self.classes().get()] {
                @if !*self.floating_label() {
                    (label)
                }
                select
                    id=[select_id.as_deref()]
                    class="form-select"
                    name=[self.name().get()]
                    multiple[*self.multiple()]
                    size=[self.rows().get()]
                    autocomplete=[self.autocomplete().get()]
                    autofocus[*self.autofocus()]
                    required[*self.required()]
                    disabled[*self.disabled()]
                {
                    @for entry in self.entries() {
                        @match entry {
                            Entry::Item(opt) => {
                                option
                                    value=(opt.value().as_str().unwrap_or(""))
                                    selected[*opt.selected()]
                                    disabled[*opt.disabled()]
                                {
                                    (opt.label().using(cx))
                                }
                            }
                            Entry::Group(group) => {
                                optgroup
                                    label=(group.label().using(cx))
                                    disabled[*group.disabled()]
                                {
                                    @for opt in group.items() {
                                        option
                                            value=(opt.value().as_str().unwrap_or(""))
                                            selected[*opt.selected()]
                                            disabled[*opt.disabled()]
                                        {
                                            (opt.label().using(cx))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                @if *self.floating_label() {
                    (label)
                }
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Field {
    // **< Field BUILDER >***************************************************************************

    /// Establece el identificador único (`id`) del contenedor del campo.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor de la lista de selección.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    /// Establece el nombre del campo (atributo `name`).
    ///
    /// Sin él, el valor seleccionado no se transmite al servidor al enviar el formulario. Para
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

    /// Establece si la etiqueta se muestra flotante sobre el campo.
    ///
    /// Cuando está activo, la etiqueta se superpone al control y permanece flotante siempre que
    /// haya una opción visible.
    ///
    /// Si se usa la etiqueta flotante, el [`setup()`](Self::setup) del componente anulará los
    /// valores establecidos con [`with_multiple()`](Self::with_multiple) y
    /// [`with_rows()`](Self::with_rows) antes del renderizado.
    #[builder_fn]
    pub fn with_floating_label(mut self, floating_label: bool) -> Self {
        self.floating_label = floating_label;
        self
    }

    /// Establece o elimina el texto de ayuda del campo (basta pasar `None` para quitarlo).
    #[builder_fn]
    pub fn with_help_text(mut self, help_text: impl Into<Option<L10n>>) -> Self {
        self.help_text.alter_opt(help_text.into());
        self
    }

    /// Añade un elemento individual a la lista de selección.
    ///
    /// Los elementos y grupos se muestran en el orden en que se añaden.
    #[builder_fn]
    pub fn with_item(mut self, item: Item) -> Self {
        self.entries.push(Entry::Item(item));
        self
    }

    /// Añade un grupo de elementos a la lista de selección.
    ///
    /// Los elementos y grupos se muestran en el orden en que se añaden.
    #[builder_fn]
    pub fn with_group(mut self, group: Group) -> Self {
        self.entries.push(Entry::Group(group));
        self
    }

    /// Establece si el control permite seleccionar varios elementos.
    ///
    /// Al activar la selección múltiple, se muestra una lista en lugar de un desplegable. Se
    /// recomienda combinar con [`with_rows()`](Self::with_rows) para controlar el número de filas
    /// visibles.
    ///
    /// Para un número reducido de elementos con etiquetas descriptivas considera usar
    /// [`form::check::Field`] en su lugar, ofrece una presentación más clara y es más accesible en
    /// pantallas pequeñas.
    ///
    /// Se anula si se usa con [`with_floating_label(true)`](Self::with_floating_label).
    #[builder_fn]
    pub fn with_multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Establece el número de filas visibles de la lista de selección.
    ///
    /// Cuando se establece un valor mayor que 1, el control se muestra como lista en lugar de
    /// desplegable, tanto en modo simple como múltiple. Con `None` se omite el atributo y presenta
    /// el control como desplegable (comportamiento por defecto).
    ///
    /// Es especialmente útil con selección múltiple para controlar el número de filas visibles sin
    /// necesidad de recurrir al desplazamiento.
    ///
    /// Se anula si se usa con [`with_floating_label(true)`](Self::with_floating_label).
    #[builder_fn]
    pub fn with_rows(mut self, rows: Option<u16>) -> Self {
        self.rows.alter_opt(rows);
        self
    }

    /// Establece la configuración de autocompletado del campo.
    ///
    /// Permite al navegador rellenar automáticamente el elemento seleccionado en listas de países
    /// (`"country"`), idiomas (`"language"`), sexo (`"sex"`) u otros campos con valores
    /// predefinidos. En listas de selección múltiples no es útil en la práctica, ya que los
    /// navegadores no gestionan selecciones múltiples con autocompletado.
    ///
    /// Usa los métodos de [`form::Autocomplete`] para los valores más habituales. Pasa `None` para
    /// omitir el atributo.
    #[builder_fn]
    pub fn with_autocomplete(mut self, autocomplete: Option<form::Autocomplete>) -> Self {
        self.autocomplete.alter_opt(autocomplete);
        self
    }

    /// Establece si el campo recibe el foco automáticamente al cargar la página.
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

    /// Establece si el campo está deshabilitado.
    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
