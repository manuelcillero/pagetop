use crate::{Lazy, app, base, theme_static_files, trace};
use super::ThemeTrait;

use std::sync::RwLock;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// Temas registrados.
static THEMES: Lazy<RwLock<Vec<&dyn ThemeTrait>>> = Lazy::new(|| {
    RwLock::new(vec![
        &base::theme::aliner::AlinerTheme,
        &base::theme::minimal::MinimalTheme,
        &base::theme::bootsier::BootsierTheme,
    ])
});

pub fn register_theme(theme: &'static dyn ThemeTrait) {
    let mut themes = THEMES.write().unwrap();
    match themes.iter().find(|t| t.name() == theme.name()) {
        None => {
            trace::info!("{}", theme.name());
            themes.push(theme);
        },
        Some(_) => {},
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
