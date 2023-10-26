//! Localización (L10n).
//!
//! PageTop usa el conjunto de especificaciones [Fluent](https://www.projectfluent.org/) para la
//! localización de aplicaciones.
//!
//! # Sintaxis Fluent (FTL)
//!
//! El formato utilizado para describir los recursos de traducción utilizados por Fluent se llama
//! [FTL](https://www.projectfluent.org/fluent/guide/). FTL está diseñado para ser fácil de leer,
//! pero al mismo tiempo permite representar conceptos complejos del lenguaje natural para tratar
//! género, plurales, conjugaciones, y otros.
//!
//!
//! # Recursos Fluent
//!
//! PageTop usa [fluent-templates](https://docs.rs/fluent-templates/) para integrar los recursos de
//! localización en el binario de la aplicación. El siguiente ejemplo agrupa archivos y subcarpetas
//! de *src/locale* que tienen un [Identificador de Idioma Unicode](https://docs.rs/unic-langid/)
//! válido y los asigna a su identificador correspondiente:
//!
//! ```text
//! src/locale/
//!      ├── common.ftl
//!      ├── en-US/
//!      │   ├── default.ftl
//!      │   └── main.ftl
//!      ├── es-ES/
//!      │   ├── default.ftl
//!      │   └── main.ftl
//!      ├── es-MX/
//!      │   ├── default.ftl
//!      │   └── main.ftl
//!      └── fr/
//!          ├── default.ftl
//!          └── main.ftl
//! ```
//!
//! Ejemplo de un archivo *src/locale/en-US/main.ftl*:
//!
//! ```text
//! hello-world = Hello world!
//! hello-user = Hello, {$userName}!
//! shared-photos =
//!     {$userName} {$photoCount ->
//!         [one] added a new photo
//!        *[other] added {$photoCount} new photos
//!     } of {$userGender ->
//!         [male] him and his family
//!         [female] her and her family
//!        *[other] the family
//!     }.
//! ```
//!
//! Ejemplo del archivo equivalente *src/locale/es-ES/main.ftl*:
//!
//! ```text
//! hello-world = Hola mundo!
//! hello-user = ¡Hola, {$userName}!
//! shared-photos =
//!     {$userName} {$photoCount ->
//!         [one] ha añadido una nueva foto
//!        *[other] ha añadido {$photoCount} nuevas fotos
//!     } de {$userGender ->
//!         [male] él y su familia
//!         [female] ella y su familia
//!        *[other] la familia
//!     }.
//! ```
//!
//! # Cómo aplicar la localización en tu código
//!
//! Una vez hayas creado tu directorio de recursos FTL usa la macro
//! [`static_locales!`](crate::static_locales) para integrarlos en tu módulo o aplicación. Si tus
//! recursos se encuentran en el directorio `"src/locale"` bastará con declarar:
//!
//! ```
//! use pagetop::prelude::*;
//!
//! static_locales!(LOCALES_SAMPLE);
//! ```
//!
//! Y si están en otro directorio, entonces puedes usar:
//!
//! ```
//! use pagetop::prelude::*;
//!
//! static_locales!(LOCALES_SAMPLE in "path/to/locale");
//! ```

use crate::html::{Markup, PreEscaped};
use crate::{config, kv, trace, LazyStatic, LOCALES_PAGETOP};

pub use fluent_templates;
pub use unic_langid::LanguageIdentifier;

pub(crate) use fluent_templates::Loader;
pub(crate) use fluent_templates::StaticLoader as Locales;

use unic_langid::langid;

use std::collections::HashMap;

const LANGUAGE_SET_FAILURE: &str = "language_set_failure";

static LANGUAGES: LazyStatic<HashMap<String, (LanguageIdentifier, &str)>> = LazyStatic::new(|| {
    kv![
        "en"    => (langid!("en-US"), "English"),
        "en-GB" => (langid!("en-GB"), "English (British)"),
        "en-US" => (langid!("en-US"), "English (United States)"),
        "es"    => (langid!("es-ES"), "Spanish"),
        "es-ES" => (langid!("es-ES"), "Spanish (Spain)"),
    ]
});

static FALLBACK_LANGID: LazyStatic<LanguageIdentifier> = LazyStatic::new(|| langid!("en-US"));

