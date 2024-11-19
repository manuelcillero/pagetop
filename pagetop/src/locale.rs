//! Localization (L10n).
//!
//! PageTop uses the [Fluent](https://www.projectfluent.org/) specifications for application
//! localization, leveraging the [fluent-templates](https://docs.rs/fluent-templates/) crate to
//! integrate translation resources directly into the application binary.
//!
//! # Fluent Syntax (FTL)
//!
//! The format used to describe the translation resources used by Fluent is called
//! [FTL](https://www.projectfluent.org/fluent/guide/). FTL is designed to be both readable and
//! expressive, enabling complex natural language constructs like gender, plurals, and conjugations.
//!
//! # Fluent Resources
//!
//! Localization resources are organized in the *src/locale* directory, with subdirectories for
//! each valid [Unicode Language Identifier](https://docs.rs/unic-langid/):
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
//! Example of a file *src/locale/en-US/main.ftl*:
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
//! Example of the equivalent file *src/locale/es-ES/main.ftl*:
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
//! # How to apply localization in your code
//!
//! Once you have created your FTL resource directory, use the
//! [`static_locales!`](crate::static_locales) macro to integrate them into your module or
//! application. If your resources are located in the `"src/locale"` directory, simply declare:
//!
//! ```
//! use pagetop::prelude::*;
//!
//! static_locales!(LOCALES_SAMPLE);
//! ```
//!
//! But if they are in another directory, then you can use:
//!
//! ```
//! use pagetop::prelude::*;
//!
//! static_locales!(LOCALES_SAMPLE in "path/to/locale");
//! ```

use crate::html::{Markup, PreEscaped};
use crate::{global, kv, AutoDefault};

pub use fluent_bundle::FluentValue;
pub use fluent_templates;
pub use unic_langid::LanguageIdentifier;

use fluent_templates::Loader;
use fluent_templates::StaticLoader as Locales;

use unic_langid::langid;

use std::collections::HashMap;
use std::sync::LazyLock;

use std::fmt;

const LANGUAGE_SET_FAILURE: &str = "language_set_failure";

/// A mapping between language codes (e.g., "en-US") and their corresponding [`LanguageIdentifier`]
/// and human-readable names.
static LANGUAGES: LazyLock<HashMap<String, (LanguageIdentifier, &str)>> = LazyLock::new(|| {
    kv![
        "en"    => (langid!("en-US"), "English"),
        "en-GB" => (langid!("en-GB"), "English (British)"),
        "en-US" => (langid!("en-US"), "English (United States)"),
        "es"    => (langid!("es-ES"), "Spanish"),
        "es-ES" => (langid!("es-ES"), "Spanish (Spain)"),
    ]
});

pub static LANGID_FALLBACK: LazyLock<LanguageIdentifier> = LazyLock::new(|| langid!("en-US"));

/// Sets the application's default
/// [Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier)
/// through `SETTINGS.app.language`.
pub static DEFAULT_LANGID: LazyLock<&LanguageIdentifier> =
    LazyLock::new(|| langid_for(&global::SETTINGS.app.language).unwrap_or(&LANGID_FALLBACK));

pub fn langid_for(language: impl Into<String>) -> Result<&'static LanguageIdentifier, String> {
    let language = language.into();
    if language.is_empty() {
        return Ok(&LANGID_FALLBACK);
    }
    LANGUAGES
        .get(&language)
        .map(|(langid, _)| langid)
        .ok_or_else(|| format!("No langid for Unicode Language Identifier \"{language}\"."))
}

#[macro_export]
/// Defines a set of localization elements and local translation texts, removing Unicode isolating
/// marks around arguments to improve readability and compatibility in certain rendering contexts.
macro_rules! static_locales {
    ( $LOCALES:ident $(, $core_locales:literal)? ) => {
        $crate::locale::fluent_templates::static_loader! {
            static $LOCALES = {
                locales: "src/locale",
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",
                // Removes unicode isolating marks around arguments.
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
                // Removes unicode isolating marks around arguments.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
}

static_locales!(LOCALES_PAGETOP);

#[derive(AutoDefault)]
enum L10nOp {
    #[default]
    None,
    Text(String),
    Translate(String),
}

#[derive(AutoDefault)]
pub struct L10n {
    op: L10nOp,
    locales: Option<&'static Locales>,
    args: HashMap<String, FluentValue<'static>>,
}

impl L10n {
    pub fn none() -> Self {
        L10n::default()
    }

    pub fn n(text: impl Into<String>) -> Self {
        L10n {
            op: L10nOp::Text(text.into()),
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
        self.args
            .insert(arg.into(), FluentValue::from(value.into()));
        self
    }

    pub fn using(&self, langid: &LanguageIdentifier) -> Option<String> {
        match &self.op {
            L10nOp::None => None,
            L10nOp::Text(text) => Some(text.to_owned()),
            L10nOp::Translate(key) => match self.locales {
                Some(locales) => {
                    if self.args.is_empty() {
                        locales.try_lookup(langid, key)
                    } else {
                        locales.try_lookup_with_args(langid, key, &self.args)
                    }
                }
                None => None,
            },
        }
    }

    /// Escapes the content using the default language identifier.
    pub fn markup(&self) -> Markup {
        let content = self.using(&DEFAULT_LANGID).unwrap_or_default();
        PreEscaped(content)
    }

    /// Escapes the content using the specified language identifier.
    pub fn escaped(&self, langid: &LanguageIdentifier) -> Markup {
        let content = self.using(langid).unwrap_or_default();
        PreEscaped(content)
    }
}

impl fmt::Display for L10n {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.op {
            L10nOp::None => write!(f, ""),
            L10nOp::Text(text) => write!(f, "{text}"),
            L10nOp::Translate(key) => {
                if let Some(locales) = self.locales {
                    write!(
                        f,
                        "{}",
                        if self.args.is_empty() {
                            locales.lookup(
                                match key.as_str() {
                                    LANGUAGE_SET_FAILURE => &LANGID_FALLBACK,
                                    _ => &DEFAULT_LANGID,
                                },
                                key,
                            )
                        } else {
                            locales.lookup_with_args(
                                match key.as_str() {
                                    LANGUAGE_SET_FAILURE => &LANGID_FALLBACK,
                                    _ => &DEFAULT_LANGID,
                                },
                                key,
                                &self.args,
                            )
                        }
                    )
                } else {
                    write!(f, "Unknown localization {key}")
                }
            }
        }
    }
}
