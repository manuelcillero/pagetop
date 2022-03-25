use crate::{Lazy, app, theme_static_files, trace};
use super::ThemeTrait;

use std::sync::RwLock;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// Temas registrados.
static THEMES: Lazy<RwLock<Vec<&dyn ThemeTrait>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

pub fn register_theme(theme: &'static dyn ThemeTrait) {
    let mut themes = THEMES.write().unwrap();
    if !themes.iter().any(|t| t.name() == theme.name()) {
        trace::debug!("Registering \"{}\" theme", theme.name());
        themes.push(theme);
    }
}

pub fn theme_by_name(name: &str) -> Option<&'static dyn ThemeTrait> {
    match THEMES.write().unwrap().iter().find(
        |t| t.name().to_lowercase() == name.to_lowercase()
    ) {
        Some(theme) => Some(*theme),
        _ => None,
    }
}

pub fn themes(cfg: &mut app::web::ServiceConfig) {
    theme_static_files!(cfg, "/theme");

    for t in THEMES.read().unwrap().iter() {
        t.configure_theme(cfg);
    }
}
