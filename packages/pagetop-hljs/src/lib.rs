//! <div align="center">
//!
//! <h1>PageTop HighlightJS</h1>
//!
//! <p>Integra <a href="https://highlightjs.org">highlight.js</a> para mostrar fragmentos de código con resaltado de sintaxis con <strong>PageTop</strong>.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
//! [![Doc API](https://img.shields.io/docsrs/pagetop-hljs?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-hljs)
//! [![Crates.io](https://img.shields.io/crates/v/pagetop-hljs.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-hljs)
//! [![Descargas](https://img.shields.io/crates/d/pagetop-hljs.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-hljs)
//!
//! </div>
//!
//! ## Uso
//!
//! Añade `pagetop-hljs` a tu archivo `Cargo.toml`:
//!
//! ```rust
//! [dependencies]
//! pagetop-hljs = "<Version>"
//! ```
//!
//! Incluye `pagetop_hljs::HighlightJS` en las dependencias del paquete o aplicación que lo requiera:
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! impl PackageTrait for MyPackage {
//!     // ...
//!     fn dependencies(&self) -> Vec<PackageRef> {
//!         vec![
//!             // ...
//!             &pagetop_hljs::HighlightJS,
//!             // ...
//!         ]
//!     }
//!
//!     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
//!         cfg.route("/snippet", service::web::get().to(hljs_sample));
//!     }
//!     // ...
//! }
//! ```
//!
//! Y finalmente añade tus fragmentos de código con resaltado de sintaxis en páginas web:
//!
//! ```rust
//! use pagetop_hljs::prelude::*;
//!
//! async fn hljs_sample(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
//!     Page::new(request)
//!         .with_component(Snippet::with(
//!             HljsLang::Rust,
//!             r###"
//! // This is the main function.
//! fn main() {
//!     // Print text to the console.
//!     println!("Hello World!");
//! }
//!             "###,
//!         ))
//!         .render()
//! }
//! ```

#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/favicon.ico"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/pagetop_hljs.png"
)]

use pagetop::prelude::*;

// API *********************************************************************************************

pub mod config;

pub mod hljs_context;
pub mod hljs_lang;
pub mod hljs_mode;
pub mod hljs_theme;
pub mod snippet;

// PRELUDE *****************************************************************************************

pub mod prelude {
    pub use crate::hljs_context::HljsContext;
    pub use crate::hljs_lang::HljsLang;
    pub use crate::hljs_mode::HljsMode;
    pub use crate::hljs_theme::HljsTheme;
    pub use crate::snippet::Snippet;
}

include_files!(hljs);

include_locales!(LOCALES_HLJS);

/// Implementa [`PackageTrait`].
pub struct HighlightJS;

impl PackageTrait for HighlightJS {
    fn description(&self) -> L10n {
        L10n::t("hljs_description", &LOCALES_HLJS)
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![action::page::AfterRenderBody::new(after_render_body)]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        include_files_service!(cfg, hljs => "/hljs");
    }
}

// Versión de la librería Highlight.js.
const HLJS_VERSION: &str = "11.7.0";

// Define los recursos para la página según se use highlight.js en su versión "core" o "common".
fn after_render_body(page: &mut Page) {
    use hljs_context::HljsContext;
    use hljs_lang::HljsLang;
    use hljs_mode::HljsMode;
    use hljs_theme::HljsTheme;

    let cx = page.context();

    if cx.is_hljs_enabled() {
        if let Some(languages) = cx.hljs_languages() {
            match cx.hljs_mode() {
                HljsMode::Core => {
                    cx.alter_assets(AssetsOp::AddJavaScript(
                        JavaScript::from("/hljs/js/core.min.js").with_version(HLJS_VERSION),
                    ));
                    for l in languages {
                        cx.alter_assets(AssetsOp::AddJavaScript(
                            JavaScript::from(HljsLang::to_url(l)).with_version(HLJS_VERSION),
                        ));
                    }
                }
                _ => {
                    cx.alter_assets(AssetsOp::AddJavaScript(
                        JavaScript::from("/hljs/js/highlight.min.js").with_version(HLJS_VERSION),
                    ));
                }
            }

            // Configura highlight.js (deshabilitando autodetección del lenguaje).
            #[rustfmt::skip]
            cx.alter_assets(AssetsOp::AddJavaScript(
                JavaScript::inline("highlight.js", join_string!("
                    hljs.configure({
                        tabReplace: '", " ".repeat(config::SETTINGS.hljs.tabsize), "',
                        languages: [],
                    });
                    hljs.highlightAll();
                ")),
            ));

            cx.alter_assets(AssetsOp::AddStyleSheet(
                StyleSheet::from(HljsTheme::to_url(cx.hljs_theme().to_string()))
                    .with_version(HLJS_VERSION),
            ));
        }
    }
}
