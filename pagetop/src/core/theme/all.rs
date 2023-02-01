use super::ThemeStaticRef;
use crate::util::Handle;
use crate::{trace, LazyStatic};

use std::sync::RwLock;

// Temas registrados.
static THEMES: LazyStatic<RwLock<Vec<(Handle, ThemeStaticRef)>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn register_theme(handle: Handle, theme: Option<ThemeStaticRef>) {
    if let Some(theme) = theme {
        let mut registered_themes = THEMES.write().unwrap();
        if !registered_themes.iter().any(|t| t.0 == handle) {
            trace::debug!("Registering theme \"{}\"", theme.single_name());
            registered_themes.push((handle, theme));
        }
    }
}

pub fn theme_by_single_name(single_name: &str) -> Option<ThemeStaticRef> {
    match THEMES
        .write()
        .unwrap()
        .iter()
        .find(|t| t.1.single_name().to_lowercase() == single_name.to_lowercase())
    {
        Some((_, theme)) => Some(*theme),
        _ => None,
    }
}
