use crate::core::TypeInfo;
use crate::html::maud::{Escaper, RenderAttrs};
use crate::{AutoDefault, CowStr, builder_fn, trace, util};

use thiserror::Error;

use std::any::Any;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::panic::Location;
use std::sync::Arc;

// **< PropsExtra >*********************************************************************************

/// Encapsula un valor tipado extra para almacenar en [`Props`].
///
/// Internamente usa [`Arc`] para que [`Props`] pueda implementar [`Clone`] sin requerir que los
/// valores almacenados sean clonables. El nombre del tipo almacenado permite generar mensajes de
/// error precisos.
pub struct PropsExtra {
    value: Arc<dyn Any + Send + Sync>,
    type_name: &'static str,
}

impl Clone for PropsExtra {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            type_name: self.type_name,
        }
    }
}

impl fmt::Debug for PropsExtra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.type_name)
    }
}

// **< PropsError >*********************************************************************************

/// Errores de acceso a valores extra de [`Props`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum PropsError {
    /// La clave no existe. Incluye la clave (`key`).
    #[error("extra \"{key}\" not found")]
    ExtraNotFound { key: &'static str },
    /// La clave existe pero el tipo solicitado no coincide con el almacenado. Incluye la clave
    /// (`key`), tipo esperado (`expected`) y tipo realmente encontrado (`found`) para facilitar el
    /// diagnóstico.
    #[error("type mismatch for extra \"{key}\": expected \"{expected}\", found \"{found}\"")]
    ExtraTypeMismatch {
        key: &'static str,
        expected: &'static str,
        found: &'static str,
    },
}

// **< PropsOp >************************************************************************************

