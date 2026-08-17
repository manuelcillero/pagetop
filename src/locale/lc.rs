use crate::html::{Markup, PreEscaped, Render};
use crate::{AutoDefault, CowStr, include_locales};

use super::{LangId, Locale};

include_locales!(LOCALES_PAGETOP);

use fluent_templates::Loader;
use fluent_templates::StaticLoader as Locales;

use std::collections::HashMap;

use std::fmt;

// Tipo de localización a aplicar.
//
// * `None` - No se aplica ninguna localización.
// * `Text` - Con una cadena literal que se devolverá tal cual.
// * `Translate` - Con la clave a resolver en el `Locales` indicado.
#[derive(AutoDefault, Clone, Debug)]
enum LcKind {
    #[default]
    None,
    Text(CowStr),
    Translate(CowStr),
}

/// Crea instancias para traducir *textos localizados*.
///
/// Cada instancia puede representar:
///
/// - Un texto puro (`n()`) que no requiere traducción.
/// - Una clave para traducir un texto del conjunto de traducciones predefinidas de PageTop (`l()`).
/// - Una clave para traducir de un conjunto concreto de traducciones (`t()`).
/// - Ningún contenido (`none()`), para representar la ausencia en un campo `Lc` opcional.
///
/// # ¿Cuál usar, `get()`, `lookup()` o `using()`?
///
/// Los tres métodos resuelven la traducción; difieren en el tipo que devuelven y en la fuente de
/// idioma que aceptan.
///
/// - [`get()`](Self::get) y [`lookup()`](Self::lookup) devuelven `Option<String>`: el texto
///   traducido en bruto, con el escapado habitual de cualquier `String` al interpolarlo, y `None`
///   si no aplica o no hay traducción. Son la opción adecuada para cualquier valor de atributo
///   (`attr=[...]` en [`html!`](crate::html::html)), como `title`, `aria-label` o `alt`.
///
///   Entre ambos, la opción habitual es [`lookup()`](Self::lookup), que acepta una fuente de idioma
///   explícita (`&impl LangId`, normalmente `cx: &Context`). [`get()`](Self::get) queda para los
///   casos marginales en los que no hay `Context` ni otra fuente de idioma a mano, y resuelve con
///   el idioma por defecto o de respaldo de la aplicación.
///
/// - [`using()`](Self::using) devuelve [`Markup`] para presentar al usuario, listo para insertarse
///   en el contenido de una plantilla (`(x.using(cx))` dentro de [`html!`](crate::html::html)).
///   Nunca devuelve `None`, por lo que una traducción no encontrada se convierte en marcado vacío.
///   Para `n()` se escapa el texto para evitar HTML no controlado. Para valores de atributo, usa
///   siempre [`lookup()`](Self::lookup).
///
/// # Ejemplo
///
/// Los argumentos dinámicos se añaden con `with_arg()` o `with_args()`.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// // Texto literal sin traducción.
/// let raw = Lc::n("© 2025 PageTop").get();
///
/// // Traducción simple con clave y argumentos.
/// let hello = Lc::l("greeting")
///     .with_arg("name", "Manuel")
///     .get();
/// ```
///
/// También sirve para traducciones contra un conjunto de recursos concreto.
///
/// ```rust,ignore
/// // Traducción con clave, conjunto de traducciones y fuente de idioma.
/// let bye = Lc::t("goodbye", &LOCALES_CUSTOM).lookup(&Locale::resolve("it"));
/// ```
#[derive(AutoDefault, Clone)]
pub struct Lc {
    op: LcKind,
    #[default(&LOCALES_PAGETOP)]
    locales: &'static Locales,
    args: Vec<(CowStr, CowStr)>,
}

impl fmt::Debug for Lc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lc")
            .field("op", &self.op)
            .field("args", &self.args)
            // No se puede mostrar `locales`; se representa con un texto fijo.
            .field("locales", &"<StaticLoader>")
            .finish()
    }
}

impl Lc {
    /// **n** = *“native”*. Crea una instancia con una cadena literal sin traducción.
    pub fn n(text: impl Into<CowStr>) -> Self {
        Self {
            op: LcKind::Text(text.into()),
            ..Default::default()
        }
    }

    /// **l** = *“library”*. Crea una instancia con una clave del conjunto de traducciones
    /// predefinidas propias de PageTop.
    pub fn l(key: impl Into<CowStr>) -> Self {
        Self {
            op: LcKind::Translate(key.into()),
            ..Default::default()
        }
    }

    /// **t** = *“translate”*. Crea una instancia con una clave de un conjunto de traducciones
    /// específico.
    pub fn t(key: impl Into<CowStr>, locales: &'static Locales) -> Self {
        Self {
            op: LcKind::Translate(key.into()),
            locales,
            ..Default::default()
        }
    }

