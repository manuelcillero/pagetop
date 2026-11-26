use crate::core::TypeInfo;
use crate::core::component::Context;
use crate::html::flex::FlexItem;
use crate::html::grid::GridItem;
use crate::html::maud::{Escaper, RenderAttrs};
use crate::html::props::{PropValue, PropsError, PropsExtra, PropsOp};
use crate::html::spacing::{Margin, Padding};
use crate::locale::{LangId, LanguageIdentifier, Lc};
use crate::{AutoDefault, CowStr, builder_impl, trace, util};

use std::collections::HashMap;
use std::fmt::Write;
use std::panic::Location;
use std::sync::Arc;

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
/// Se omite la construcción explícita de `Context` (`let mut cx = Context::default();`) para no
/// distraer del resto del ejemplo.
///
/// ```rust
/// # use pagetop::prelude::*;
/// # let mut cx = Context::default();
/// let props = Props::new("hx-get", "/api/items")
///     .with_prop(PropsOp::set("hx-target", "#lista"))
///     .with_prop(PropsOp::set("hx-swap", "outerHTML"));
///
/// let markup = html! {
///     button (props.unpack(&mut cx)) { "Cargar" }
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
/// # let mut cx = Context::default();
/// let props = Props::default().with_id("My Button");
/// let markup = html! { button (props.unpack(&mut cx)) { "OK" } };
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
/// # let mut cx = Context::default();
/// let props = Props::default()
///     .with_prop(PropsOp::add_classes("btn btn-primary"))
///     .with_prop(PropsOp::add_classes("active"))
///     .with_prop(PropsOp::replace_classes("btn-primary", "btn-secondary"));
///
/// let markup = html! { button (props.unpack(&mut cx)) { "OK" } };
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
/// # let mut cx = Context::default();
/// let props = Props::default()
///     .with_prop(PropsOp::add_style("color", "red"))
///     .with_prop(PropsOp::add_style("font-weight", "bold"))
///     .with_prop(PropsOp::add_style("color", "blue"))
///     .with_prop(PropsOp::remove_style("font-weight"));
///
/// let markup = html! { button (props.unpack(&mut cx)) { "OK" } };
/// assert_eq!(markup.into_string(), r#"<button style="color: blue">OK</button>"#);
/// ```
///
/// # Atributos localizados
///
/// [`PropsOp::set()`] admite como valor un texto localizado ([`Lc`]) en lugar de un texto fijo.
/// Se traduce al renderizar, con el idioma del [`Context`] que recibe [`unpack()`](Self::unpack),
/// y se escapa como cualquier otro valor; si no tiene traducción, el atributo se omite. Sirve para
/// los atributos que un componente no tiene como campo propio (`aria-label`, `placeholder`,
/// `title`...), sin traducirlos antes de crearlo.
///
/// ```rust
/// # use pagetop::prelude::*;
/// let mut cx = Context::default().with_langid(&Locale::resolve("es-ES"));
/// let props = Props::default()
///     .with_prop(PropsOp::set("hx-post", "/items/7/delete"))
///     .with_prop(PropsOp::set("aria-label", Lc::l("button_delete")));
///
/// let markup = html! { button (props.unpack(&mut cx)) { "X" } };
/// assert_eq!(
///     markup.into_string(),
///     r#"<button hx-post="/items/7/delete" aria-label="Eliminar">X</button>"#
/// );
/// ```
///
/// Un atributo localizado sustituye al de texto fijo del mismo nombre y al revés, como dos
/// [`PropsOp::set()`] seguidos. [`get_prop()`](Self::get_prop) lo devuelve sin traducir, como
/// [`PropValue::Lc`], y [`PropValue::lookup()`] lo traduce al idioma que se le indique.
///
/// # Atributos duplicados junto a `Props`
///
/// Cuando el componente combina `(self.props().unpack(cx))` con un atributo del mismo nombre en el
/// mismo elemento (una clase, un `#id`, o `nombre=valor`), la macro [`html!`](crate::html::html)
/// evita automáticamente la duplicación. Recopila en tiempo de compilación los nombres de los
/// atributos del elemento y al renderizar se omiten los duplicados en tiempo de ejecución. No
/// depende del orden en que se escriban ni requiere ninguna acción del desarrollador.
///
/// ```rust
/// # use pagetop::prelude::*;
/// # let mut cx = Context::default();
/// let props = Props::default().with_prop(PropsOp::set("title", "de Props"));
///
/// let markup = html! { span title="literal" (props.unpack(&mut cx)) { "OK" } };
///
/// // El atributo literal prevalece; `Props` omite su propio "title" en vez de duplicarlo.
/// assert_eq!(markup.into_string(), r#"<span title="literal">OK</span>"#);
/// ```
///
/// Con `class` ocurre lo mismo. Si el elemento escribe alguna clase propia, con `class="..."` o con
/// la forma abreviada (`.clase`, `.clase[cond]`), se omiten **todas** las clases de `Props`. Para
/// combinar las clases del componente con las que se añadan con `with_prop()`, el componente debe
/// incorporarlas a sus `Props` en su `setup()`, por ejemplo con [`PropsOp::prepend_classes()`], y
/// no las escribe en el elemento.
///
/// ```rust
/// # use pagetop::prelude::*;
/// # let mut cx = Context::default();
/// let props = Props::classes("extra");
///
/// let markup = html! { div class="card" (props.unpack(&mut cx)) {} };
/// assert_eq!(markup.into_string(), r#"<div class="card"></div>"#);
///
/// let props = props.with_prop(PropsOp::prepend_classes("card"));
/// let markup = html! { div (props.unpack(&mut cx)) {} };
/// assert_eq!(markup.into_string(), r#"<div class="card extra"></div>"#);
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
///             button (self.props().unpack(cx)) {
///                 (self.label().using(cx))
///             }
///         })
///     }
/// }
///
/// #[builder_impl]
/// impl MyButton {
///     /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
///     pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
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
    attrs: Vec<(CowStr, PropValue)>,
    extras: HashMap<&'static str, PropsExtra>,
    // Posicionamiento Flexbox/Grid y espaciado. Cada uno lleva un valor por punto de corte, lo que
    // suma más de 1 KB, y casi ningún componente los usa. Se guardan aparte, tras un `Arc` que sólo
    // se reserva al establecer el primero. Así `Props` (y con él cada componente, el `Context` y la
    // `Page`) es diez veces menor, y clonar el componente para renderizarlo no copia esos valores,
    // sólo cuenta una referencia (`Arc::make_mut` los duplica si se modifican estando compartidos).
    layout: Option<Arc<Layout>>,
}

