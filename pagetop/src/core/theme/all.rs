use super::ThemeStaticRef;
use crate::{app, base, configure_service_for_static_files, trace, LazyStatic};

use std::sync::RwLock;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// Temas registrados.
static THEMES: LazyStatic<RwLock<Vec<ThemeStaticRef>>> = LazyStatic::new(|| {
    RwLock::new(vec![
        &base::theme::aliner::Aliner,
        &base::theme::minimal::Minimal,
        &base::theme::bootsier::Bootsier,
        &base::theme::bulmix::Bulmix,
    ])
});

pub fn register_themes(themes: Vec<ThemeStaticRef>) {
    let mut registered_themes = THEMES.write().unwrap();
    for theme in themes {
        if !registered_themes
            .iter()
            .any(|t| t.handle() == theme.handle())
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

pub fn configure_services(cfg: &mut app::web::ServiceConfig) {
    configure_service_for_static_files!(cfg, "/theme", bundle_theme);

    for t in THEMES.read().unwrap().iter() {
        t.configure_service(cfg);
    }
}
