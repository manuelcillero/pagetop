use super::ModuleStaticRef;
use crate::core::hook::add_action;
use crate::{app, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::{db::*, run_now};

use std::sync::RwLock;

// DISABLED MODULES ********************************************************************************

static DISABLED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn disable_modules(modules: Vec<ModuleStaticRef>) {
    let mut disabled_modules = DISABLED_MODULES.write().unwrap();
    for module in modules {
        if !disabled_modules
            .iter()
            .any(|m| m.handler() == module.handler())
        {
            trace::debug!("Disabling the \"{}\" module", module.single_name());
            disabled_modules.push(module);
        }
    }
}

// ENABLED MODULES *********************************************************************************

static ENABLED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn enable_modules(modules: Vec<ModuleStaticRef>) {
    for module in modules {
        let mut list: Vec<ModuleStaticRef> = Vec::new();
        add_to_enabled(&mut list, module);
        list.reverse();
        ENABLED_MODULES.write().unwrap().append(&mut list);
    }
}

fn add_to_enabled(list: &mut Vec<ModuleStaticRef>, module: ModuleStaticRef) {
    if !ENABLED_MODULES
        .read()
        .unwrap()
        .iter()
        .any(|m| m.handler() == module.handler())
        && !list.iter().any(|m| m.handler() == module.handler())
    {
        if DISABLED_MODULES
            .read()
            .unwrap()
            .iter()
            .any(|m| m.handler() == module.handler())
        {
            panic!(
                "Trying to enable \"{}\" module which is disabled",
                module.single_name()
            );
        } else {
            trace::debug!("Enabling the \"{}\" module", module.single_name());
            list.push(module);

            let mut dependencies = module.dependencies();
            dependencies.reverse();
            for d in dependencies.iter() {
                add_to_enabled(list, *d);
            }
        }
    }
}

// CONFIGURE MODULES *******************************************************************************

pub fn modules(cfg: &mut app::web::ServiceConfig) {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        m.configure_service(cfg);
    }
}

pub fn register_actions() {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        for a in m.actions().into_iter() {
            add_action(a);
        }
    }
}

#[cfg(feature = "database")]
pub fn run_migrations() {
    run_now({
        struct Migrator;
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<MigrationItem> {
                let mut migrations = vec![];
                for m in DISABLED_MODULES.read().unwrap().iter() {
                    migrations.append(&mut m.migrations());
                }
                migrations
            }
        }
        Migrator::down(&app::db::DBCONN, None)
    })
    .unwrap();

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
    })
    .unwrap();
}