// Posicionamiento y espaciado de un componente; ver `Props::layout`.
#[derive(AutoDefault, Clone, Debug, PartialEq)]
struct Layout {
    flex_item: FlexItem,
    grid_item: GridItem,
    margin: Margin,
    padding: Padding,
}

#[builder_impl]
impl Props {
    /// Crea una colección con un primer atributo ya establecido.
    ///
    /// Como [`PropsOp::set()`], el valor puede ser texto fijo o un texto localizado ([`Lc`]), y no
    /// admite `"id"`, `"class"` ni `"style"`: para empezar con clases está
    /// [`classes()`](Self::classes), y el identificador se añade con [`with_id()`](Self::with_id).
    pub fn new(name: impl Into<CowStr>, value: impl Into<PropValue>) -> Self {
        Self::default().with_prop(PropsOp::set(name, value))
    }

    /// Crea una colección con las clases CSS iniciales indicadas.
    pub fn classes(classes: impl Into<CowStr>) -> Self {
        Self::default().with_prop(PropsOp::add_classes(classes))
    }

    // **< Props BUILDER >**************************************************************************

    /// Establece el identificador del componente; equivale a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.apply_id(id.into().as_ref());
        self
    }

    /// Modifica el identificador, las clases, los atributos o los valores extra según la operación
    /// indicada, incluido el posicionamiento Flexbox y Grid y el espaciado (`FlexItem`,
    /// `GridItem`, `Margin`, `Padding`). El método recomendado para construir cada operación es
    /// usar los constructores de [`PropsOp`], aunque `FlexItem`, `GridItem`, `Margin` y `Padding`
    /// pueden pasarse directamente gracias a sus `From` hacia `PropsOp`.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        match op.into() {
            PropsOp::SetId(value) => {
                self.apply_id(value.as_ref());
            }
            PropsOp::EnsureId(value) => {
                if self.id.is_none() {
                    self.apply_id(value.as_ref());
                }
            }
            PropsOp::AddClasses(classes) => {
                let Some(normalized) = util::normalize_ascii(classes.as_ref()) else {
                    return self;
                };
                let pos = self.classes.len();
                self.insert_classes(normalized.as_ref().split_ascii_whitespace(), pos);
            }
            PropsOp::PrependClasses(classes) => {
                let Some(normalized) = util::normalize_ascii(classes.as_ref()) else {
                    return self;
                };
                self.insert_classes(normalized.as_ref().split_ascii_whitespace(), 0);
            }
            PropsOp::ReplaceClasses(old, new) => {
                let Some(old) = util::normalize_ascii(old.as_ref()) else {
                    return self;
                };
                let Some(new) = util::normalize_ascii(new.as_ref()) else {
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
                let Some(old) = util::normalize_ascii(old.as_ref()) else {
                    return self;
                };
                let Some(new) = util::normalize_ascii(new.as_ref()) else {
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
                let Some(normalized) = util::normalize_ascii(classes.as_ref()) else {
                    return self;
                };
                self.classes.retain(|c| {
                    !normalized
                        .as_ref()
                        .split_ascii_whitespace()
                        .any(|r| r == c.as_str())
                });
            }
            PropsOp::ClearClasses => {
                self.classes.clear();
            }
            PropsOp::AddStyle(property, value) => {
                self.set_style(property.as_ref(), value.as_ref());
            }
            PropsOp::RemoveStyle(property) => {
                self.remove_style(property.as_ref());
            }
            PropsOp::ClearStyles => {
                self.styles.clear();
            }
            PropsOp::Set(name, value) => {
                if is_reserved(&name) {
                    return self;
                }
                if let Some(pos) = self.attrs.iter().position(|(k, _)| k == &name) {
                    self.attrs[pos].1 = value;
                } else {
                    self.attrs.push((name, value));
                }
            }
            PropsOp::Rename(from, to) => {
                if is_reserved(&from) || is_reserved(&to) {
                    return self;
                }
                if let Some(pos) = self.attrs.iter().position(|(k, _)| k == &from) {
                    let (_, value) = self.attrs.remove(pos);
                    if !self.attrs.iter().any(|(k, _)| k == &to) {
                        self.attrs.push((to, value));
                    }
                }
            }
            PropsOp::Remove(name) => {
                if is_reserved(&name) {
                    return self;
                }
                self.attrs.retain(|(k, _)| k != &name);
            }
            PropsOp::SetExtra(key, extra) => {
                self.extras.insert(key, extra);
            }
            PropsOp::RemoveExtra(key) => {
                self.extras.remove(key);
            }
            PropsOp::FlexItem(placement) => {
                let layout = self.layout_mut();
                layout.flex_item = layout.flex_item.merge(placement);
            }
            PropsOp::GridItem(placement) => {
                let layout = self.layout_mut();
                layout.grid_item = layout.grid_item.merge(placement);
            }
            PropsOp::Margin(margin) => {
                let layout = self.layout_mut();
                layout.margin = layout.margin.merge(margin);
            }
            PropsOp::Padding(padding) => {
                let layout = self.layout_mut();
                layout.padding = layout.padding.merge(padding);
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
        let property = util::normalize_property(property)?;
        self.styles
            .iter()
            .find(|(k, _)| k.as_ref() == property)
            .map(|(_, v)| v.to_string())
    }

    /// Devuelve el valor del atributo indicado, si existe: un texto fijo o un texto localizado, sin
    /// traducir. Para leerlo, [`PropValue::lookup()`] lo devuelve en el idioma indicado, sea cual
    /// sea su tipo.
    ///
    /// `"id"`, `"class"` y `"style"` no son atributos de esta lista y devuelven siempre `None`; se
    /// consultan con [`get_id()`](Self::get_id), [`get_classes()`](Self::get_classes) y
    /// [`get_styles()`](Self::get_styles).
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let props = Props::new("hx-get", "/items").with_prop(PropsOp::set("title", Lc::n("Items")));
    /// assert!(matches!(props.get_prop("title"), Some(PropValue::Lc(_))));
    /// assert!(props.get_prop("hx-post").is_none());
    ///
    /// let lang = Locale::default();
    /// let value = |name| props.get_prop(name).and_then(|v| v.lookup(&lang));
    /// assert_eq!(value("hx-get").as_deref(), Some("/items"));
    /// assert_eq!(value("title").as_deref(), Some("Items"));
    /// ```
    pub fn get_prop(&self, name: impl AsRef<str>) -> Option<&PropValue> {
        let name = name.as_ref();
        self.attrs
            .iter()
            .find(|(k, _)| k.as_ref() == name)
            .map(|(_, value)| value)
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

    /// Devuelve `true` si no hay ningún identificador, clases, estilos, atributos adicionales,
    /// posicionamiento Flexbox/Grid ni espaciado definidos, sin tener en cuenta los valores extra
    /// (que nunca se emiten en el HTML, ver [`extra()`](Self::extra)).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.classes.is_empty()
            && self.styles.is_empty()
            && self.attrs.is_empty()
            && self
                .layout
                .as_deref()
                .is_none_or(|layout| *layout == Layout::default())
    }

    /// Devuelve `true` si la clase o **alguna** de las clases indicadas está presente.
    pub fn has_classes(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii_non_blank(classes.as_ref()) else {
            return false;
        };
        normalized
            .as_ref()
            .split_ascii_whitespace()
            .any(|class| self.classes.iter().any(|c| c == class))
    }

    /// Devuelve `true` si la clase o **todas** las clases indicadas están presentes.
    pub fn has_all_classes(&self, classes: impl AsRef<str>) -> bool {
        let Ok(normalized) = util::normalize_ascii_non_blank(classes.as_ref()) else {
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

    /// Devuelve el valor extra clonado o el **valor `default`** si no existe.
    ///
    /// En desarrollo y en pruebas falla si existe con otro tipo; en producción devuelve también
    /// `default`.
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
        self.extra_cloned(key).unwrap_or(default)
    }

    /// Devuelve el valor extra clonado o el **valor por defecto del tipo** si no existe.
    ///
    /// En desarrollo y en pruebas falla si existe con otro tipo; en producción devuelve también el
    /// valor por defecto.
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
        self.extra_cloned(key).unwrap_or_default()
    }

    /// Devuelve el valor extra clonado o el **valor evaluado por la función `f`** si no existe.
    ///
    /// En desarrollo y en pruebas falla si existe con otro tipo; en producción evalúa también `f`.
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
        self.extra_cloned(key).unwrap_or_else(f)
    }

    // Un valor ausente es normal; pedirlo con un tipo distinto del guardado es un error de
    // programación.
    fn extra_cloned<T: Clone + 'static>(&self, key: &'static str) -> Option<T> {
        self.extra::<T>(key)
            .inspect_err(|err| {
                debug_assert!(
                    !matches!(err, PropsError::ExtraTypeMismatch { .. }),
                    "{err}"
                );
            })
            .ok()
            .cloned()
    }

    // **< Props RENDER >***************************************************************************

    /// Extrae `Props` en la posición de atributos de [`html!`] usando el `Context` activo:
    /// `button (self.props().unpack(cx)) { ... }`.
    ///
    /// `Props` no implementa [`RenderAttrs`] directamente. Obliga a pasar siempre el `Context`
    /// vigente en el punto donde se renderiza, aunque no lo necesite ningún atributo propio. Recibe
    /// `&mut Context` porque aquí, en el momento de extraer los atributos, es donde se resuelven
    /// [`FlexItem`], [`GridItem`], [`Margin`] y [`Padding`]: las clases que devuelven se añaden a
    /// las del propio componente al escribir el atributo `class`. Su idioma es también el de los
    /// atributos localizados (con un [`Lc`] como valor).
    ///
    /// [`Flex`] y [`Grid`] resuelven su propio posicionamiento (Flexbox/CSS Grid) directamente: no
    /// son una propiedad de `Props`, son los únicos componentes que lo ofrecen.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// # let mut cx = Context::default();
    /// let props = Props::default().with_id("example");
    /// let markup = html! { button (props.unpack(&mut cx)) { "OK" } };
    /// assert_eq!(markup.into_string(), r#"<button id="example">OK</button>"#);
    /// ```
    ///
    /// [`html!`]: crate::html::html
    /// [`FlexItem`]: crate::html::flex::FlexItem
    /// [`GridItem`]: crate::html::grid::GridItem
    /// [`Margin`]: crate::html::spacing::Margin
    /// [`Padding`]: crate::html::spacing::Padding
    /// [`Flex`]: crate::base::component::Flex
    /// [`Grid`]: crate::base::component::Grid
    pub fn unpack<'a>(&'a self, cx: &mut Context) -> impl RenderAttrs + 'a {
        self.unpack_with_classes(cx, String::new())
    }

    /// Como [`unpack()`](Self::unpack), pero añade `classes` a las clases del componente sólo en
    /// este renderizado, sin modificar `Props`.
    ///
    /// Sirve en `prepare()`, que sólo recibe `&self`, para las clases que dependen del [`Context`]
    /// o de lo que se calcula al renderizar; las clases fijas del componente se añaden a sus
    /// `Props` en `setup()`. Así lo hacen [`Flex`], [`Grid`] y [`Mosaic`] con las clases de su
    /// posicionamiento.
    ///
    /// `classes` son nombres separados por espacios que se escriben tal cual, sin normalizar ni
    /// quitar duplicados, después de las clases del componente y antes de las que resuelven
    /// [`FlexItem`], [`GridItem`], [`Margin`] y [`Padding`].
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// # let mut cx = Context::default();
    /// let props = Props::classes("card").with_id("news");
    /// let markup = html! { div (props.unpack_with_classes(&mut cx, "card-wide")) {} };
    /// assert_eq!(markup.into_string(), r#"<div id="news" class="card card-wide"></div>"#);
    /// ```
    ///
    /// [`FlexItem`]: crate::html::flex::FlexItem
    /// [`GridItem`]: crate::html::grid::GridItem
    /// [`Margin`]: crate::html::spacing::Margin
    /// [`Padding`]: crate::html::spacing::Padding
    /// [`Flex`]: crate::base::component::Flex
    /// [`Grid`]: crate::base::component::Grid
    /// [`Mosaic`]: crate::base::component::Mosaic
    pub fn unpack_with_classes<'a>(
        &'a self,
        cx: &mut Context,
        classes: impl Into<String>,
    ) -> impl RenderAttrs + 'a {
        let mut classes = classes.into();
        if let Some(layout) = self.layout.as_deref() {
            layout.flex_item.apply(cx, &mut classes);
            layout.grid_item.apply(cx, &mut classes);
            layout.margin.apply(cx, &mut classes);
            layout.padding.apply(cx, &mut classes);
        }
        PropsUnpack {
            props: self,
            classes,
            lang: Lang(cx.langid()),
        }
    }

    // **< Props PRIVATE >**************************************************************************

    // Posicionamiento y espaciado para modificarlos: los crea con sus valores por defecto si aún no
    // existen y, si están compartidos con un clon, los duplica antes.
    fn layout_mut(&mut self) -> &mut Layout {
        Arc::make_mut(self.layout.get_or_insert_with(Default::default))
    }

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
    // propiedad o el valor quedan vacíos tras recortar espacios. No aplica `normalize_ascii`: ver
    // la documentación de `PropsOp::AddStyle` sobre por qué los valores de estilo no se restringen
    // a ASCII.
    fn set_style(&mut self, property: &str, value: &str) {
        let Some(property) = util::normalize_property(property) else {
            return;
        };
        let Some(value) = util::non_blank(value) else {
            return;
        };
        if let Some(pos) = self.styles.iter().position(|(k, _)| k.as_ref() == property) {
            self.styles[pos].1 = value.to_string().into();
        } else {
            self.styles
                .push((property.into(), value.to_string().into()));
        }
    }

    // Elimina la propiedad de estilo indicada, si existe.
    fn remove_style(&mut self, property: &str) {
        if let Some(property) = util::normalize_property(property) {
            self.styles.retain(|(k, _)| k.as_ref() != property);
        };
    }
}

