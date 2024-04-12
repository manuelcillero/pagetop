use crate::config;
use crate::core::theme::ThemeRef;
use crate::LazyStatic;

use std::sync::RwLock;

// THEMES ******************************************************************************************

pub static THEMES: LazyStatic<RwLock<Vec<ThemeRef>>> = LazyStatic::new(|| RwLock::new(Vec::new()));

// DEFAULT THEME ***********************************************************************************

pub static THEME_DEFAULT: LazyStatic<ThemeRef> =
    LazyStatic::new(|| match theme_by_short_name(&config::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &crate::base::theme::Inception,
    });

// THEME BY NAME ***********************************************************************************

pub fn theme_by_short_name(short_name: &str) -> Option<ThemeRef> {
    let short_name = short_name.to_lowercase();
    match THEMES
        .read()
        .unwrap()
        .iter()
        .find(|t| t.short_name().to_lowercase() == short_name)
    {
        Some(theme) => Some(*theme),
        _ => None,
    }
}
