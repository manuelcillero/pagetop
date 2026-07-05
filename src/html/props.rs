use crate::html::maud::{Escaper, Render};
use crate::{AutoDefault, CowStr, builder_fn, util};

use std::fmt::Write;

// **< PropsOp >************************************************************************************

/// Operaciones disponibles sobre atributos HTML y clases CSS en [`Props`].
///
/// Cada variante lleva los datos necesarios para ejecutarse. El método recomendado para usarlas es
/// recurrir a los constructores asociados como [`set()`](Self::set), [`set_id()`](Self::set_id),
/// [`remove()`](Self::remove), [`add_classes()`](Self::add_classes), etc.
///
/// Las variantes `*Id` operan sobre el atributo `id` del componente. Cuando se usa `"id"` como
/// nombre de atributo en `Set`, el valor se normaliza igual que en `SetId` o `EnsureId`.
///
/// Las variantes `*Classes` operan siempre sobre la lista de clases CSS para el componente. Cuando
/// se usa `"class"` como nombre en `Set` o `Remove` la operación se aplica a la lista de clases
/// completa. Así, `Set("class", ...)` reemplaza la lista de clases completa por las nuevas clases
/// indicadas, y `Remove("class")` vacía la lista de clases.
#[derive(Clone, Debug, PartialEq)]
pub enum PropsOp {
    /// Establece el identificador del elemento normalizando el valor: recorta espacios, convierte a
    /// minúsculas y sustituye los espacios intermedios por `_`. Si el resultado es vacío, elimina
    /// el identificador.
    SetId(CowStr),
    /// Establece el identificador del elemento si aún no hay ninguno definido, de modo que no
    /// sobrescribe un valor asignado con anterioridad. Aplica la misma normalización que
    /// [`SetId`](Self::SetId); si el resultado es vacío, la operación tampoco tiene efecto.
    EnsureId(CowStr),
    /// Añade el atributo o sustituye su valor si ya existe. Usar `"id"` como nombre aplica la misma
    /// normalización que [`SetId`](Self::SetId). Usar `"class"` como nombre reemplaza la lista
    /// completa de clases por las nuevas indicadas; la operación se ignora si el valor contiene
    /// caracteres no ASCII.
    Set(CowStr, CowStr),
    /// Elimina el atributo indicado, incluido `"id"`. Si se usa `"class"` como nombre se vacía la
    /// lista de clases.
    Remove(CowStr),
    /// Añade las clases que no existan al final de la lista. La operación se ignora si el valor
    /// contiene caracteres no ASCII.
    AddClasses(CowStr),
    /// Añade las clases que no existan al principio de la lista. La operación se ignora si el valor
    /// contiene caracteres no ASCII.
    PrependClasses(CowStr),
    /// Elimina las clases indicadas de la lista. La operación se ignora si el valor contiene
    /// caracteres no ASCII.
    RemoveClasses(CowStr),
}

impl PropsOp {
    /// Crea la variante [`SetId`](Self::SetId) con el identificador indicado.
    pub fn set_id(id: impl Into<CowStr>) -> Self {
        Self::SetId(id.into())
    }

    /// Crea la variante [`EnsureId`](Self::EnsureId) con el identificador indicado.
    pub fn ensure_id(id: impl Into<CowStr>) -> Self {
        Self::EnsureId(id.into())
    }

    /// Crea la variante [`Set`](Self::Set) con nombre y valor del atributo.
    pub fn set(name: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        Self::Set(name.into(), value.into())
    }

    /// Crea la variante [`Remove`](Self::Remove) para el atributo indicado.
    pub fn remove(name: impl Into<CowStr>) -> Self {
        Self::Remove(name.into())
    }

    /// Crea la variante [`AddClasses`](Self::AddClasses) con las clases indicadas.
    pub fn add_classes(classes: impl Into<CowStr>) -> Self {
        Self::AddClasses(classes.into())
    }

    /// Crea la variante [`PrependClasses`](Self::PrependClasses) con las clases indicadas.
    pub fn prepend_classes(classes: impl Into<CowStr>) -> Self {
        Self::PrependClasses(classes.into())
    }

    /// Crea la variante [`RemoveClasses`](Self::RemoveClasses) con las clases indicadas.
    pub fn remove_classes(classes: impl Into<CowStr>) -> Self {
        Self::RemoveClasses(classes.into())
    }
}

// **< Props >**************************************************************************************