/// Operaciones sobre el identificador, clases CSS, atributos HTML y valores extra en [`Props`].
///
/// Cada variante lleva los datos necesarios para ejecutarse. El método recomendado para usarlas es
/// recurrir a los constructores asociados como [`set_id()`](Self::set_id),
/// [`add_classes()`](Self::add_classes), [`set()`](Self::set), etc.
///
/// Las variantes `*Id` operan sobre el atributo `id` del componente. Cuando se usa `"id"` como
/// nombre de atributo en `Set`, el valor se normaliza igual que [`SetId`](Self::SetId).
///
/// Las variantes `*Classes` gestionan la lista de clases CSS. Además, `Set("class", ...)`
/// reemplaza la lista completa de clases y `Remove("class")` la vacía.
///
/// Las variantes `*Style` gestionan las declaraciones de estilos para el atributo `style`, con una
/// propiedad cada vez. Además, `Set("style", ...)` reemplaza la lista completa de estilos y
/// `Remove("style")` la vacía.
///
/// Las variantes [`Set`](Self::Set) y [`Remove`](Self::Remove) son operaciones de propósito
/// general. `Set` añade o reemplaza cualquier atributo HTML por nombre y valor, y `Remove` lo
/// elimina. Los atributos `id`, `class` y `style` tienen semántica especial documentada en cada
/// variante.
///
/// Las variantes `*Extra` permiten añadir valores tipados usando una clave. Están pensadas para
/// ampliar el comportamiento de componentes ya existentes. Como no es posible añadir campos a la
/// estructura de un componente ya definido, temas y extensiones pueden definir un trait con nuevos
/// métodos que leen y escriben valores extra en [`Props`]. Esos valores se interpretan como si
/// fueran valores internos del componente para tomar decisiones durante el renderizado.
#[derive(Clone, Debug)]
pub enum PropsOp {
    /// Establece el identificador del componente normalizando el valor: recorta espacios, convierte
    /// a minúsculas y sustituye los espacios intermedios por `_`. Si el resultado es vacío, elimina
    /// el identificador.
    SetId(CowStr),
    /// Establece el identificador del componente **sólo si aún no hay ninguno definido**. Aplica la
    /// misma normalización que [`SetId`](Self::SetId); si el resultado es vacío, la operación no
    /// tiene efecto.
    EnsureId(CowStr),
    /// Añade la clase o clases que no existan al final de la lista. La operación se ignora si el
    /// valor contiene caracteres no ASCII.
    AddClasses(CowStr),
    /// Añade la clase o clases que no existan al principio de la lista. La operación se ignora si
    /// el valor contiene caracteres no ASCII.
    PrependClasses(CowStr),
    /// Sustituye **una o más** clases del primer valor por las clases indicadas en el segundo
    /// valor, insertando las nuevas en la posición de la primera clase a sustituir encontrada, con
    /// independencia del orden en que aparecen en el primer valor. Las que no existan se ignoran.
    /// Si **ninguna** de las clases a sustituir existe, la operación no tiene efecto y no se
    /// inserta nada. Se ignora si alguno de los dos valores contiene caracteres no ASCII.
    ReplaceClasses(CowStr, CowStr),
    /// A diferencia de [`ReplaceClasses`](Self::ReplaceClasses), exige que **todas** las clases del
    /// primer valor estén presentes, independientemente de su orden; si falta una sola, la
    /// operación no tiene efecto: ninguna clase se elimina ni se inserta. Si todas están presentes,
    /// las sustituye por las clases indicadas en el segundo valor, insertando las nuevas en la
    /// posición de la primera clase a sustituir encontrada. Se ignora si alguno de los dos valores
    /// contiene caracteres no ASCII.
    ReplaceAllClasses(CowStr, CowStr),
    /// Elimina la clase o clases indicadas de la lista. La operación se ignora si el valor contiene
    /// caracteres no ASCII.
    RemoveClasses(CowStr),
    /// Añade una declaración de estilo (propiedad, valor) o sustituye su valor si la propiedad ya
    /// existe, conservando su posición; si no, se añade al final. A diferencia de las clases, el
    /// valor admite caracteres no ASCII (p. ej. `content`, `font-family`) y distingue mayúsculas y
    /// minúsculas. El nombre de la propiedad se normaliza a minúsculas. Si la propiedad o el valor
    /// quedan vacíos tras recortar espacios, la operación se ignora.
    AddStyle(CowStr, CowStr),
    /// Elimina la propiedad de estilo indicada, si existe.
    RemoveStyle(CowStr),
    /// Añade un atributo o sustituye su valor si ya existe.
    ///
    /// Usar `"id"` como nombre de atributo aplica al valor la misma normalización que
    /// [`SetId`](Self::SetId).
    ///
    /// Usar `"class"` como nombre de atributo reemplaza la lista completa de clases por las nuevas
    /// indicadas; la operación se ignora si el valor contiene caracteres no ASCII.
    ///
    /// Usar `"style"` como nombre de atributo reemplaza la lista completa de estilos por los nuevos
    /// indicados, interpretando el valor como declaraciones `"propiedad: valor"` separadas por `;`
    /// (igual que el propio atributo `style` HTML). El separador `;` respeta paréntesis y comillas,
    /// tal que valores como una *data URI* (`background: url(data:image/png;base64,...)`) o una
    /// cadena con `;` (`content: "a;b"`) se interpretan correctamente. En cualquier caso, se
    /// recomienda usar [`PropsOp::add_style()`](Self::add_style) para declarar estilos.
    Set(CowStr, CowStr),
    /// Elimina el atributo indicado. Usar `"id"` elimina el identificador; usar `"class"` vacía la
    /// lista de clases; y usar `"style"` vacía la lista de estilos.
    Remove(CowStr),
    /// Almacena un valor extra tipado asociado a la clave indicada. Si ya existe uno con esa clave,
    /// lo reemplaza.
    SetExtra(&'static str, PropsExtra),
    /// Elimina el valor extra asociado a la clave indicada, si existe.
    RemoveExtra(&'static str),
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

    /// Crea la variante [`AddClasses`](Self::AddClasses) con la clase o clases indicadas.
    pub fn add_classes(classes: impl Into<CowStr>) -> Self {
        Self::AddClasses(classes.into())
    }

    /// Crea la variante [`PrependClasses`](Self::PrependClasses) con la clase o clases indicadas.
    pub fn prepend_classes(classes: impl Into<CowStr>) -> Self {
        Self::PrependClasses(classes.into())
    }

    /// Crea la variante [`ReplaceClasses`](Self::ReplaceClasses) con las clases a sustituir (`old`)
    /// y las nuevas clases (`new`).
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::classes("button primary")
    ///     .with_prop(PropsOp::replace_classes("button", "btn"));
    /// assert_eq!(props.get_classes(), Some("btn primary".to_string()));
    ///
    /// // Basta con que exista alguna clase de `old` para aplicar el reemplazo.
    /// let props = Props::classes("btn primary")
    ///     .with_prop(PropsOp::replace_classes("primary secondary", "danger"));
    /// assert_eq!(props.get_classes(), Some("btn danger".to_string()));
    /// ```
    pub fn replace_classes(old: impl Into<CowStr>, new: impl Into<CowStr>) -> Self {
        Self::ReplaceClasses(old.into(), new.into())
    }

    /// Crea la variante [`ReplaceAllClasses`](Self::ReplaceAllClasses) con las clases a sustituir
    /// (`old`) y las nuevas clases (`new`).
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::classes("btn primary")
    ///     .with_prop(PropsOp::replace_all_classes("btn primary", "btn danger"));
    /// assert_eq!(props.get_classes(), Some("btn danger".to_string()));
    ///
    /// // Si falta una sola clase de `old`, no hay reemplazo.
    /// let props = Props::classes("btn primary")
    ///     .with_prop(PropsOp::replace_all_classes("primary secondary", "danger"));
    /// assert_eq!(props.get_classes(), Some("btn primary".to_string()));
    /// ```
    pub fn replace_all_classes(old: impl Into<CowStr>, new: impl Into<CowStr>) -> Self {
        Self::ReplaceAllClasses(old.into(), new.into())
    }

    /// Crea la variante [`RemoveClasses`](Self::RemoveClasses) con la clase o clases indicadas.
    pub fn remove_classes(classes: impl Into<CowStr>) -> Self {
        Self::RemoveClasses(classes.into())
    }

    /// Crea la variante [`AddStyle`](Self::AddStyle) con la propiedad y el valor de estilo
    /// indicados.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::default()
    ///     .with_prop(PropsOp::add_style("color", "red"))
    ///     .with_prop(PropsOp::add_style("font-weight", "bold"))
    ///     .with_prop(PropsOp::add_style("color", "blue"));
    /// assert_eq!(props.get_styles(), Some("color: blue; font-weight: bold".to_string()));
    /// ```
    pub fn add_style(property: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        Self::AddStyle(property.into(), value.into())
    }

    /// Crea la variante [`RemoveStyle`](Self::RemoveStyle) para la propiedad de estilo indicada.
    pub fn remove_style(property: impl Into<CowStr>) -> Self {
        Self::RemoveStyle(property.into())
    }

    /// Crea la variante [`Set`](Self::Set) con nombre y valor del atributo.
    pub fn set(name: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        Self::Set(name.into(), value.into())
    }

    /// Crea la variante [`Remove`](Self::Remove) para el atributo indicado.
    pub fn remove(name: impl Into<CowStr>) -> Self {
        Self::Remove(name.into())
    }

    /// Crea la variante [`SetExtra`](Self::SetExtra) con la clave y el valor indicados.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// const EXT_SIZE: &str = "myext.size";
    /// let props = Props::default().with_prop(PropsOp::set_extra(EXT_SIZE, 42_u32));
    /// assert_eq!(props.extra_or(EXT_SIZE, 0_u32), 42);
    /// ```
    pub fn set_extra<T: Any + Send + Sync + 'static>(key: &'static str, value: T) -> Self {
        Self::SetExtra(
            key,
            PropsExtra {
                value: Arc::new(value),
                type_name: TypeInfo::FullName.of::<T>(),
            },
        )
    }

    /// Crea la variante [`RemoveExtra`](Self::RemoveExtra) para la clave indicada.
    pub fn remove_extra(key: &'static str) -> Self {
        Self::RemoveExtra(key)
    }
}

// **< Props >**************************************************************************************

/// Recoge el identificador, clases CSS, atributos HTML y valores extra de un componente.
///
/// Guarda estos valores con operaciones [`PropsOp`]. Cuando se renderiza usando
/// [`html!`](crate::html::html) se emite primero el identificador `id` (si existe), luego `class`
/// (si hay clases), después `style` (si hay declaraciones de estilo) y por último el resto de
/// atributos; normalmente se asignan al elemento raíz del componente.
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
/// [`SetId`](PropsOp::SetId) (usando [`PropsOp::set_id()`]) normaliza el valor asignado al
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
/// [`EnsureId`](PropsOp::EnsureId) (usando [`PropsOp::ensure_id()`]) sólo asigna si no hay
/// identificador previo:
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
///     .with_prop(PropsOp::add_classes("active"))
///     .with_prop(PropsOp::replace_classes("btn-primary", "btn-secondary"));
///
/// let markup = html! { button (props) { "OK" } };
/// assert_eq!(markup.into_string(), r#"<button class="btn btn-secondary active">OK</button>"#);
/// ```
///
/// # Estilos CSS
///
/// Cada declaración se añade indicando una propiedad y su valor. Si la propiedad ya existe,
/// [`AddStyle`](PropsOp::AddStyle) sustituye su valor conservando la posición, sin duplicarla.
///
/// ```rust
/// # use pagetop::prelude::*;
/// let props = Props::default()
///     .with_prop(PropsOp::add_style("color", "red"))
///     .with_prop(PropsOp::add_style("font-weight", "bold"))
///     .with_prop(PropsOp::add_style("color", "blue"))
///     .with_prop(PropsOp::remove_style("font-weight"));
///
/// let markup = html! { button (props) { "OK" } };
/// assert_eq!(markup.into_string(), r#"<button style="color: blue">OK</button>"#);
/// ```
///
/// # Atributos duplicados junto a `Props`
///
/// Cuando el componente combina `(self.props())` con un atributo literal del mismo nombre en el
/// mismo elemento (una clase, un `#id`, o `nombre=valor`), la macro [`html!`](crate::html::html)
/// evita automáticamente la duplicación. Recopila en tiempo de compilación los nombres de los
/// atributos del elemento y al renderizar se omiten los duplicados en tiempo de ejecución. No
/// depende del orden en que se escriban ni requiere ninguna acción del desarrollador.
///
/// ```rust
/// # use pagetop::prelude::*;
/// let props = Props::default().with_prop(PropsOp::set("title", "de Props"));
///
/// let markup = html! { span title="literal" (props) { "OK" } };
///
/// // El atributo literal prevalece; `Props` omite su propio "title" en vez de duplicarlo.
/// assert_eq!(markup.into_string(), r#"<span title="literal">OK</span>"#);
/// ```
///
/// # Valores extra
///
/// Las variantes [`SetExtra`](PropsOp::SetExtra) y [`RemoveExtra`](PropsOp::RemoveExtra), usando
/// [`PropsOp::set_extra()`] y [`PropsOp::remove_extra()`] respectivamente, permiten adjuntar
/// valores tipados a un `Props`. Son útiles para que temas y extensiones amplíen el comportamiento
/// de componentes ya existentes mediante traits con nuevos métodos que lean y escriban esos
/// valores.
///
/// ```rust
/// # use pagetop::prelude::*;
/// const EXT_ENABLED: &str = "myext.enabled";
/// const EXT_LABEL: &str = "myext.label";
///
/// let props = Props::default()
///     .with_prop(PropsOp::set_extra(EXT_ENABLED, true))
///     .with_prop(PropsOp::set_extra(EXT_LABEL, "flotante".to_string()));
///
/// assert!(props.extra_or(EXT_ENABLED, false));
/// assert_eq!(props.extra_or(EXT_LABEL, String::new()), "flotante");
///
/// // Tipo incorrecto devuelve el valor por defecto indicado:
/// assert_eq!(props.extra_or(EXT_ENABLED, 0_u8), 0);
/// ```
///
/// Los valores extra no se emiten en el HTML al renderizar; son exclusivamente para uso interno de
/// temas y extensiones.
///
/// # Integración en componentes
///
/// El patrón recomendado es añadir un campo `props: Props` con su método *builder* delegado:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// #[derive(AutoDefault, Clone, Getters)]
/// pub struct MyButton {
///     label: Lc,
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
///     /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
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
    classes: Vec<String>,
    styles: Vec<(CowStr, CowStr)>,
    attrs: Vec<(CowStr, CowStr)>,
    extras: HashMap<&'static str, PropsExtra>,
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

