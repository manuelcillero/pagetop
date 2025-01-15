use pagetop::prelude::*;

use crate::config;
use crate::lang::HljsLang;
use crate::mode::HljsMode;
use crate::theme::HljsTheme;

use std::collections::HashSet;

// Parámetros para el contexto.
const PARAM_HLJS_ENABLED: &str = "hljs.enabled";
const PARAM_HLJS_MODE: &str = "hljs.mode";
const PARAM_HLJS_LANGS: &str = "hljs.langs";
const PARAM_HLJS_THEME: &str = "hljs.theme";

/// Extiende el contexto de renderizado con funcionalidades de HighlightJS.
pub trait HljsContext {
    /// Habilita el resaltado de sintaxis en el contexto actual.
    fn enable_hljs(&mut self);

    /// Deshabilita el resaltado de sintaxis en el contexto actual.
    fn disable_hljs(&mut self);

    /// Fuerza el uso del modo ***core*** o ***common*** de *highlight.js* en el contexto actual,
    /// ignorando [`config::SETTINGS.hljs.mode`](crate::config::Hljs#structfield.mode) de las
    /// opciones de configuración.
    fn force_hljs_mode(&mut self, mode: &HljsMode);

    /// Añade un nuevo lenguaje al contexto actual para el resaltado de fragmentos de código. Se
    /// requiere al menos un lenguaje para cargar la librería *highlight.js*. Recuerda que cada
    /// componente [`Snippet`](crate::snippet::HljsSnippet) añade automáticamente el lenguaje que
    /// necesita. Solo aplica cuando el contexto está configurado en el modo ***core***.
    fn add_hljs_language(&mut self, language: &HljsLang);

    /// Cambia el tema del contexto actual para mostrar los fragmentos de código. Ten en cuenta que
    /// *highlight.js* utilizará el mismo tema para todos los framentos en este contexto.
    fn set_hljs_theme(&mut self, theme: &HljsTheme);

    fn is_hljs_enabled(&self) -> bool;

    fn hljs_mode(&self) -> HljsMode;

    fn hljs_languages(&self) -> Option<HashSet<String>>;

    fn hljs_theme(&self) -> HljsTheme;
}

impl HljsContext for Context {
    fn enable_hljs(&mut self) {
        self.alter_param::<bool>(PARAM_HLJS_ENABLED, &true);
    }

    fn disable_hljs(&mut self) {
        self.alter_param::<bool>(PARAM_HLJS_ENABLED, &false);
    }

    fn force_hljs_mode(&mut self, mode: &HljsMode) {
        self.alter_param::<HljsMode>(PARAM_HLJS_MODE, mode);
    }

    fn add_hljs_language(&mut self, language: &HljsLang) {
        let languages = match self.get_param::<String>(PARAM_HLJS_LANGS) {
            Ok(previous) => join_string!(previous, ";", language.to_string()),
            _ => language.to_string(),
        };
        self.alter_param::<String>(PARAM_HLJS_LANGS, &languages);
    }

    fn set_hljs_theme(&mut self, theme: &HljsTheme) {
        self.alter_param::<String>(PARAM_HLJS_THEME, &theme.to_string());
    }

    // HljsContext GETTERS.

    fn is_hljs_enabled(&self) -> bool {
        self.get_param::<bool>(PARAM_HLJS_ENABLED).unwrap_or(true)
    }

    fn hljs_mode(&self) -> HljsMode {
        self.get_param::<HljsMode>(PARAM_HLJS_MODE)
            .unwrap_or(config::SETTINGS.hljs.mode)
    }

    fn hljs_languages(&self) -> Option<HashSet<String>> {
        if let Ok(languages) = self.get_param::<String>(PARAM_HLJS_LANGS) {
            let set: HashSet<String> = languages.split(';').map(|s| s.to_string()).collect();
            return Some(set);
        }
        None
    }

    fn hljs_theme(&self) -> HljsTheme {
        self.get_param::<HljsTheme>(PARAM_HLJS_THEME)
            .unwrap_or(config::SETTINGS.hljs.theme)
    }
}