impl Props {
    // Escribe los atributos, omitiendo cualquiera que esté en `exclude` (recopilados por `html!` a
    // partir de los atributos literales del elemento). Registra un `trace::debug!` por cada
    // atributo duplicado, con la posición exacta del `html!` que lo produjo (propagado gracias a
    // `#[track_caller]`, heredado desde `PropsUnpack::render_attrs_to()`) para facilitar la
    // localización del problema.
    #[track_caller]
    fn write_attrs(&self, classes: &str, lang: Lang, w: &mut String, exclude: &[&str]) {
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
        // Clases propias del componente + las que aplica `Props::unpack()`/`unpack_with_classes()`.
        let mut all_classes: Vec<&str> = self.classes.iter().map(String::as_str).collect();
        all_classes.extend(classes.split_ascii_whitespace());
        if let Some((first, rest)) = all_classes.split_first() {
            if exclude.contains(&"class") {
                trace::debug!(
                    caller = %Location::caller(),
                    attribute = "class",
                    discarded = %all_classes.join(" "),
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
                    discarded = ?value,
                    id = %self.id.as_deref().unwrap_or("<none>"),
                    "Ignoring Props attribute already set as a literal on the same element"
                );
                continue;
            }
            let translated;
            let value = match value {
                PropValue::Text(text) => text.as_ref(),
                PropValue::Lc(lc) => match translate(lc, lang) {
                    Some(text) => {
                        translated = text;
                        translated.as_str()
                    }
                    None => continue,
                },
            };
            w.push(' ');
            let _ = write!(Escaper::new(w), "{}", name);
            w.push_str("=\"");
            let _ = write!(Escaper::new(w), "{}", value);
            w.push('"');
        }
    }
}

