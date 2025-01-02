//! Opciones de configuración.
//!
//! Ejemplo:
//!
//! ```toml
//! [hljs]
//! mode = "core"
//! theme = "zenburn"
//! tabsize = 8
//! ```
//!
//! Uso:
//!
//! ```rust
//! use pagetop_hljs::config;
//!
//! assert_eq!(config::SETTINGS.hljs.theme, "zenburn");
//! ```
//!
//! Consulta [`pagetop::config`] para aprender cómo `PageTop` lee los archivos de opciones y aplica
//! los valores de configuración.

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
/// Opciones de configuración para la sección [`[hljs]`](Hljs) (ver [`SETTINGS`]).
pub struct Settings {
    pub hljs: Hljs,
}
#[derive(Debug, Deserialize)]
/// Sección `[hljs]` de la configuración.
///
/// Ver [`Settings`].
pub struct Hljs {
    /// Usa ***core*** para importar una librería mínima y cargar solo los lenguajes añadidos vía
    /// [`add_hljs_language()`](crate::context::HljsContext::add_hljs_language). Por otro lado, usa
    /// ***common*** para importar una librería extendida con los 40 lenguajes más habituales según
    /// [`HljsLang`](crate::lang::HljsLang). Ten en cuenta que al usar la librería *common* te
    /// limitas a los lenguajes que vienen precargados.
    /// Valor por defecto: *"core"*
    pub mode: HljsMode,
    /// Tema por defecto en formato *kebab-case* para mostrar los fragmentos de código en las
    /// páginas web (ver [`HljsTheme`]).
    /// Valor por defecto: *"default"*
    pub theme: HljsTheme,
    /// Número de espacios para el carácter tabulador.
    /// Valor por defecto: *4*
    pub tabsize: usize,
}