    /// Crea una instancia **sin contenido**: no traduce nada y no representa ningún texto.
    ///
    /// Equivale a [`Lc::default()`](Default::default), con un nombre más explícito. Útil para
    /// representar la ausencia en un campo `Lc` opcional sin requerir `Option<Lc>`:
    /// [`get()`](Self::get) y [`lookup()`](Self::lookup) devuelven `None`, y
    /// [`using()`](Self::using) devuelve un marcado vacío.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// assert_eq!(Lc::none().get(), None);
    /// ```
    pub fn none() -> Self {
        Self::default()
    }

    // **< Lc BUILDER >*****************************************************************************

    /// Añade un argumento `{$arg}` => `value` a la traducción.
    pub fn with_arg(mut self, arg: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        self.args.push((arg.into(), value.into()));
        self
    }

    /// Añade varios argumentos a la traducción de una vez (p. ej. usando la macro
    /// [`util::kv!`](crate::util::kv) o también `vec![("k", "v")]`, incluso un array de duplas u
    /// otras colecciones).
    pub fn with_args<I, K, V>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<CowStr>,
        V: Into<CowStr>,
    {
        self.args
            .extend(args.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    // **< Lc GETTERS >*****************************************************************************

    /// Resuelve la traducción usando el idioma por defecto o, si no procede, el de respaldo de la
    /// aplicación.
    ///
    /// Devuelve `None` si no aplica o no encuentra una traducción válida.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let text = Lc::l("greeting").with_arg("name", "Manuel").get();
    /// ```
    pub fn get(&self) -> Option<String> {
        self.lookup(&Locale::default())
    }

    /// Resuelve la traducción usando la fuente de idioma proporcionada.
    ///
    /// Devuelve `None` si no aplica o no encuentra una traducción válida.
    ///
    /// Es la opción adecuada para cualquier valor de atributo (`attr=[...]` en
    /// [`html!`](crate::html::html)). Para insertar el resultado directamente en el contenido de
    /// una plantilla, utiliza [`using()`](Self::using) en su lugar, que ya devuelve [`Markup`] con
    /// el escapado adecuado y nunca `None`.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// struct ResourceLang;
    ///
    /// impl LangId for ResourceLang {
    ///     fn langid(&self) -> &'static LanguageIdentifier {
    ///         Locale::resolve("es-MX").langid()
    ///     }
    /// }
    ///
    /// let r = ResourceLang;
    /// let text = Lc::l("greeting").with_arg("name", "Usuario").lookup(&r);
    /// ```
    pub fn lookup(&self, language: &impl LangId) -> Option<String> {
        match &self.op {
            LcKind::None => None,
            LcKind::Text(text) => Some(text.clone().into_owned()),
            LcKind::Translate(key) => {
                if self.args.is_empty() {
                    self.locales.try_lookup(language.langid(), key.as_ref())
                } else {
                    let mut args = HashMap::with_capacity(self.args.len());
                    for (k, v) in self.args.iter() {
                        args.insert(k.clone(), v.as_ref().into());
                    }
                    self.locales
                        .try_lookup_with_args(language.langid(), key.as_ref(), &args)
                }
            }
        }
    }

    /// Traduce el texto y lo devuelve como [`Markup`] usando la fuente de idioma proporcionada.
    ///
    /// Devuelve un marcado vacío si no aplica o no encuentra una traducción válida.
    ///
    /// Las claves de traducción (`l()`/`t()`) son texto de confianza, con traducciones en ficheros
    /// `.ftl` que pueden incluir marcado HTML (p. ej. `<strong>`, enlaces, etc.) que se insertan
    /// tal cual. El texto literal (`n()`), en cambio, suele venir de datos en tiempo de ejecución
    /// que no debe interpretarse como marcado, por lo que se escapa igual que cualquier otro valor
    /// interpolado con [`html!`](crate::html::html).
    ///
    /// No debe usarse para un valor de atributo (`attr=[...]` en [`html!`](crate::html::html)): el
    /// marcado de confianza que se aplica en `l()`/`t()` no tiene sentido ahí, y podría romper el
    /// delimitador del atributo si la traducción contuviera una comilla. Para atributos, usa
    /// siempre [`lookup()`](Self::lookup) en su lugar.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let html = Lc::l("welcome.message").using(&Locale::resolve("es"));
    /// ```
    pub fn using(&self, language: &impl LangId) -> Markup {
        match &self.op {
            LcKind::Text(text) => text.render(),
            _ => PreEscaped(self.lookup(language).unwrap_or_default()),
        }
    }
}
