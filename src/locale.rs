//! Localización (L10n).
//!
//! PageTop utiliza las especificaciones de [Fluent](https://www.projectfluent.org/) para la
//! localización de aplicaciones, y aprovecha [fluent-templates](https://docs.rs/fluent-templates/)
//! para integrar los recursos de traducción directamente en el binario de la aplicación.
//!
//! # Sintaxis Fluent (FTL)
//!
//! El formato empleado para describir los recursos de traducción se denomina
//! [FTL](https://www.projectfluent.org/fluent/guide/). Está diseñado para ser legible y expresivo,
//! permitiendo representar construcciones complejas del lenguaje natural como el género, el plural
//! o las conjugaciones verbales.
//!
//! # Recursos Fluent
//!
//! Por defecto, las traducciones están en el directorio `src/locale`, con subdirectorios para cada
//! [Identificador de Idioma Unicode](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier)
//! válido. Podríamos tener una estructura como esta:
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
//!      └── fr-FR/
//!          ├── default.ftl
//!          └── main.ftl
//! ```
//!
//! Ejemplo de un archivo en `src/locale/en-US/main.ftl`:
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
//! Y su archivo equivalente para español en `src/locale/es-ES/main.ftl`:
//!
//! ```text
//! hello-world = ¡Hola, mundo!
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
//!
//! # Cómo aplicar la localización en tu código
//!
//! Una vez creado el directorio con los recursos FTL, basta con usar la macro
//! [`include_locales!`](crate::include_locales) para integrarlos en la aplicación.
//!
//! Si los recursos se encuentran en el directorio por defecto `src/locale` del *crate*, sólo hay
//! que declarar:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! include_locales!(LOCALES_SAMPLE);
//! ```
//!
//! Si están ubicados en otro directorio, se puede usar la forma:
//!
//! ```rust,ignore
//! include_locales!(LOCALES_SAMPLE from "ruta/a/las/traducciones");
//! ```
//!
//! Y *voilà*, sólo queda operar con los idiomas soportados por PageTop usando [`Locale`] y traducir
//! textos con [`L10n`].

pub use fluent_templates;
pub use unic_langid::{CharacterDirection, LanguageIdentifier};

use unic_langid::langid;

mod languages;

mod definition;
pub use definition::{LangId, Locale};

mod request;
pub use request::RequestLocale;

mod l10n;
pub use l10n::L10n;

/// Incluye un conjunto de recursos **Fluent** con textos de traducción propios.
///
/// Esta macro integra en el binario de la aplicación los archivos FTL ubicados en los siguientes
/// directorios opcionales de recursos Fluent:
///
/// - `$dir_locales`, con los subdirectorios de cada idioma. Por ejemplo, `"files/ftl"` o
///   `"assets/translations"`. Si no se indica, se usará el directorio por defecto `"src/locale"`.
/// - `$core_locales`, que añade un conjunto de traducciones que se cargan para **todos** los
///   idiomas. Sirve para definir textos comunes que no tienen por qué duplicarse en cada
///   subdirectorio de idioma.
///
/// Cada extensión o tema puede definir sus propios recursos de traducción usando esta macro. Para
/// más detalles sobre el sistema de localización consulta el módulo [`locale`](crate::locale).
///
/// # Ejemplos
///
/// Uso básico con el directorio por defecto `"src/locale"`:
///
/// ```rust
/// # use pagetop::prelude::*;
/// include_locales!(LOCALES_SAMPLE);
/// ```
///
/// Uso indicando recursos comunes (además de `"src/locale"`):
///
/// ```rust,ignore
/// include_locales!(LOCALES_SAMPLE, "src/core-locale");
/// ```
///
/// Uso con un directorio de recursos Fluent alternativo:
///
/// ```rust,ignore
/// include_locales!(LOCALES_SAMPLE from "ruta/a/las/traducciones");
/// ```
#[macro_export]
macro_rules! include_locales {
    // Se desactiva la inserción de marcas de aislamiento Unicode (FSI/PDI) en los argumentos para
    // mejorar la legibilidad y la compatibilidad en ciertos contextos de renderizado.
    ( $LOCALES:ident $(, $core_locales:literal)? ) => {
        $crate::locale::fluent_templates::static_loader! {
            static $LOCALES = {
                locales: "src/locale",
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",
                // Elimina marcas de aislamiento Unicode en los argumentos.
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
                // Elimina marcas de aislamiento Unicode en los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
}
