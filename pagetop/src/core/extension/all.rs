use crate::core::action::add_action;
use crate::core::extension::ExtensionRef;
use crate::core::theme::all::THEMES;
use crate::{global, include_files, include_files_service, service, trace};

use std::sync::{LazyLock, RwLock};

// EXTENSIONS **************************************************************************************

static ENABLED_EXTENSIONS: LazyLock<RwLock<Vec<ExtensionRef>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

static DROPPED_EXTENSIONS: LazyLock<RwLock<Vec<ExtensionRef>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

// REGISTER EXTENSIONS *****************************************************************************

pub fn register_extensions(root_extension: Option<ExtensionRef>) {
    // Initialize a list for extensions to be enabled.
    let mut enabled_list: Vec<ExtensionRef> = Vec::new();

    // Add default theme to the enabled list.
    add_to_enabled(&mut enabled_list, &crate::base::theme::Basic);

    // If a root extension is provided, add it to the enabled list.
    if let Some(extension) = root_extension {
        add_to_enabled(&mut enabled_list, extension);
    }

    // Add default welcome page extension to the enabled list.
    add_to_enabled(&mut enabled_list, &crate::base::extension::Welcome);

    // Save the final list of enabled extensions.
    ENABLED_EXTENSIONS
        .write()
        .unwrap()
        .append(&mut enabled_list);

    // Initialize a list for extensions to be dropped.
    let mut dropped_list: Vec<ExtensionRef> = Vec::new();
    // If a root extension is provided, analyze its dropped list.
    if let Some(extension) = root_extension {
        add_to_dropped(&mut dropped_list, extension);
    }
    // Save the final list of dropped extensions.
    DROPPED_EXTENSIONS
        .write()
        .unwrap()
        .append(&mut dropped_list);
}

fn add_to_enabled(list: &mut Vec<ExtensionRef>, extension: ExtensionRef) {
    // Check if the extension is not already in the enabled list to avoid duplicates.
    if !list.iter().any(|p| p.type_id() == extension.type_id()) {
        // Add the extension dependencies in reverse order first.
        for d in extension.dependencies().iter().rev() {
            add_to_enabled(list, *d);
        }

        // Add the extension itself to the enabled list.
        list.push(extension);

        // Check if the extension has an associated theme to register.
        if let Some(theme) = extension.theme() {
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
            trace::debug!("Enabling \"{}\" extension", extension.short_name());
        }
    }
}

fn add_to_dropped(list: &mut Vec<ExtensionRef>, extension: ExtensionRef) {
    // Iterate through extensions recommended to be dropped.
    for d in &extension.drop_extensions() {
        // Check if the extension is not already in the dropped list.
        if !list.iter().any(|p| p.type_id() == d.type_id()) {
            // Check if the extension is currently enabled. If so, log a warning.
            if ENABLED_EXTENSIONS
                .read()
                .unwrap()
                .iter()
                .any(|p| p.type_id() == extension.type_id())
            {
                trace::warn!(
                    "Trying to drop \"{}\" extension which is enabled",
                    extension.short_name()
                );
            } else {
                // If the extension is not enabled, add it to the dropped list and log the action.
                list.push(*d);
                trace::debug!("Extension \"{}\" dropped", d.short_name());
                // Recursively add the dependencies of the dropped extension to the dropped list.
                // This ensures that all dependencies are also considered for dropping.
                for dependency in &extension.dependencies() {
                    add_to_dropped(list, *dependency);
                }
            }
        }
    }
}

// REGISTER ACTIONS ********************************************************************************

pub fn register_actions() {
    for m in ENABLED_EXTENSIONS.read().unwrap().iter() {
        for a in m.actions().into_iter() {
            add_action(a);
        }
    }
}

// INIT EXTENSIONS *********************************************************************************

pub fn init_extensions() {
    trace::info!("Calling application bootstrap");
    for m in ENABLED_EXTENSIONS.read().unwrap().iter() {
        m.init();
    }
}

// CONFIGURE SERVICES ******************************************************************************

include_files!(assets);

pub fn configure_services(scfg: &mut service::web::ServiceConfig) {
    for m in ENABLED_EXTENSIONS.read().unwrap().iter() {
        m.configure_service(scfg);
    }
    include_files_service!(
        scfg, assets => "/", [&global::SETTINGS.dev.pagetop_project_dir, "static"]
    );
}