// `id`, `class` y `style` tienen sus propias operaciones; `Set`, `Rename` y `Remove` las ignoran.
// Sin distinguir mayúsculas, como HTML: `Class` también es el atributo `class`.
fn is_reserved(name: &str) -> bool {
    let reserved = ["id", "class", "style"]
        .iter()
        .any(|reserved| name.eq_ignore_ascii_case(reserved));
    debug_assert!(
        !reserved,
        "attribute `{name}` has its own PropsOp variants; Set, Rename and Remove do not accept it"
    );
    reserved
}

// Fuera de línea: dentro del bucle de `write_attrs()`, que recorre sobre todo texto, empeora el
// código generado para todos los atributos.
#[inline(never)]
fn translate(lc: &Lc, lang: Lang) -> Option<String> {
    lc.lookup(&lang)
}

// **< PropsUnpack >********************************************************************************

// Devuelto por `Props::unpack()` y por `unpack_with_classes()` (`unpack()` la llama con un
// acumulador vacío). `classes` son las clases que recibe `unpack_with_classes()` (en `Flex`/`Grid`,
// las de sus métodos privados `flex_classes()`/`grid_classes()`) más las de `FlexItem::apply()`/
// `GridItem::apply()`/`Margin::apply()`/`Padding::apply()`, pendientes sólo de añadir a las del
// componente (ver `Props::write_attrs()`). `lang` es el idioma del `Context` de `unpack()`, para
// los atributos localizados.
struct PropsUnpack<'a> {
    props: &'a Props,
    classes: String,
    lang: Lang,
}

// Idioma del renderizado, copiado del `Context` para traducir sin tenerlo prestado.
#[derive(Clone, Copy)]
struct Lang(&'static LanguageIdentifier);

impl LangId for Lang {
    #[inline]
    fn langid(&self) -> &'static LanguageIdentifier {
        self.0
    }
}

#[doc(hidden)]
impl RenderAttrs for PropsUnpack<'_> {
    #[track_caller]
    fn render_attrs_to(&self, w: &mut String, exclude: &[&str]) {
        self.props.write_attrs(&self.classes, self.lang, w, exclude);
    }
}
