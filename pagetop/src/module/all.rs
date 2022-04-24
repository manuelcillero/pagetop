use crate::{Lazy, app, run_now, trace};
use crate::db::*;
use super::ModuleTrait;

use std::sync::RwLock;

// Módulos registrados.
static MODULES: Lazy<RwLock<Vec<&dyn ModuleTrait>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

pub fn register_module(module: &'static dyn ModuleTrait) {
    let mut list: Vec<&dyn ModuleTrait> = Vec::new();
    add_to(&mut list, module);
    list.reverse();
    MODULES.write().unwrap().append(&mut list);
}

fn add_to(list: &mut Vec<&dyn ModuleTrait>, module: &'static dyn ModuleTrait) {
    if !MODULES.read().unwrap().iter().any(|m| m.name() == module.name()) {
        if !list.iter().any(|m| m.name() == module.name()) {
            trace::debug!("Registering \"{}\" module", module.name());
            list.push(module);

            let mut dependencies = module.dependencies();
            dependencies.reverse();
            for d in dependencies.iter() {
                add_to(list, *d);
            }
        }
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
