use crate::{Lazy, run_now};
use crate::db::migration::*;
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
pub fn run_migrations() {
    run_now({
        struct Migrator;
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {
                let mut migrations = vec![];
                for m in MODULES.read().unwrap().iter() {
                    migrations.append(&mut m.migrations());
                }
                migrations
            }
        }
        Migrator::up(&server::db::DBCONN, None)
    }).unwrap();
}
