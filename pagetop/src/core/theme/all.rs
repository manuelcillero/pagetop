use crate::{Lazy, theme_static_files, trace};
use crate::core::app;
use super::ThemeTrait;

use std::sync::RwLock;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// Temas registrados.
static THEMES: Lazy<RwLock<Vec<&dyn ThemeTrait>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

pub fn register_themes(themes: Vec<&'static dyn ThemeTrait>) {
    for t in themes {
        register_theme(t)
    }
}

pub fn register_theme(theme: &'static dyn ThemeTrait) {
    let mut themes = THEMES.write().unwrap();
    if !themes.iter().any(|t| t.handler() == theme.handler()) {
        trace::debug!("Registering theme \"{}\"", theme.single_name());
        themes.push(theme);
    }
}

pub fn theme_by_single_name(single_name: &str) -> Option<&'static dyn ThemeTrait> {
    match THEMES.write().unwrap().iter().find(
        |t| t.single_name().to_lowercase() == single_name.to_lowercase()
    ) {
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
