use super::{ModuleStaticRef, ThemeStaticRef};

use crate::base;
use crate::core::hook::add_action;
use crate::{server, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::{db::*, run_now};

use std::sync::RwLock;

// MODULES *****************************************************************************************

static ENABLED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

static DROPPED_MODULES: LazyStatic<RwLock<Vec<ModuleStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

// THEMES ******************************************************************************************

static THEMES: LazyStatic<RwLock<Vec<ThemeStaticRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

pub fn theme_by_single_name(single_name: &str) -> Option<ThemeStaticRef> {
    let single_name = single_name.to_lowercase();
    match THEMES
        .read()
        .unwrap()
        .iter()
        .find(|t| t.single_name().to_lowercase() == single_name)
    {
        Some(theme) => Some(*theme),
        _ => None,
    }
}

// REGISTER MODULES ********************************************************************************

pub fn register_modules(app: ModuleStaticRef) {
    // List of modules to drop.
    let mut list: Vec<ModuleStaticRef> = Vec::new();
    add_to_dropped(&mut list, app);
    DROPPED_MODULES.write().unwrap().append(&mut list);

    // List of modules to enable.
    let mut list: Vec<ModuleStaticRef> = Vec::new();

    // 1 of 3. Enable base modules.
    add_to_enabled(&mut list, &base::module::menu::Menu);
    add_to_enabled(&mut list, &base::module::saturn::Saturn);

    // 2 of 3. Enable application modules.
    add_to_enabled(&mut list, app);

    // 3 of 3. Enable default homepage.
    add_to_enabled(&mut list, &base::module::homepage::DefaultHomePage);

    list.reverse();
    ENABLED_MODULES.write().unwrap().append(&mut list);
}

fn add_to_dropped(list: &mut Vec<ModuleStaticRef>, module: ModuleStaticRef) {
    for d in module.drop_modules().iter() {
        if !list.iter().any(|m| m.handle() == d.handle()) {
            list.push(*d);
            trace::debug!("Module \"{}\" dropped", d.single_name());
        }
    }
    for d in module.dependencies().iter() {
        add_to_dropped(list, *d);
    }
}

fn add_to_enabled(list: &mut Vec<ModuleStaticRef>, module: ModuleStaticRef) {
    if !list.iter().any(|m| m.handle() == module.handle()) {
        if DROPPED_MODULES
            .read()
            .unwrap()
            .iter()
            .any(|m| m.handle() == module.handle())
        {
            panic!(
                "Trying to enable \"{}\" module which is dropped",
                module.single_name()
            );
        } else {
            list.push(module);

            let mut dependencies = module.dependencies();
            dependencies.reverse();
            for d in dependencies.iter() {
                add_to_enabled(list, *d);
            }

            if let Some(theme) = module.theme() {
                let mut registered_themes = THEMES.write().unwrap();
                if !registered_themes
                    .iter()
                    .any(|t| t.handle() == theme.handle())
                {
                    registered_themes.push(theme);
                    trace::debug!("Enabling \"{}\" theme", theme.single_name());
                }
            } else {
                trace::debug!("Enabling \"{}\" module", module.single_name());
            }
        }
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
                for m in DROPPED_MODULES.read().unwrap().iter() {
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
