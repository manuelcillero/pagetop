use super::ThemeStaticRef;
use crate::{trace, LazyStatic};

use std::sync::RwLock;

// Temas registrados.
static THEMES: LazyStatic<RwLock<Vec<ThemeStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn register_theme(theme: Option<ThemeStaticRef>) {
    if let Some(theme) = theme {
        let handle = theme.handle();
        let mut registered_themes = THEMES.write().unwrap();
        if !registered_themes.iter().any(|t| t.handle() == handle) {
            trace::debug!("Registering theme \"{}\"", theme.single_name());
            registered_themes.push(theme);
        }
    }
}

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