pub fn langid_fallback() -> &'static LanguageIdentifier {
    &FALLBACK_LANGID
}

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// global para la aplicación a partir de `SETTINGS.app.language`.
pub(crate) static LANGID: LazyStatic<&LanguageIdentifier> = LazyStatic::new(|| {
    langid_for(config::SETTINGS.app.language.as_str()).unwrap_or(langid_fallback())
});

pub fn langid_for(language: impl Into<String>) -> Result<&'static LanguageIdentifier, String> {
    let language = language.into();
    match LANGUAGES.get(language.as_str()) {
        Some((langid, _)) => Ok(langid),
        None => {
            if language.is_empty() {
                Ok(&FALLBACK_LANGID)
            } else {
                Err(L10n::l(LANGUAGE_SET_FAILURE)
                    .with_arg("language", config::SETTINGS.app.language.as_str())
                    .debug())
            }
        }
    }
}

#[macro_export]
/// Define un conjunto de elementos de localización y textos locales de traducción.
macro_rules! static_locales {
    ( $LOCALES:ident $(, $core_locales:literal)? ) => {
        $crate::locale::fluent_templates::static_loader! {
            static $LOCALES = {
                locales: "src/locale",
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",

                // Elimina las marcas Unicode que delimitan los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
    ( $LOCALES:ident in $dir_locales:literal $(, $core_locales:literal)? ) => {
        $crate::locale::fluent_templates::static_loader! {
            static $LOCALES = {
                locales: $dir_locales,
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",

                // Elimina las marcas Unicode que delimitan los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
}

#[derive(Default)]
enum L10nOp {
    #[default]
    None,
    Text(String),
    Translate(String),
}

#[derive(Default)]
pub struct L10n {
    op: L10nOp,
    locales: Option<&'static Locales>,
    args: HashMap<String, String>,
}

impl L10n {
    pub fn n(text: impl Into<String>) -> Self {
        let text = text.into();
        L10n {
            op: if text.trim().is_empty() {
                L10nOp::None
            } else {
                L10nOp::Text(text)
            },
            ..Default::default()
        }
    }

    pub fn l(key: impl Into<String>) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            locales: Some(&LOCALES_PAGETOP),
            ..Default::default()
        }
    }

    pub fn t(key: impl Into<String>, locales: &'static Locales) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            locales: Some(locales),
            ..Default::default()
        }
    }

    pub fn with_arg(mut self, arg: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.insert(arg.into(), value.into());
        self
    }

    pub fn using(&self, langid: &LanguageIdentifier) -> Option<String> {
        match &self.op {
            L10nOp::None => None,
            L10nOp::Text(text) => Some(text.to_owned()),
            L10nOp::Translate(key) => match self.locales {
                Some(locales) => locales.lookup_with_args(
                    langid,
                    key,
                    &self
                        .args
                        .iter()
                        .fold(HashMap::new(), |mut args, (key, value)| {
                            args.insert(key.to_string(), value.to_owned().into());
                            args
                        }),
                ),
                None => None,
            },
        }
    }

    pub fn escaped(&self, langid: &LanguageIdentifier) -> Markup {
        PreEscaped(self.using(langid).unwrap_or_default())
    }

    pub fn trace(&self) -> String {
        let message = self.to_string();
        trace::trace!(message);
        message
    }

    pub fn debug(&self) -> String {
        let message = self.to_string();
        trace::debug!(message);
        message
    }

    pub fn info(&self) -> String {
        let message = self.to_string();
        trace::info!(message);
        message
    }

    pub fn warn(&self) -> String {
        let message = self.to_string();
        trace::warn!(message);
        message
    }

    pub fn error(&self) -> String {
        let message = self.to_string();
        trace::error!(message);
        message
    }
}

impl ToString for L10n {
    fn to_string(&self) -> String {
        match &self.op {
            L10nOp::None => "".to_owned(),
            L10nOp::Text(text) => text.to_owned(),
            L10nOp::Translate(key) => match self.locales {
                Some(locales) => locales
                    .lookup_with_args(
                        match key.as_str() {
                            LANGUAGE_SET_FAILURE => &FALLBACK_LANGID,
                            _ => &LANGID,
                        },
                        key,
                        &self
                            .args
                            .iter()
                            .fold(HashMap::new(), |mut args, (key, value)| {
                                args.insert(key.to_string(), value.to_owned().into());
                                args
                            }),
                    )
                    .unwrap_or(key.to_owned()),
                None => key.to_owned(),
            },
        }
    }
}
