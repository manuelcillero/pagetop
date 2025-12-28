use crate::locale::{L10n, LangId};
use crate::{builder_fn, AutoDefault};

/// Valor opcional para atributos HTML.
///
/// `Attr<T>` encapsula un `Option<T>` y sirve como tipo base para representar atributos HTML
/// opcionales, uniformes y tipados.
///
/// Este tipo **no impone ninguna normalización ni semántica concreta**; dichas reglas se definen en
/// implementaciones concretas como `Attr<L10n>` y `Attr<String>`, o en tipos específicos como
/// [`AttrId`] y [`AttrName`].
#[derive(AutoDefault, Clone, Debug)]
pub struct Attr<T>(Option<T>);

impl<T> Attr<T> {
    /// Crea un atributo vacío.
    pub fn empty() -> Self {
        Self(None)
    }

    /// Crea un atributo con valor.
    pub fn some(value: T) -> Self {
        Self(Some(value))
    }

    // **< Attr<T> BUILDER >************************************************************************

    /// Establece un valor para el atributo.
    #[builder_fn]
    pub fn with_value(mut self, value: T) -> Self {
        self.0 = Some(value);
        self
    }

    /// Elimina el valor del atributo.
    #[builder_fn]
    pub fn with_none(mut self) -> Self {
        self.0 = None;
        self
    }

    // **< Attr<T> GETTERS >************************************************************************

    /// Devuelve el valor (clonado), si existe.
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.0.clone()
    }

    /// Devuelve una referencia al valor, si existe.
    pub fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }

    /// Devuelve el valor (propiedad), si existe.
    pub fn into_inner(self) -> Option<T> {
        self.0
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

// **< Attr<L10n> >*********************************************************************************

/// Extiende [`Attr`] para [texto localizado](crate::locale) en atributos HTML.
///
/// Encapsula un [`L10n`] para manejar traducciones de forma segura en atributos.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Traducción por clave en las locales por defecto de PageTop.
/// let hello = Attr::<L10n>::new(L10n::l("test_hello_world"));
///
/// // Español disponible.
/// assert_eq!(
///     hello.lookup(&Locale::resolve("es-ES")),
///     Some("¡Hola mundo!".to_string())
/// );
///
/// // Japonés no disponible, traduce al idioma de respaldo (`"en-US"`).
/// assert_eq!(
///     hello.lookup(&Locale::resolve("ja-JP")),
///     Some("Hello world!".to_string())
/// );
///
/// // Uso típico en un atributo:
/// let title = hello.value(&Locale::resolve("es-ES"));
/// // Ejemplo: html! { a title=(title) { "Link" } }
/// ```
impl Attr<L10n> {
    /// Crea una nueva instancia `Attr<L10n>`.
    pub fn new(value: L10n) -> Self {
        Self::some(value)
    }

    /// Devuelve la traducción para `language` si puede resolverse.
    pub fn lookup(&self, language: &impl LangId) -> Option<String> {
        self.0.as_ref()?.lookup(language)
    }

    /// Devuelve la traducción para `language` o una cadena vacía si no existe.
    pub fn value(&self, language: &impl LangId) -> String {
        self.lookup(language).unwrap_or_default()
    }
}

// **< Attr<String> >*******************************************************************************

/// Extiende [`Attr`] para cadenas de texto.
impl Attr<String> {
    /// Devuelve el texto como `&str` si existe.
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

// **< AttrId >*************************************************************************************

/// Identificador normalizado para el atributo `id` o similar de HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso:
///
/// - Se eliminan los espacios al principio y al final.
/// - Se convierte a minúsculas.
/// - Se sustituyen los espacios (`' '`) intermedios por guiones bajos (`_`).
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let id = AttrId::new("  main Section ");
/// assert_eq!(id.as_str(), Some("main_section"));
///
/// let empty = AttrId::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct AttrId(Attr<String>);

impl AttrId {
    /// Crea un nuevo `AttrId` normalizando el valor.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self::default().with_id(id)
    }

    // **< AttrId BUILDER >*************************************************************************

    /// Establece un identificador nuevo normalizando el valor.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        let id = id.as_ref().trim();
        if id.is_empty() {
            self.0 = Attr::default();
        } else {
            self.0 = Attr::some(id.to_ascii_lowercase().replace(' ', "_"));
        }
        self
    }

    // **< AttrId GETTERS >*************************************************************************

    /// Devuelve el identificador normalizado, si existe.
    pub fn get(&self) -> Option<String> {
        self.0.get()
    }

    /// Devuelve el identificador normalizado (sin clonar), si existe.
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_str()
    }

    /// Devuelve el identificador normalizado (propiedad), si existe.
    pub fn into_inner(self) -> Option<String> {
        self.0.into_inner()
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// **< AttrName >***********************************************************************************

/// Nombre normalizado para el atributo `name` o similar de HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso:
///
/// - Se eliminan los espacios al principio y al final.
/// - Se convierte a minúsculas.
/// - Se sustituyen los espacios (`' '`) intermedios por guiones bajos (`_`).
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let name = AttrName::new("  DISplay name ");
/// assert_eq!(name.as_str(), Some("display_name"));
///
/// let empty = AttrName::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct AttrName(Attr<String>);

impl AttrName {
    /// Crea un nuevo `AttrName` normalizando el valor.
    pub fn new(name: impl AsRef<str>) -> Self {
        Self::default().with_name(name)
    }

    // **< AttrName BUILDER >***********************************************************************

    /// Establece un nombre nuevo normalizando el valor.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        let name = name.as_ref().trim();
        if name.is_empty() {
            self.0 = Attr::default();
        } else {
            self.0 = Attr::some(name.to_ascii_lowercase().replace(' ', "_"));
        }
        self
    }

    // **< AttrName GETTERS >***********************************************************************

    /// Devuelve el nombre normalizado, si existe.
    pub fn get(&self) -> Option<String> {
        self.0.get()
    }

    /// Devuelve el nombre normalizado (sin clonar), si existe.
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_str()
    }

    /// Devuelve el nombre normalizado (propiedad), si existe.
    pub fn into_inner(self) -> Option<String> {
        self.0.into_inner()
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// **< AttrValue >**********************************************************************************

/// Cadena normalizada para renderizar en atributos HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso:
///
/// - Se eliminan los espacios al principio y al final.
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let s = AttrValue::new("  a new string   ");
/// assert_eq!(s.as_str(), Some("a new string"));
///
/// let empty = AttrValue::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct AttrValue(Attr<String>);

impl AttrValue {
    /// Crea un nuevo `AttrValue` normalizando el valor.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self::default().with_str(value)
    }

    // **< AttrValue BUILDER >**********************************************************************

    /// Establece una cadena nueva normalizando el valor.
    #[builder_fn]
    pub fn with_str(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().trim();
        if value.is_empty() {
            self.0 = Attr::default();
        } else {
            self.0 = Attr::some(value.to_string());
        }
        self
    }

    // **< AttrValue GETTERS >**********************************************************************

    /// Devuelve la cadena normalizada, si existe.
    pub fn get(&self) -> Option<String> {
        self.0.get()
    }

    /// Devuelve la cadena normalizada (sin clonar), si existe.
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_str()
    }

    /// Devuelve la cadena normalizada (propiedad), si existe.
    pub fn into_inner(self) -> Option<String> {
        self.0.into_inner()
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
