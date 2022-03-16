use crate::{Lazy, trace};
use crate::core::theme::ThemeTrait;
use crate::core::module::ModuleTrait;
use crate::core::server;

use std::sync::RwLock;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

// -----------------------------------------------------------------------------
// Temas registrados y tema predeterminado.
// -----------------------------------------------------------------------------

pub static THEMES: Lazy<RwLock<Vec<&dyn ThemeTrait>>> = Lazy::new(
    || { RwLock::new(Vec::new()) }
);

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

pub static MODULES: Lazy<RwLock<Vec<&dyn ModuleTrait>>> = Lazy::new(
    || { RwLock::new(Vec::new()) }
);

pub fn modules(cfg: &mut server::web::ServiceConfig) {
    for m in MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub fn migrations() {
    trace::info!("Checking migrations");
    for m in MODULES.read().unwrap().iter() {
        m.migrations(&*server::db::DBCONN).expect("Failed to run migrations");
    }
}
