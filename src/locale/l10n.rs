use crate::html::{Markup, PreEscaped};
use crate::{include_locales, AutoDefault, CowStr};

use super::{LangId, Locale};

use fluent_templates::Loader;
use fluent_templates::StaticLoader as Locales;

use std::collections::HashMap;

use std::fmt;

include_locales!(LOCALES_PAGETOP);

/// Operación de localización a realizar.
///
/// * `None` - No se aplica ninguna localización.
/// * `Text` - Con una cadena literal que se devolverá tal cual.
/// * `Translate` - Con la clave a resolver en el `Locales` indicado.
#[derive(AutoDefault, Clone, Debug)]
enum L10nOp {
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
/// - Una clave para traducir un texto de las traducciones predefinidas de PageTop (`l()`).
/// - Una clave para traducir de un conjunto concreto de traducciones (`t()`).
///
/// # Ejemplo
///
/// Los argumentos dinámicos se añaden con `with_arg()` o `with_args()`.
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Texto literal sin traducción.
/// let raw = L10n::n("© 2025 PageTop").get();
///
/// // Traducción simple con clave y argumentos.
/// let hello = L10n::l("greeting")
///     .with_arg("name", "Manuel")
///     .get();
/// ```
///
/// También sirve para traducciones contra un conjunto de recursos concreto.
///
/// ```rust,ignore
/// // Traducción con clave, conjunto de traducciones y fuente de idioma.
/// let bye = L10n::t("goodbye", &LOCALES_CUSTOM).lookup(&Locale::resolve("it"));
/// ```
#[derive(AutoDefault, Clone)]
pub struct L10n {
    op: L10nOp,
    #[default(&LOCALES_PAGETOP)]
    locales: &'static Locales,
    args: Vec<(CowStr, CowStr)>,
}

impl L10n {
    /// **n** = *“native”*. Crea una instancia con una cadena literal sin traducción.
    pub fn n(text: impl Into<CowStr>) -> Self {
        Self {
            op: L10nOp::Text(text.into()),
            ..Default::default()
        }
    }

    /// **l** = *“lookup”*. Crea una instancia para traducir usando una clave del conjunto de
    /// traducciones predefinidas.
    pub fn l(key: impl Into<CowStr>) -> Self {
        Self {
            op: L10nOp::Translate(key.into()),
            ..Default::default()
        }
    }

    /// **t** = *“translate”*. Crea una instancia para traducir usando una clave de un conjunto de
    /// traducciones específico.
    pub fn t(key: impl Into<CowStr>, locales: &'static Locales) -> Self {
        Self {
            op: L10nOp::Translate(key.into()),
            locales,
            ..Default::default()
        }
    }

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

    /// Resuelve la traducción usando el idioma por defecto o, si no procede, el de respaldo de la
    /// aplicación.
    ///
    /// Devuelve `None` si no aplica o no encuentra una traducción válida.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let text = L10n::l("greeting").with_arg("name", "Manuel").get();
    /// ```
    pub fn get(&self) -> Option<String> {
        self.lookup(&Locale::default())
    }

    /// Resuelve la traducción usando la fuente de idioma proporcionada.
    ///
    /// Devuelve `None` si no aplica o no encuentra una traducción válida.
    ///
    /// # Ejemplo
    ///
    /// ```rust
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
    /// let text = L10n::l("greeting").with_arg("name", "Usuario").lookup(&r);
    /// ```
    pub fn lookup(&self, language: &impl LangId) -> Option<String> {
        match &self.op {
            L10nOp::None => None,
            L10nOp::Text(text) => Some(text.clone().into_owned()),
            L10nOp::Translate(key) => {
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
    /// Si no se encuentra una traducción válida, devuelve una cadena vacía.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let html = L10n::l("welcome.message").using(&Locale::resolve("es"));
    /// ```
    pub fn using(&self, language: &impl LangId) -> Markup {
        PreEscaped(self.lookup(language).unwrap_or_default())
    }
}

impl fmt::Debug for L10n {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("L10n")
            .field("op", &self.op)
            .field("args", &self.args)
            // No se puede mostrar `locales`; se representa con un texto fijo.
            .field("locales", &"<StaticLoader>")
            .finish()
    }
}
