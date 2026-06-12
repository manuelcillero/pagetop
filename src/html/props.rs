use crate::html::maud::{Escaper, Render};
use crate::{AutoDefault, CowStr, builder_fn, util};

use std::fmt::Write;

// **< PropsOp >************************************************************************************

/// Operaciones disponibles sobre atributos HTML y clases CSS en [`Props`].
///
/// Cada variante es autocontenida, lleva todos los datos que necesita para ejecutarse. El método
/// recomendado para construirlas es usar los constructores asociados ([`set()`](Self::set),
/// [`remove()`](Self::remove), [`add_classes()`](Self::add_classes), etc.).
///
/// Las variantes `*Classes` operan siempre sobre la lista de clases CSS para el componente.
///
/// Cuando se usa `"class"` como nombre de atributo en `Set` o `Remove` la operación se aplica a la
/// lista de clases completa. Así, `Set("class", ...)` reemplaza la lista de clases completa por las
/// nuevas clases indicadas, y `Remove("class")` vacía la lista de clases.
#[derive(Clone, Debug, PartialEq)]
pub enum PropsOp {
    /// Añade el atributo o sustituye su valor si ya existe. Usar `"class"` como nombre reemplaza la
    /// lista completa de clases por las nuevas indicadas; la operación se ignora si el valor
    /// contiene caracteres no ASCII.
    Set(CowStr, CowStr),
    /// Elimina el atributo indicado. Usar `"class"` como nombre vacía la lista de clases.
    Remove(CowStr),
    /// Añade las clases que no existan al final de la lista.
    AddClasses(CowStr),
    /// Añade las clases que no existan al principio de la lista.
    PrependClasses(CowStr),
    /// Elimina las clases indicadas de la lista.
    RemoveClasses(CowStr),
}

impl PropsOp {
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

/// Colección de pares `atributo="valor"` y clases CSS para aplicar en componentes.
///
/// Permite añadir dinámicamente pares `atributo="valor"` y clases CSS al elemento raíz de un
/// componente. Al renderizar los atributos en `html!` primero emite el atributo `class` (si hay
/// clases) y luego el resto de atributos.
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
/// impl Component for MyButton {
///     fn new() -> Self { Self::default() }
///
///     fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
///         Ok(html! {
///             button (self.props()) {
///                 (self.label().using(cx))
///             }
///         })
///     }
/// }
///
/// impl MyButton {
///     /// Modifica los atributos HTML o las clases CSS del elemento raíz.
///     #[builder_fn]
///     pub fn with_prop(mut self, op: PropsOp) -> Self {
///         self.props.alter_prop(op);
///         self
///     }
/// }
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct Props {
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

    /// Modifica los atributos o clases según la operación indicada.
    ///
    /// - [`Set(name, value)`](PropsOp::Set) añade el atributo o reemplaza su valor.
    ///   `Set("class", ...)` reemplaza la lista de clases completa.
    /// - [`Remove(name)`](PropsOp::Remove) elimina el atributo. `Remove("class")` vacía la lista de
    ///   clases.
    /// - [`AddClasses(clases)`](PropsOp::AddClasses) añade clases al final (sin duplicados).
    /// - [`PrependClasses(clases)`](PropsOp::PrependClasses) añade clases al principio (sin
    ///   duplicados).
    /// - [`RemoveClasses(clases)`](PropsOp::RemoveClasses) elimina las clases indicadas.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        match op {
            PropsOp::Set(name, value) => {
                if name.as_ref() == "class" {
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
                if name.as_ref() == "class" {
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

    /// Devuelve el valor del atributo indicado, si existe.
    pub fn get_prop(&self, name: impl AsRef<str>) -> Option<&str> {
        let name = name.as_ref();
        self.attrs
            .iter()
            .find(|(k, _)| k.as_ref() == name)
            .map(|(_, v)| v.as_ref())
    }

    /// Devuelve `true` si no hay ningún atributo definido.
    pub fn is_props_empty(&self) -> bool {
        self.attrs.is_empty()
    }

    /// Devuelve la lista de clases como cadena de texto, si hay clases definidas.
    pub fn get_classes(&self) -> Option<String> {
        if self.classes.is_empty() {
            None
        } else {
            Some(self.classes.join(" "))
        }
    }

    /// Devuelve `true` si no hay ninguna clase definida.
    pub fn is_classes_empty(&self) -> bool {
        self.classes.is_empty()
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

    // **< Props PRIVADO >**************************************************************************

    #[inline]
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
