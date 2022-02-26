use crate::Lazy;
use crate::config::SETTINGS;
use crate::core::theme::Theme;
use crate::core::module::Module;
use crate::core::response::page::PageContainer;
use crate::core::server;
use crate::base;

use std::sync::RwLock;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// -----------------------------------------------------------------------------
// Temas registrados y tema por defecto.
// -----------------------------------------------------------------------------

pub static THEMES: Lazy<RwLock<Vec<&dyn Theme>>> = Lazy::new(|| {
    RwLock::new(vec![
        &base::theme::aliner::AlinerTheme,
        &base::theme::minimal::MinimalTheme,
        &base::theme::bootsier::BootsierTheme,
    ])
});

pub static DEFAULT_THEME: Lazy<&dyn Theme> = Lazy::new(|| {
    for t in THEMES.read().unwrap().iter() {
        if t.name().to_lowercase() == SETTINGS.app.theme.to_lowercase() {
            return *t;
        }
    }
    &base::theme::bootsier::BootsierTheme
});

pub fn themes(cfg: &mut server::web::ServiceConfig) {
    cfg.service(actix_web_static_files::ResourceFiles::new(
        "/theme",
        assets()
    ));

    for t in THEMES.read().unwrap().iter() {
        t.configure_theme(cfg);
    }
}

// -----------------------------------------------------------------------------
// Módulos registrados.
// -----------------------------------------------------------------------------

pub static MODULES: Lazy<RwLock<Vec<&dyn Module>>> = Lazy::new(|| {
    RwLock::new(vec![
        &base::module::admin::AdminModule,
        &base::module::user::UserModule,
    ])
});

pub fn modules(cfg: &mut server::web::ServiceConfig) {
    for m in MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}

// -----------------------------------------------------------------------------
// Componentes globales.
// -----------------------------------------------------------------------------

pub static COMPONENTS: Lazy<RwLock<HashMap<&str, PageContainer>>> = Lazy::new(
    || { RwLock::new(HashMap::new()) }
);