/// Colección de identificador, atributos HTML y clases CSS para aplicar en componentes.
///
/// Al renderizar en `html!` emite primero `id` (si existe), luego `class` (si hay clases) y después
/// el resto de atributos.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let props = Props::new("hx-get", "/api/items")
///     .with_prop(PropsOp::set("hx-target", "#lista"))
///     .with_prop(PropsOp::set("hx-swap", "outerHTML"));
///
/// let markup = html! {
///     button (props) { "Cargar" }
/// };
///
/// assert_eq!(
///     markup.into_string(),
///     r##"<button hx-get="/api/items" hx-target="#lista" hx-swap="outerHTML">Cargar</button>"##
/// );
/// ```
///
/// # Identificadores
///
/// [`SetId`](PropsOp::SetId) (usando [`PropsOp::set_id`]) normaliza el valor asignado al
/// identificador del componente: recorta espacios, convierte a minúsculas y sustituye los espacios
/// intermedios por `_`.
///
/// ```rust
/// # use pagetop::prelude::*;
/// let props = Props::default().with_id("My Button");
/// let markup = html! { button (props) { "OK" } };
/// assert_eq!(markup.into_string(), r#"<button id="my_button">OK</button>"#);
/// ```
///
/// [`EnsureId`](PropsOp::EnsureId) (usando [`PropsOp::ensure_id`]) sólo asigna si no
/// hay identificador previo:
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Con `id` previo: `EnsureId` no tiene efecto.
/// let props = Props::default()
///     .with_id("explicit")
///     .with_prop(PropsOp::ensure_id("default"));
/// assert_eq!(props.get_id(), Some("explicit".to_string()));
///
/// // Sin `id` previo: `EnsureId` asigna el valor.
/// let props = Props::default().with_prop(PropsOp::ensure_id("default"));
/// assert_eq!(props.get_id(), Some("default".to_string()));
/// ```
///
/// # Clases CSS
///
/// ```rust
/// # use pagetop::prelude::*;
/// let props = Props::default()
///     .with_prop(PropsOp::add_classes("btn btn-primary"))
///     .with_prop(PropsOp::add_classes("active"));
///
/// let markup = html! { button (props) { "OK" } };
/// assert_eq!(markup.into_string(), r#"<button class="btn btn-primary active">OK</button>"#);
/// ```
///
/// # Integración en componentes
///
/// El patrón recomendado es añadir un campo `props: Props` con su método *builder* delegado:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// #[derive(AutoDefault, Clone, Getters)]
/// pub struct MyButton {
///     label: L10n,
///     props: Props,
/// }
///
/// #[async_trait]
/// impl Component for MyButton {
///     fn new() -> Self { Self::default() }
///
///     async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
///         Ok(html! {
///             button (self.props()) {
///                 (self.label().using(cx))
///             }
///         })
///     }
/// }
///
/// impl MyButton {
///     /// Modifica identificador, clases CSS o atributos HTML del elemento raíz.
///     #[builder_fn]
///     pub fn with_prop(mut self, op: PropsOp) -> Self {
///         self.props.alter_prop(op);
///         self
///     }
/// }
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct Props {
    id: Option<String>,
    attrs: Vec<(CowStr, CowStr)>,
    classes: Vec<String>,
}

impl Props {
    /// Crea una colección con un primer atributo ya establecido.
    pub fn new(name: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        Self::default().with_prop(PropsOp::set(name, value))
    }

    /// Crea una colección con las clases CSS iniciales indicadas.
    pub fn classes(classes: impl Into<CowStr>) -> Self {
        Self::default().with_prop(PropsOp::add_classes(classes))
    }

    // **< Props BUILDER >**************************************************************************

