use crate::{Lazy, app, trace};
use crate::core::hook::add_hook;
use super::ModuleTrait;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
use crate::{
    db::*,
    run_now,
};

use std::sync::RwLock;

// Enabled modules.
static ENABLED_MODULES: Lazy<RwLock<Vec<&dyn ModuleTrait>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

/* Disabled modules.
static DISABLED_MODULES: Lazy<RwLock<Vec<&dyn ModuleTrait>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
}); */

pub fn enable_modules(modules: Vec<&'static dyn ModuleTrait>) {
    for m in modules {
        enable(m)
    }
}

fn enable(module: &'static dyn ModuleTrait) {
    let mut list: Vec<&dyn ModuleTrait> = Vec::new();
    add_to(&mut list, module);
    list.reverse();
    ENABLED_MODULES.write().unwrap().append(&mut list);
}

fn add_to(list: &mut Vec<&dyn ModuleTrait>, module: &'static dyn ModuleTrait) {
    if !ENABLED_MODULES.read().unwrap().iter().any(|m| m.handler() == module.handler()) {
        if !list.iter().any(|m| m.handler() == module.handler()) {
            trace::debug!("Enabling module \"{}\"", module.single_name());
            list.push(module);

            let mut dependencies = module.dependencies();
            dependencies.reverse();
            for d in dependencies.iter() {
                add_to(list, *d);
            }
        }
    }
}
/*
#[allow(unused_variables)]
pub fn disable_module(module: &'static dyn ModuleTrait) {
}
*/
pub fn modules(cfg: &mut app::web::ServiceConfig) {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        m.configure_service(cfg);
    }
}

pub fn register_hooks() {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        for a in m.actions().into_iter() {
            add_hook(a);
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
                for m in ENABLED_MODULES.read().unwrap().iter() {
                    migrations.append(&mut m.migrations());
                }
                migrations
            }
        }
        Migrator::up(&app::db::DBCONN, None)
    }).unwrap();
}
