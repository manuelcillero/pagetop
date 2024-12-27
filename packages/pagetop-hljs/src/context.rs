use pagetop::prelude::*;

use crate::config;
use crate::lang::HljsLang;
use crate::mode::HljsMode;
use crate::theme::HljsTheme;

use std::collections::HashSet;

// Context parameters.
const PARAM_HLJS_ENABLED: &str = "hljs.enabled";
const PARAM_HLJS_MODE: &str = "hljs.mode";
const PARAM_HLJS_LANGS: &str = "hljs.langs";
const PARAM_HLJS_THEME: &str = "hljs.theme";

/// Extend Context with HighlightJS features.
pub trait HljsContext {
    /// Enable syntax highlighting in current context.
    fn enable_hljs(&mut self);

    /// Preventing syntax highlighting in current context.
    fn disable_hljs(&mut self);

    /// Force the use of the *highlight.js* ***core*** or ***common*** mode in current context,
    /// ignoring the [`config::SETTINGS.hljs.mode`](crate::config::Hljs#structfield.mode)
    /// configuration setting.
    fn force_hljs_mode(&mut self, mode: &HljsMode);

    /// Add a new language to the context for processing code snippets. It is necessary to add at
    /// least one language to load the *highlight.js* library. Each
    /// [`Snippet`](crate::snippet::Snippet) component automatically adds its required language.
    fn add_hljs_language(&mut self, language: &HljsLang);

    /// Change the theme in current context for displaying code snippets. The same theme is used for
    /// all snippets in the given context.
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
