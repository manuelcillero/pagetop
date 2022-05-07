use crate::{Lazy, app, run_now, trace};
use crate::api::action::add_action;
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
    if !MODULES.read().unwrap().iter().any(|m| m.handler() == module.handler()) {
        if !list.iter().any(|m| m.handler() == module.handler()) {
            trace::debug!("Register module: \"{}\"", module.name());
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
        m.configure_service(cfg);
    }
}

pub fn register_actions() {
    for m in MODULES.read().unwrap().iter() {
        for a in m.actions().into_iter() {
            add_action(a);
        }
    }
}

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub fn run_migrations() {
    run_now({
        struct Migrator;
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<MigrationItem> {
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
