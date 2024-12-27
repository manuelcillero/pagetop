//! Configuration settings for package.
//!
//! Example:
//!
//! ```toml
//! [hljs]
//! mode = "core"
//! theme = "zenburn"
//! tabsize = 8
//! ```
//!
//! Usage:
//!
//! ```rust
//! use pagetop_hljs::config;
//!
//! assert_eq!(config::SETTINGS.hljs.theme, "zenburn");
//! ```
//! See [`pagetop::config`] to learn how PageTop reads configuration files and uses settings.

use pagetop::prelude::*;

use crate::mode::HljsMode;
use crate::theme::HljsTheme;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [hljs]
    "hljs.mode"    => "core",
    "hljs.theme"   => "default",
    "hljs.tabsize" => 4,
]);

#[derive(Debug, Deserialize)]
/// Configuration settings for the [`[hljs]`](Hljs) section (see [`SETTINGS`] package).
pub struct Settings {
    pub hljs: Hljs,
}
#[derive(Debug, Deserialize)]
/// Section `[hljs]` of the configuration settings.
///
/// See [`Settings`].
pub struct Hljs {
    /// Use ***core*** to import a minimal library and load only the languages added via
    /// [`add_hljs_language()`](crate::hljs_context::HljsContext::add_hljs_language). Alternatively,
    /// ***common*** imports an extended library containing around 40 popular languages (see
    /// [`HljsLang`](crate::hljs_lang::HljsLang)). Note that using the *common* library restricts
    /// you to the languages that are preloaded.
    /// Default value: *"core"*
    pub mode: HljsMode,
    /// Default theme in kebab-case used to display code snippets on web pages (see [`HljsTheme`]).
    /// Default value: *"default"*
    pub theme: HljsTheme,
    /// Number of spaces for *tab* character.
    /// Default value: *4*
    pub tabsize: usize,
}
