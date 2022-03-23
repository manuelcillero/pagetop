use crate::{Lazy, app, run_now, trace};
use crate::db::migration::*;
use super::ModuleTrait;

use std::sync::RwLock;

// Módulos registrados.
static MODULES: Lazy<RwLock<Vec<&dyn ModuleTrait>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

pub fn register_module(module: &'static dyn ModuleTrait) {
    let mut modules = MODULES.write().unwrap();
    match modules.iter().find(|m| m.name() == module.name()) {
        None => {
            trace::info!("{}", module.name());
            modules.push(module);
        },
        Some(_) => {},
    }
}

pub fn modules(cfg: &mut app::web::ServiceConfig) {
    for m in MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub fn migrations() {
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
        Migrator::up(&app::db::DBCONN, None)
    }).unwrap();
}
