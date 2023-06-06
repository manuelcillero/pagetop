use crate::core::theme::ThemeStaticRef;
use crate::LazyStatic;

use std::sync::RwLock;

// THEMES ******************************************************************************************

pub static THEMES: LazyStatic<RwLock<Vec<ThemeStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

// THEME BY NAME ***********************************************************************************

pub fn theme_by_single_name(single_name: &str) -> Option<ThemeStaticRef> {
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
