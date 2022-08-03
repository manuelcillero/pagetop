use super::ModuleStaticRef;
use crate::core::hook::add_action;
use crate::core::theme;
use crate::{app, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::{db::*, run_now};

use std::sync::RwLock;

// REGISTER MODULES ********************************************************************************

static ENABLED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

static DISCARDED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn register_modules(app: ModuleStaticRef) {
    // Revisa los módulos a deshabilitar.
    let mut list: Vec<ModuleStaticRef> = Vec::new();
    add_to_discarded(&mut list, app);
    DISCARDED_MODULES.write().unwrap().append(&mut list);

    // Habilita los módulos de la aplicación.
    let mut list: Vec<ModuleStaticRef> = Vec::new();
    add_to_enabled(&mut list, app);
    list.reverse();
    ENABLED_MODULES.write().unwrap().append(&mut list);
}

fn add_to_discarded(list: &mut Vec<ModuleStaticRef>, module: ModuleStaticRef) {
    for u in module.uninstall_modules().iter() {
        if !list.iter().any(|m| m.handler() == u.handler()) {
            list.push(*u);
            trace::debug!("Module \"{}\" discarded", u.single_name());
        }
    }
    for d in module.dependencies().iter() {
        add_to_discarded(list, *d);
    }
}

fn add_to_enabled(list: &mut Vec<ModuleStaticRef>, module: ModuleStaticRef) {
    if !list.iter().any(|m| m.handler() == module.handler()) {
        if DISCARDED_MODULES
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
            list.push(module);

            let mut dependencies = module.dependencies();
            dependencies.reverse();
            for d in dependencies.iter() {
                add_to_enabled(list, *d);
            }

            trace::debug!("Enabling \"{}\" module", module.single_name());
        }
    }
}

// REGISTER THEMES *********************************************************************************

pub fn register_themes() {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        theme::all::register_themes(m.themes());
    }
}

// REGISTER ACTIONS ********************************************************************************

pub fn register_actions() {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        for a in m.actions().into_iter() {
            add_action(a);
        }
    }
}

// RUN MIGRATIONS **********************************************************************************

#[cfg(feature = "database")]
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
    })
    .unwrap();

    run_now({
        struct Migrator;
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<MigrationItem> {
                let mut migrations = vec![];
                for m in DISCARDED_MODULES.read().unwrap().iter() {
                    migrations.append(&mut m.migrations());
                }
                migrations
            }
        }
        Migrator::down(&app::db::DBCONN, None)
    })
    .unwrap();
}

// INIT MODULES ************************************************************************************

pub fn init_modules() {
    trace::info!("Calling application bootstrap");
    for m in ENABLED_MODULES.read().unwrap().iter() {
        m.init();
    }
}

// CONFIGURE SERVICES ******************************************************************************

pub fn configure_services(cfg: &mut app::web::ServiceConfig) {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        m.configure_service(cfg);
    }
}