    /// Modifica el identificador, las clases, los atributos o los valores extra según la operación
    /// indicada. El método recomendado para construir cada operación es usar los constructores de
    /// [`PropsOp`].
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
            PropsOp::ReplaceClasses(old, new) => {
                let Some(old) = util::normalize_ascii_or_empty(old.as_ref(), "Props::with_prop")
                else {
                    return self;
                };
                let Some(new) = util::normalize_ascii_or_empty(new.as_ref(), "Props::with_prop")
                else {
                    return self;
                };
                let mut pos = self.classes.len();
                let mut replaced = false;
                for class in old.as_ref().split_ascii_whitespace() {
                    if let Some(replace_pos) = self.classes.iter().position(|c| c == class) {
                        self.classes.remove(replace_pos);
                        pos = pos.min(replace_pos);
                        replaced = true;
                    }
                }
                if replaced {
                    self.insert_classes(new.as_ref().split_ascii_whitespace(), pos);
                }
            }
            PropsOp::ReplaceAllClasses(old, new) => {
                let Some(old) = util::normalize_ascii_or_empty(old.as_ref(), "Props::with_prop")
                else {
                    return self;
                };
                let Some(new) = util::normalize_ascii_or_empty(new.as_ref(), "Props::with_prop")
                else {
                    return self;
                };
                if !self.has_all_classes(old.as_ref()) {
                    return self;
                }
                let mut pos = self.classes.len();
                for class in old.as_ref().split_ascii_whitespace() {
                    if let Some(replace_pos) = self.classes.iter().position(|c| c == class) {
                        self.classes.remove(replace_pos);
                        pos = pos.min(replace_pos);
                    }
                }
                self.insert_classes(new.as_ref().split_ascii_whitespace(), pos);
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
            PropsOp::AddStyle(property, value) => {
                self.set_style(property.as_ref(), value.as_ref());
            }
            PropsOp::RemoveStyle(property) => {
                self.remove_style(property.as_ref());
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
                } else if name.as_ref() == "style" {
                    self.styles.clear();
                    self.parse_styles(value.as_ref());
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
                } else if name.as_ref() == "style" {
                    self.styles.clear();
                } else {
                    self.attrs.retain(|(k, _)| k != &name);
                }
            }
            PropsOp::SetExtra(key, extra) => {
                self.extras.insert(key, extra);
            }
            PropsOp::RemoveExtra(key) => {
                self.extras.remove(key);
            }
        }
        self
    }

    // **< Props GETTERS >**************************************************************************

    /// Devuelve el identificador normalizado del componente, si existe.
    #[inline]
    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    /// Devuelve la lista de clases como cadena de texto, si hay clases definidas.
    pub fn get_classes(&self) -> Option<String> {
        if self.classes.is_empty() {
            None
        } else {
            Some(self.classes.join(" "))
        }
    }

    /// Devuelve las declaraciones de estilo como cadena de texto (separadas por `"; "`), si hay
    /// estilos definidos.
    pub fn get_styles(&self) -> Option<String> {
        if self.styles.is_empty() {
            None
        } else {
            Some(
                self.styles
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        }
    }

    /// Devuelve el valor de la propiedad de estilo indicada, si existe.
    pub fn get_style(&self, property: impl AsRef<str>) -> Option<String> {
        let property = property.as_ref().trim().to_ascii_lowercase();
        self.styles
            .iter()
            .find(|(k, _)| k.as_ref() == property)
            .map(|(_, v)| v.to_string())
    }

    /// Devuelve el valor del atributo indicado, si existe.
    ///
    /// Los nombres `"id"`, `"class"` y `"style"` son equivalentes a llamar a
    /// [`get_id()`](Self::get_id), [`get_classes()`](Self::get_classes) y
    /// [`get_styles()`](Self::get_styles) respectivamente.
    pub fn get_prop(&self, name: impl AsRef<str>) -> Option<String> {
        match name.as_ref() {
            "id" => self.id.clone(),
            "class" => self.get_classes(),
            "style" => self.get_styles(),
            name => self
                .attrs
                .iter()
                .find(|(k, _)| k.as_ref() == name)
                .map(|(_, v)| v.to_string()),
        }
    }

    /// Devuelve `true` si no hay ningún identificador definido.
    #[inline]
    pub fn is_id_empty(&self) -> bool {
        self.id.is_none()
    }

    /// Devuelve `true` si no hay ninguna clase definida.
    #[inline]
    pub fn is_classes_empty(&self) -> bool {
        self.classes.is_empty()
    }

    /// Devuelve `true` si no hay ningún estilo definido.
    #[inline]
    pub fn is_styles_empty(&self) -> bool {
        self.styles.is_empty()
    }

    /// Devuelve `true` si no hay ningún atributo adicional definido, sin tener en cuenta el
    /// identificador, las clases ni los estilos.
    #[inline]
    pub fn is_attrs_empty(&self) -> bool {
        self.attrs.is_empty()
    }

    /// Devuelve `true` si no hay ningún identificador, clases, estilos o atributos adicionales
    /// definidos, sin tener en cuenta los valores extra.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.classes.is_empty()
            && self.styles.is_empty()
            && self.attrs.is_empty()
    }

    /// Devuelve `true` si la clase o **alguna** de las clases indicadas está presente.
    pub fn has_classes(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii(classes.as_ref()) else {
            return false;
        };
        normalized
            .as_ref()
            .split_ascii_whitespace()
            .any(|class| self.classes.iter().any(|c| c == class))
    }

    /// Devuelve `true` si la clase o **todas** las clases indicadas están presentes.
    pub fn has_all_classes(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii(classes.as_ref()) else {
            return false;
        };
        normalized
            .as_ref()
            .split_ascii_whitespace()
            .all(|class| self.classes.iter().any(|c| c == class))
    }

    /// Recupera una referencia tipada al valor extra asociado a la clave `key`.
    ///
    /// Devuelve un [`Result`] que indica si la clave existe y si el tipo coincide:
    ///
    /// - `Ok(&T)` si la clave existe y el tipo coincide. El tipo `T` debe ser el mismo que se usó
    ///   al almacenar el valor con [`PropsOp::set_extra()`].
    /// - `Err(PropsError::ExtraNotFound)` si la clave no existe.
    /// - `Err(PropsError::ExtraTypeMismatch)` si el tipo no coincide.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// const EXT_COUNT: &str = "myext.count";
    /// const EXT_OTHER: &str = "myext.other";
    ///
    /// let props = Props::default().with_prop(PropsOp::set_extra(EXT_COUNT, 7_i32));
    ///
    /// assert_eq!(*props.extra::<i32>(EXT_COUNT).unwrap(), 7);
    /// assert_eq!(
    ///     props.extra::<i32>(EXT_OTHER),
    ///     Err(PropsError::ExtraNotFound { key: EXT_OTHER })
    /// );
    /// assert!(matches!(
    ///     props.extra::<u32>(EXT_COUNT),
    ///     Err(PropsError::ExtraTypeMismatch { .. })
    /// ));
    /// ```
    pub fn extra<T: 'static>(&self, key: &'static str) -> Result<&T, PropsError> {
        let ev = self
            .extras
            .get(key)
            .ok_or(PropsError::ExtraNotFound { key })?;
        ev.value
            .downcast_ref::<T>()
            .ok_or_else(|| PropsError::ExtraTypeMismatch {
                key,
                expected: TypeInfo::FullName.of::<T>(),
                found: ev.type_name,
            })
    }

    /// Devuelve el valor extra clonado o el **valor `default`** si no existe o el tipo no coincide.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// const EXT_FLAG: &str = "myext.flag";
    /// const EXT_OTHER: &str = "myext.other";
    ///
    /// let props = Props::default().with_prop(PropsOp::set_extra(EXT_FLAG, true));
    ///
    /// assert!(props.extra_or(EXT_FLAG, false));
    /// assert!(!props.extra_or(EXT_OTHER, false));
    /// ```
    pub fn extra_or<T: Clone + 'static>(&self, key: &'static str, default: T) -> T {
        self.extra::<T>(key).ok().cloned().unwrap_or(default)
    }

    /// Devuelve el valor extra clonado o el **valor por defecto del tipo** si no existe o el tipo
    /// no coincide.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// const EXT_FLAG: &str = "myext.flag";
    /// const EXT_COUNT: &str = "myext.count";
    ///
    /// let props = Props::default();
    ///
    /// assert_eq!(props.extra_or_default::<bool>(EXT_FLAG), false);
    /// assert_eq!(props.extra_or_default::<i32>(EXT_COUNT), 0);
    /// ```
    pub fn extra_or_default<T: Clone + Default + 'static>(&self, key: &'static str) -> T {
        self.extra::<T>(key).ok().cloned().unwrap_or_default()
    }

    /// Devuelve el valor extra clonado o el **valor evaluado por la función `f`** si no existe o el
    /// tipo no coincide.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// const EXT_LABEL: &str = "myext.label";
    ///
    /// let props = Props::default();
    ///
    /// let result = props.extra_or_else(EXT_LABEL, || "default".to_string());
    /// assert_eq!(result, "default");
    /// ```
    pub fn extra_or_else<T: Clone + 'static, F: FnOnce() -> T>(
        &self,
        key: &'static str,
        f: F,
    ) -> T {
        self.extra::<T>(key).ok().cloned().unwrap_or_else(f)
    }

    // **< Props PRIVATE >**************************************************************************

    fn apply_id(&mut self, id: &str) {
        self.id = util::normalize_token(id);
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

    // Añade o sustituye una declaración "propiedad: valor". Si la propiedad ya existe, sustituye
    // su valor conservando la posición; si no, la añade al final. Ignora la declaración si la
    // propiedad o el valor quedan vacíos tras recortar espacios. No aplica
    // normalize_ascii_or_empty: ver la documentación de `PropsOp::AddStyle` sobre por qué los
    // valores de estilo no se restringen a ASCII.
    fn set_style(&mut self, property: &str, value: &str) {
        let property = property.trim().to_ascii_lowercase();
        let value = value.trim();
        if property.is_empty() || value.is_empty() {
            return;
        }
        if let Some(pos) = self.styles.iter().position(|(k, _)| k.as_ref() == property) {
            self.styles[pos].1 = value.to_string().into();
        } else {
            self.styles
                .push((property.into(), value.to_string().into()));
        }
    }

    // Interpreta una cadena "propiedad: valor" separadas por ";" (igual que el atributo HTML
    // `style`) y aplica cada declaración con `set_style`. Ignora las declaraciones sin ":".
    fn parse_styles(&mut self, styles: &str) {
        for style in Self::split_style_declarations(styles) {
            let style = style.trim();
            if style.is_empty() {
                continue;
            }
            let Some((property, value)) = style.split_once(':') else {
                trace::debug!(
                    target = "Props::with_prop",
                    declaration = %style,
                    "Ignoring malformed style declaration (missing \":\")"
                );
                continue;
            };
            self.set_style(property, value);
        }
    }

    // Divide una cadena de declaraciones de estilo por ";", igual que `str::split(';')`, pero sin
    // cortar dentro de paréntesis (`url(...)`) ni de cadenas entre comillas simples o dobles
    // (`content: "a;b"`). No es un análisis CSS completo: no reconoce comentarios `/* ... */` ni
    // comillas escapadas, y unos paréntesis o comillas sin cerrar arrastran el resto de la cadena
    // a la última declaración.
    fn split_style_declarations(styles: &str) -> Vec<&str> {
        let mut depth = 0i32;
        let mut quote = None;
        let mut start = 0;
        let mut parts = Vec::new();
        for (i, c) in styles.char_indices() {
            if quote.is_none() && (c == '\'' || c == '"') {
                quote = Some(c);
            } else if quote == Some(c) {
                quote = None;
            } else if quote.is_none() && c == '(' {
                depth += 1;
            } else if quote.is_none() && c == ')' {
                depth = (depth - 1).max(0);
            } else if quote.is_none() && depth == 0 && c == ';' {
                parts.push(&styles[start..i]);
                start = i + 1;
            }
        }
        parts.push(&styles[start..]);
        parts
    }

    // Elimina la propiedad de estilo indicada, si existe.
    fn remove_style(&mut self, property: &str) {
        let property = property.trim().to_ascii_lowercase();
        self.styles.retain(|(k, _)| k.as_ref() != property);
    }
}

