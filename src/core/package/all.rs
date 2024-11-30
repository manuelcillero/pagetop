use crate::core::action::add_action;
use crate::core::package::PackageRef;
use crate::core::theme::all::THEMES;
use crate::{global, service, static_files, static_files_service, trace};

use std::sync::{LazyLock, RwLock};

static_files!(assets);

// PACKAGES ****************************************************************************************

static ENABLED_PACKAGES: LazyLock<RwLock<Vec<PackageRef>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

static DROPPED_PACKAGES: LazyLock<RwLock<Vec<PackageRef>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

// REGISTER PACKAGES *******************************************************************************

pub fn register_packages(root_package: Option<PackageRef>) {
    // Initialize a list for packages to be enabled.
    let mut enabled_list: Vec<PackageRef> = Vec::new();

    // Add default theme to the enabled list.
    add_to_enabled(&mut enabled_list, &crate::base::theme::Basic);

    // Add default welcome page package to the enabled list.
    add_to_enabled(&mut enabled_list, &crate::base::package::Welcome);

    // If a root package is provided, add it to the enabled list.
    if let Some(package) = root_package {
        add_to_enabled(&mut enabled_list, package);
    }
    // Reverse the order to ensure packages are sorted from none to most dependencies.
    enabled_list.reverse();
    // Save the final list of enabled packages.
    ENABLED_PACKAGES.write().unwrap().append(&mut enabled_list);

    // Initialize a list for packages to be dropped.
    let mut dropped_list: Vec<PackageRef> = Vec::new();
    // If a root package is provided, analyze its dropped list.
    if let Some(package) = root_package {
        add_to_dropped(&mut dropped_list, package);
    }
    // Save the final list of dropped packages.
    DROPPED_PACKAGES.write().unwrap().append(&mut dropped_list);
}

fn add_to_enabled(list: &mut Vec<PackageRef>, package: PackageRef) {
    // Check if the package is not already in the enabled list to avoid duplicates.
    if !list.iter().any(|p| p.type_id() == package.type_id()) {
        // Add the package to the enabled list.
        list.push(package);

        // Reverse dependencies to add them in correct order (dependencies first).
        let mut dependencies = package.dependencies();
        dependencies.reverse();
        for d in &dependencies {
            add_to_enabled(list, *d);
        }

        // Check if the package has an associated theme to register.
        if let Some(theme) = package.theme() {
            let mut registered_themes = THEMES.write().unwrap();
            // Ensure the theme is not already registered to avoid duplicates.
            if !registered_themes
                .iter()
                .any(|t| t.type_id() == theme.type_id())
            {
                registered_themes.push(theme);
                trace::debug!("Enabling \"{}\" theme", theme.short_name());
            }
        } else {
            trace::debug!("Enabling \"{}\" package", package.short_name());
        }
    }
}

fn add_to_dropped(list: &mut Vec<PackageRef>, package: PackageRef) {
    // Iterate through packages recommended to be dropped.
    for d in &package.drop_packages() {
        // Check if the package is not already in the dropped list.
        if !list.iter().any(|p| p.type_id() == d.type_id()) {
            // Check if the package is currently enabled. If so, log a warning.
            if ENABLED_PACKAGES
                .read()
                .unwrap()
                .iter()
                .any(|p| p.type_id() == package.type_id())
            {
                trace::warn!(
                    "Trying to drop \"{}\" package which is enabled",
                    package.short_name()
                );
            } else {
                // If the package is not enabled, add it to the dropped list and log the action.
                list.push(*d);
                trace::debug!("Package \"{}\" dropped", d.short_name());
                // Recursively add the dependencies of the dropped package to the dropped list.
                // This ensures that all dependencies are also considered for dropping.
                for dependency in &package.dependencies() {
                    add_to_dropped(list, *dependency);
                }
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

// CONFIGURE SERVICES ******************************************************************************

pub fn configure_services(scfg: &mut service::web::ServiceConfig) {
    for m in ENABLED_PACKAGES.read().unwrap().iter() {
        m.configure_service(scfg);
    }
    static_files_service!(
        scfg,
        assets => "/",
        [&global::SETTINGS.dev.pagetop_project_dir, "static/assets"]
    );
}