    /// Establece el identificador del componente; equivale a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.apply_id(id.into().as_ref());
        self
    }

    /// Modifica el identificador, los atributos o las clases según la operación indicada.
    ///
    /// - [`SetId(value)`](PropsOp::SetId) establece el identificador normalizando el valor.
    /// - [`EnsureId(value)`](PropsOp::EnsureId) establece el identificador (con la misma
    ///   normalización) sólo si no hay ninguno definido.
    /// - [`Set(name, value)`](PropsOp::Set) añade el atributo o reemplaza su valor.
    ///   `Set("id", ...)` aplica la misma normalización que `SetId`.
    ///   `Set("class", ...)` reemplaza la lista de clases completa.
    /// - [`Remove(name)`](PropsOp::Remove) elimina el atributo. `Remove("id")` elimina el
    ///   identificador. `Remove("class")` vacía la lista de clases.
    /// - [`AddClasses(clases)`](PropsOp::AddClasses) añade clases al final (sin duplicados).
    /// - [`PrependClasses(clases)`](PropsOp::PrependClasses) añade clases al principio (sin
    ///   duplicados).
    /// - [`RemoveClasses(clases)`](PropsOp::RemoveClasses) elimina las clases indicadas.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        match op {
            PropsOp::SetId(value) => {
                self.apply_id(value.as_ref());
            }
            PropsOp::EnsureId(value) => {
                if self.id.is_none() {
                    self.apply_id(value.as_ref());
                }
            }
            PropsOp::Set(name, value) => {
                if name.as_ref() == "id" {
                    self.apply_id(value.as_ref());
                } else if name.as_ref() == "class" {
                    if let Some(normalized) =
                        util::normalize_ascii_or_empty(value.as_ref(), "Props::with_prop")
                    {
                        self.classes.clear();
                        self.insert_classes(normalized.as_ref().split_ascii_whitespace(), 0);
                    }
                } else if let Some(pos) = self.attrs.iter().position(|(k, _)| k == &name) {
                    self.attrs[pos].1 = value;
                } else {
                    self.attrs.push((name, value));
                }
            }
            PropsOp::Remove(name) => {
                if name.as_ref() == "id" {
                    self.id = None;
                } else if name.as_ref() == "class" {
                    self.classes.clear();
                } else {
                    self.attrs.retain(|(k, _)| k != &name);
                }
            }
            PropsOp::AddClasses(classes) => {
                let Some(normalized) =
                    util::normalize_ascii_or_empty(classes.as_ref(), "Props::with_prop")
                else {
                    return self;
                };
                let pos = self.classes.len();
                self.insert_classes(normalized.as_ref().split_ascii_whitespace(), pos);
            }
            PropsOp::PrependClasses(classes) => {
                let Some(normalized) =
                    util::normalize_ascii_or_empty(classes.as_ref(), "Props::with_prop")
                else {
                    return self;
                };
                self.insert_classes(normalized.as_ref().split_ascii_whitespace(), 0);
            }
            PropsOp::RemoveClasses(classes) => {
                let Some(normalized) =
                    util::normalize_ascii_or_empty(classes.as_ref(), "Props::with_prop")
                else {
                    return self;
                };
                self.classes.retain(|c| {
                    !normalized
                        .as_ref()
                        .split_ascii_whitespace()
                        .any(|r| r == c.as_str())
                });
            }
        }
        self
    }

    // **< Props GETTERS >**************************************************************************

    /// Devuelve el identificador normalizado del elemento, si existe.
    #[inline]
    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    /// Devuelve el valor del atributo indicado, si existe.
    ///
    /// Los nombres `"id"` y `"class"` son equivalentes a llamar a [`get_id()`](Self::get_id) y
    /// [`get_classes()`](Self::get_classes) respectivamente.
    pub fn get_prop(&self, name: impl AsRef<str>) -> Option<String> {
        match name.as_ref() {
            "id" => self.id.clone(),
            "class" => self.get_classes(),
            name => self
                .attrs
                .iter()
                .find(|(k, _)| k.as_ref() == name)
                .map(|(_, v)| v.to_string()),
        }
    }

    /// Devuelve la lista de clases como cadena de texto, si hay clases definidas.
    pub fn get_classes(&self) -> Option<String> {
        if self.classes.is_empty() {
            None
        } else {
            Some(self.classes.join(" "))
        }
    }

    /// Devuelve `true` si no hay ningún identificador definido.
    #[inline]
    pub fn is_id_empty(&self) -> bool {
        self.id.is_none()
    }

    /// Devuelve `true` si no hay ningún atributo extra definido, sin tener en cuenta el
    /// identificador ni las clases.
    #[inline]
    pub fn is_attrs_empty(&self) -> bool {
        self.attrs.is_empty()
    }

    /// Devuelve `true` si no hay ninguna clase definida.
    #[inline]
    pub fn is_classes_empty(&self) -> bool {
        self.classes.is_empty()
    }

    /// Devuelve `true` si no hay ningún identificador, atributo ni clase definidos.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.attrs.is_empty() && self.classes.is_empty()
    }

    /// Devuelve `true` si la clase o **todas** las clases indicadas están presentes.
    pub fn has_class(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii(classes.as_ref()) else {
            return false;
        };
        normalized
            .as_ref()
            .split_ascii_whitespace()
            .all(|class| self.classes.iter().any(|c| c == class))
    }

    /// Devuelve `true` si la clase o **alguna** de las clases indicadas está presente.
    pub fn has_any_class(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii(classes.as_ref()) else {
            return false;
        };
        normalized
            .as_ref()
            .split_ascii_whitespace()
            .any(|class| self.classes.iter().any(|c| c == class))
    }

    // **< Props PRIVATE >**************************************************************************

    fn apply_id(&mut self, id: &str) {
        let id = id.trim();
        self.id = if id.is_empty() {
            None
        } else {
            Some(id.to_ascii_lowercase().replace(' ', "_"))
        };
    }

    fn insert_classes<'a, I>(&mut self, classes: I, mut pos: usize)
    where
        I: IntoIterator<Item = &'a str>,
    {
        for class in classes {
            if !self.classes.iter().any(|c| c == class) {
                let class = class.to_string();
                if pos >= self.classes.len() {
                    self.classes.push(class);
                } else {
                    self.classes.insert(pos, class);
                }
                pos += 1;
            }
        }
    }
}

#[doc(hidden)]
impl Render for Props {
    fn render_to(&self, w: &mut String) {
        if let Some(id) = self.id.as_deref() {
            w.push_str(" id=\"");
            let _ = write!(Escaper::new(w), "{}", id);
            w.push('"');
        }
        if let Some((first, rest)) = self.classes.split_first() {
            w.push_str(" class=\"");
            let _ = write!(Escaper::new(w), "{}", first);
            for class in rest {
                w.push(' ');
                let _ = write!(Escaper::new(w), "{}", class);
            }
            w.push('"');
        }
        for (name, value) in &self.attrs {
            w.push(' ');
            let _ = write!(Escaper::new(w), "{}", name);
            w.push_str("=\"");
            let _ = write!(Escaper::new(w), "{}", value);
            w.push('"');
        }
    }
}
