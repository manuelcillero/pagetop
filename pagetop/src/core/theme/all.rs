use super::ThemeStaticRef;
use crate::{app, theme_static_files, trace, LazyStatic};

use std::sync::RwLock;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// Temas registrados.
static THEMES: LazyStatic<RwLock<Vec<ThemeStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn register_themes(themes: Vec<ThemeStaticRef>) {
    let mut registered_themes = THEMES.write().unwrap();
    for theme in themes {
        if !registered_themes
            .iter()
            .any(|t| t.handler() == theme.handler())
        {
            trace::debug!("Registering theme \"{}\"", theme.single_name());
            registered_themes.push(theme);
        }
    }
}

pub fn theme_by_single_name(single_name: &str) -> Option<ThemeStaticRef> {
    match THEMES
        .write()
        .unwrap()
        .iter()
        .find(|t| t.single_name().to_lowercase() == single_name.to_lowercase())
    {
        Some(theme) => Some(*theme),
        _ => None,
    }
}

pub fn themes(cfg: &mut app::web::ServiceConfig) {
    theme_static_files!(cfg, "/theme");

    for t in THEMES.read().unwrap().iter() {
        t.configure_service(cfg);
    }
}
