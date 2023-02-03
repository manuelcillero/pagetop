use super::ModuleStaticRef;

use crate::base::module;
use crate::core::hook::add_action;
use crate::core::theme;
use crate::{server, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::{db::*, run_now};

use std::sync::RwLock;

// REGISTER MODULES ********************************************************************************

static ENABLED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

static DISCARDED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn register_modules(app: ModuleStaticRef) {
    // List of modules to disable.
    let mut list: Vec<ModuleStaticRef> = Vec::new();
    add_to_discarded(&mut list, app);
    DISCARDED_MODULES.write().unwrap().append(&mut list);

    // List of modules to enable.
    let mut list: Vec<ModuleStaticRef> = Vec::new();

    // Enable base modules.
    add_to_enabled(&mut list, &module::menu::Menu);

    // Enable application modules.
    add_to_enabled(&mut list, app);

    // Enable default homepage.
    add_to_enabled(&mut list, &module::homepage::DefaultHomePage);
    add_to_enabled(&mut list, &crate::base::theme::Saturn);

    list.reverse();
    ENABLED_MODULES.write().unwrap().append(&mut list);
}

fn add_to_discarded(list: &mut Vec<ModuleStaticRef>, module: ModuleStaticRef) {
    for u in module.uninstall_modules().iter() {
        if !list.iter().any(|m| m.handle() == u.handle()) {
            list.push(*u);
            trace::debug!("Module \"{}\" discarded", u.single_name());
        }
    }
    for d in module.dependencies().iter() {
        add_to_discarded(list, *d);
    }
}

fn add_to_enabled(list: &mut Vec<ModuleStaticRef>, module: ModuleStaticRef) {
    if !list.iter().any(|m| m.handle() == module.handle()) {
        if DISCARDED_MODULES
            .read()
            .unwrap()
            .iter()
            .any(|m| m.handle() == module.handle())
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
        theme::all::register_theme(m.theme());
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

// INIT MODULES ************************************************************************************

pub fn init_modules() {
    trace::info!("Calling application bootstrap");
    for m in ENABLED_MODULES.read().unwrap().iter() {
        m.init();
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
        Migrator::up(&DBCONN, None)
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
        Migrator::down(&DBCONN, None)
    })
    .unwrap();
}

// CONFIGURE SERVICES ******************************************************************************

pub fn configure_services(cfg: &mut server::web::ServiceConfig) {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        m.configure_service(cfg);
    }
}
