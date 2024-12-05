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
//! [`include_locales!`](crate::include_locales) macro to integrate them into your module or
//! application. If your resources are located in the `"src/locale"` directory, simply declare:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! include_locales!(LOCALES_SAMPLE);
//! ```
//!
//! But if they are in another directory, then you can use:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! include_locales!(LOCALES_SAMPLE from "path/to/locale");
//! ```

use crate::html::{Markup, PreEscaped};
use crate::{global, kv, AutoDefault};

pub use fluent_templates;
pub use unic_langid::{CharacterDirection, LanguageIdentifier};

use fluent_templates::Loader;
use fluent_templates::StaticLoader as Locales;

use unic_langid::langid;

use std::collections::HashMap;
use std::sync::LazyLock;

use std::fmt;

/// A mapping between language codes (e.g., "en-US") and their corresponding [`LanguageIdentifier`]
/// and locale key names.
static LANGUAGES: LazyLock<HashMap<String, (LanguageIdentifier, &str)>> = LazyLock::new(|| {
    kv![
        "en"    => ( langid!("en-US"), "english" ),
        "en-GB" => ( langid!("en-GB"), "english_british" ),
        "en-US" => ( langid!("en-US"), "english_united_states" ),
        "es"    => ( langid!("es-ES"), "spanish" ),
        "es-ES" => ( langid!("es-ES"), "spanish_spain" ),
    ]
});

pub static FALLBACK_LANGID: LazyLock<LanguageIdentifier> = LazyLock::new(|| langid!("en-US"));

/// Sets the application's default
/// [Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier)
/// through `SETTINGS.app.language`.
pub static DEFAULT_LANGID: LazyLock<&LanguageIdentifier> =
    LazyLock::new(|| langid_for(&global::SETTINGS.app.language).unwrap_or(&FALLBACK_LANGID));

pub enum LangError {
    EmptyLang,
    UnknownLang(String),
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LangError::EmptyLang => write!(f, "The language identifier is empty."),
            LangError::UnknownLang(lang) => write!(f, "Unknown language identifier: {lang}"),
        }
    }
}

pub fn langid_for(language: impl Into<String>) -> Result<&'static LanguageIdentifier, LangError> {
    let language = language.into();
    if language.is_empty() {
        return Err(LangError::EmptyLang);
    }
    // Attempt to match the full language code (e.g., "es-MX").
    if let Some(langid) = LANGUAGES.get(&language).map(|(langid, _)| langid) {
        return Ok(langid);
    }
    // Fallback to the base language if no sublocale is found (e.g., "es").
    if let Some((base_lang, _)) = language.split_once('-') {
        if let Some(langid) = LANGUAGES.get(base_lang).map(|(langid, _)| langid) {
            return Ok(langid);
        }
    }
    Err(LangError::UnknownLang(language))
}

#[macro_export]
/// Defines a set of localization elements and local translation texts, removing Unicode isolating
/// marks around arguments to improve readability and compatibility in certain rendering contexts.
macro_rules! include_locales {
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
    ( $LOCALES:ident from $dir_locales:literal $(, $core_locales:literal)? ) => {
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

include_locales!(LOCALES_PAGETOP);

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
    #[default(&LOCALES_PAGETOP)]
    locales: &'static Locales,
    args: HashMap<String, String>,
}

impl L10n {
    pub fn n(text: impl Into<String>) -> Self {
        L10n {
            op: L10nOp::Text(text.into()),
            ..Default::default()
        }
    }

    pub fn l(key: impl Into<String>) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            ..Default::default()
        }
    }

    pub fn t(key: impl Into<String>, locales: &'static Locales) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            locales,
            ..Default::default()
        }
    }

    pub fn with_arg(mut self, arg: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.insert(arg.into(), value.into());
        self
    }

    pub fn with_args(mut self, args: HashMap<String, String>) -> Self {
        for (k, v) in args {
            self.args.insert(k, v);
        }
        self
    }

    pub fn get(&self) -> Option<String> {
        self.using(&DEFAULT_LANGID)
    }

    pub fn using(&self, langid: &LanguageIdentifier) -> Option<String> {
        match &self.op {
            L10nOp::None => None,
            L10nOp::Text(text) => Some(text.to_owned()),
            L10nOp::Translate(key) => {
                if self.args.is_empty() {
                    self.locales.try_lookup(langid, key)
                } else {
                    self.locales.try_lookup_with_args(
                        langid,
                        key,
                        &self.args.iter().fold(HashMap::new(), |mut args, (k, v)| {
                            args.insert(k.to_string(), v.to_owned().into());
                            args
                        }),
                    )
                }
            }
        }
    }

    /// Escapes translated text using the default language identifier.
    pub fn markup(&self) -> Markup {
        PreEscaped(self.get().unwrap_or_default())
    }

    /// Escapes translated text using the specified language identifier.
    pub fn escaped(&self, langid: &LanguageIdentifier) -> Markup {
        PreEscaped(self.using(langid).unwrap_or_default())
    }
}

impl fmt::Display for L10n {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.op {
            L10nOp::None => "".to_string(),
            L10nOp::Text(text) => text.clone(),
            L10nOp::Translate(key) => self.get().unwrap_or_else(|| format!("No <{}>", key)),
        };
        write!(f, "{content}")
    }
}
