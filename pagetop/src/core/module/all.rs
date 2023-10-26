use crate::core::action::add_action;
use crate::core::module::ModuleRef;
use crate::core::theme::all::THEMES;
use crate::{service, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::db::*;

use std::sync::RwLock;

// MODULES *****************************************************************************************

static ENABLED_MODULES: LazyStatic<RwLock<Vec<ModuleRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

static DROPPED_MODULES: LazyStatic<RwLock<Vec<ModuleRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

// REGISTER MODULES ********************************************************************************

pub fn register_modules(app: ModuleRef) {
    // List of modules to drop.
    let mut list: Vec<ModuleRef> = Vec::new();
    add_to_dropped(&mut list, app);
    DROPPED_MODULES.write().unwrap().append(&mut list);

    // List of modules to enable.
    let mut list: Vec<ModuleRef> = Vec::new();

    // Enable default theme.
    add_to_enabled(&mut list, &crate::base::theme::Inception);

    // Enable application modules.
    add_to_enabled(&mut list, app);

    list.reverse();
    ENABLED_MODULES.write().unwrap().append(&mut list);
}

fn add_to_dropped(list: &mut Vec<ModuleRef>, module: ModuleRef) {
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

fn add_to_enabled(list: &mut Vec<ModuleRef>, module: ModuleRef) {
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
    if let Some(dbconn) = &*DBCONN {
        use crate::locale::L10n;

        match run_now({
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
            Migrator::up(SchemaManagerConnection::Connection(dbconn), None)
        }) {
            Err(e) => {
                L10n::l("db_migration_fail")
                    .with_arg("dberr", format!("{}", e))
                    .error();
            }
            _ => {}
        };

        match run_now({
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
            Migrator::down(SchemaManagerConnection::Connection(dbconn), None)
        }) {
            Err(e) => {
                L10n::l("db_migration_fail")
                    .with_arg("dberr", format!("{}", e))
                    .error();
            }
            _ => {}
        };
    }
}

// CONFIGURE SERVICES ******************************************************************************

pub fn configure_services(scfg: &mut service::web::ServiceConfig) {
    for m in ENABLED_MODULES.read().unwrap().iter() {
        m.configure_service(scfg);
    }
}
