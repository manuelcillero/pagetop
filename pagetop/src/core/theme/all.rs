use crate::config;
use crate::core::theme::ThemeRef;
use crate::LazyStatic;

use std::sync::RwLock;

// THEMES ******************************************************************************************

pub static THEMES: LazyStatic<RwLock<Vec<ThemeRef>>> = LazyStatic::new(|| RwLock::new(Vec::new()));

// DEFAULT THEME ***********************************************************************************

pub static THEME: LazyStatic<ThemeRef> =
    LazyStatic::new(|| match theme_by_single_name(&config::SETTINGS.app.theme) {
        Some(theme) => theme,
        None => &crate::base::theme::InceptionTheme,
    });

// THEME BY NAME ***********************************************************************************

pub fn theme_by_single_name(single_name: &str) -> Option<ThemeRef> {
    let single_name = single_name.to_lowercase();
    match THEMES
        .read()
        .unwrap()
        .iter()
        .find(|t| t.single_name().to_lowercase() == single_name)
    {
        Some(theme) => Some(*theme),
        _ => None,
    }
}
