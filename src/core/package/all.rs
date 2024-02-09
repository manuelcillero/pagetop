use crate::core::action::add_action;
use crate::core::package::PackageRef;
use crate::core::theme::all::THEMES;
use crate::{config, service, service_for_static_files, static_files, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::db::*;

use std::sync::RwLock;

static_files!(base);

// PACKAGES ****************************************************************************************

static ENABLED_PACKAGES: LazyStatic<RwLock<Vec<PackageRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

static DROPPED_PACKAGES: LazyStatic<RwLock<Vec<PackageRef>>> =
    LazyStatic::new(|| RwLock::new(Vec::new()));

// REGISTER PACKAGES *******************************************************************************

pub fn register_packages(app: PackageRef) {
    // List of packages to drop.
    let mut list: Vec<PackageRef> = Vec::new();
    add_to_dropped(&mut list, app);
    DROPPED_PACKAGES.write().unwrap().append(&mut list);

    // List of packages to enable.
    let mut list: Vec<PackageRef> = Vec::new();

    // Enable default themes.
    add_to_enabled(&mut list, &crate::base::theme::Basic);
    add_to_enabled(&mut list, &crate::base::theme::Chassis);
    add_to_enabled(&mut list, &crate::base::theme::Inception);

    // Enable application packages.
    add_to_enabled(&mut list, app);

    list.reverse();
    ENABLED_PACKAGES.write().unwrap().append(&mut list);
}

fn add_to_dropped(list: &mut Vec<PackageRef>, package: PackageRef) {
    for d in package.drop_packages().iter() {
        if !list.iter().any(|p| p.handle() == d.handle()) {
            list.push(*d);
            trace::debug!("Package \"{}\" dropped", d.single_name());
        }
    }
    for d in package.dependencies().iter() {
        add_to_dropped(list, *d);
    }
}

fn add_to_enabled(list: &mut Vec<PackageRef>, package: PackageRef) {
    if !list.iter().any(|p| p.handle() == package.handle()) {
        if DROPPED_PACKAGES
            .read()
            .unwrap()
            .iter()
            .any(|p| p.handle() == package.handle())
        {
            panic!(
                "Trying to enable \"{}\" package which is dropped",
                package.single_name()
            );
        } else {
            list.push(package);

            let mut dependencies = package.dependencies();
            dependencies.reverse();
            for d in dependencies.iter() {
                add_to_enabled(list, *d);
            }

            if let Some(theme) = package.theme() {
                let mut registered_themes = THEMES.write().unwrap();
                if !registered_themes
                    .iter()
                    .any(|t| t.handle() == theme.handle())
                {
                    registered_themes.push(theme);
                    trace::debug!("Enabling \"{}\" theme", theme.single_name());
                }
            } else {
                trace::debug!("Enabling \"{}\" package", package.single_name());
            }
        }
    }
}

// REGISTER ACTIONS ********************************************************************************

pub fn register_actions() {
    for m in ENABLED_PACKAGES.read().unwrap().iter() {
        for a in m.actions().into_iter() {
            add_action(a);
        }
    }
}

// INIT PACKAGES ***********************************************************************************

pub fn init_packages() {
    trace::info!("Calling application bootstrap");
    for m in ENABLED_PACKAGES.read().unwrap().iter() {
        m.init();
    }
}

// RUN MIGRATIONS **********************************************************************************

#[cfg(feature = "database")]
pub fn run_migrations() {
    if let Some(dbconn) = &*DBCONN {
        if let Err(e) = run_now({
            struct Migrator;
            impl MigratorTrait for Migrator {
                fn migrations() -> Vec<MigrationItem> {
                    let mut migrations = vec![];
                    for m in ENABLED_PACKAGES.read().unwrap().iter() {
                        migrations.append(&mut m.migrations());
                    }
                    migrations
                }
            }
            Migrator::up(SchemaManagerConnection::Connection(dbconn), None)
        }) {
            trace::error!("Database upgrade failed ({})", e);
        };

        if let Err(e) = run_now({
            struct Migrator;
            impl MigratorTrait for Migrator {
                fn migrations() -> Vec<MigrationItem> {
                    let mut migrations = vec![];
                    for m in DROPPED_PACKAGES.read().unwrap().iter() {
                        migrations.append(&mut m.migrations());
                    }
                    migrations
                }
            }
            Migrator::down(SchemaManagerConnection::Connection(dbconn), None)
        }) {
            trace::error!("Database downgrade failed ({})", e);
        };
    }
}

// CONFIGURE SERVICES ******************************************************************************

pub fn configure_services(scfg: &mut service::web::ServiceConfig) {
    service_for_static_files!(
        scfg,
        base => "/base",
        [&config::SETTINGS.dev.pagetop_project_dir, "static/base"]
    );
    for m in ENABLED_PACKAGES.read().unwrap().iter() {
        m.configure_service(scfg);
    }
}