#[doc(hidden)]
impl RenderAttrs for Props {
    // Omite cualquier atributo que esté en `exclude` (recopilados por `html!` a partir de los
    // atributos literales del elemento). Registra un `trace::debug!` por cada atributo duplicado,
    // con la posición exacta del `html!` que lo produjo (propagado gracias a `#[track_caller]`)
    // para facilitar la localización del problema.
    #[track_caller]
    fn render_attrs_to(&self, w: &mut String, exclude: &[&str]) {
        if let Some(id) = self.id.as_deref() {
            if exclude.contains(&"id") {
                trace::debug!(
                    caller = %Location::caller(),
                    attribute = "id",
                    discarded = %id,
                    "Ignoring Props attribute already set as a literal on the same element"
                );
            } else {
                w.push_str(" id=\"");
                let _ = write!(Escaper::new(w), "{}", id);
                w.push('"');
            }
        }
        if let Some((first, rest)) = self.classes.split_first() {
            if exclude.contains(&"class") {
                trace::debug!(
                    caller = %Location::caller(),
                    attribute = "class",
                    discarded = %self.classes.join(" "),
                    id = %self.id.as_deref().unwrap_or("<none>"),
                    "Ignoring Props attribute already set as a literal on the same element"
                );
            } else {
                w.push_str(" class=\"");
                let _ = write!(Escaper::new(w), "{}", first);
                for class in rest {
                    w.push(' ');
                    let _ = write!(Escaper::new(w), "{}", class);
                }
                w.push('"');
            }
        }
        if let Some((first, rest)) = self.styles.split_first() {
            if exclude.contains(&"style") {
                let discarded = self
                    .styles
                    .iter()
                    .map(|(property, value)| format!("{property}: {value}"))
                    .collect::<Vec<_>>()
                    .join("; ");
                trace::debug!(
                    caller = %Location::caller(),
                    attribute = "style",
                    discarded = %discarded,
                    id = %self.id.as_deref().unwrap_or("<none>"),
                    "Ignoring Props attribute already set as a literal on the same element"
                );
            } else {
                w.push_str(" style=\"");
                let _ = write!(Escaper::new(w), "{}: {}", first.0, first.1);
                for (property, value) in rest {
                    w.push_str("; ");
                    let _ = write!(Escaper::new(w), "{}: {}", property, value);
                }
                w.push('"');
            }
        }
        for (name, value) in &self.attrs {
            if exclude.contains(&name.as_ref()) {
                trace::debug!(
                    caller = %Location::caller(),
                    attribute = %name,
                    discarded = %value,
                    id = %self.id.as_deref().unwrap_or("<none>"),
                    "Ignoring Props attribute already set as a literal on the same element"
                );
                continue;
            }
            w.push(' ');
            let _ = write!(Escaper::new(w), "{}", name);
            w.push_str("=\"");
            let _ = write!(Escaper::new(w), "{}", value);
            w.push('"');
        }
    }
}
