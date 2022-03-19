use crate::{Lazy, app, base, run_now, trace};
use crate::config::SETTINGS;
use crate::db::migration::*;
use crate::module::*;
use crate::theme::*;

use std::sync::RwLock;

// -----------------------------------------------------------------------------
// Módulos registrados.
// -----------------------------------------------------------------------------

pub static MODULES: Lazy<RwLock<Vec<&dyn ModuleTrait>>> = Lazy::new(
    || { RwLock::new(Vec::new()) }
);

pub fn register_module(m: &'static dyn ModuleTrait) {
    let mut modules = MODULES.write().unwrap();
    match modules.iter().find(|t| t.name() == m.name()) {
        None => {
            trace::info!("{}", m.name());
            modules.push(m);
        },
        Some(_) => {},
    }
}

pub fn modules(cfg: &mut app::web::ServiceConfig) {
    for m in MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub fn run_migrations() {
    run_now({
        struct Migrator;
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {
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

// -----------------------------------------------------------------------------
// Temas registrados y tema predeterminado.
// -----------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

pub static THEMES: Lazy<RwLock<Vec<&dyn ThemeTrait>>> = Lazy::new(|| {
    RwLock::new(vec![
        &base::theme::aliner::AlinerTheme,
        &base::theme::minimal::MinimalTheme,
        &base::theme::bootsier::BootsierTheme,
    ])
});

pub static DEFAULT_THEME: Lazy<&dyn ThemeTrait> = Lazy::new(|| {
    for t in THEMES.read().unwrap().iter() {
        if t.name().to_lowercase() == SETTINGS.app.theme.to_lowercase() {
            return *t;
        }
    }
    &base::theme::bootsier::BootsierTheme
});

pub fn register_theme(t: &'static dyn ThemeTrait) {
    THEMES.write().unwrap().push(t);
}

pub fn theme_by_name(name: &str) -> Option<&'static dyn ThemeTrait> {
    let themes = crate::global::THEMES.write().unwrap();
    match themes.iter().find(|t| t.name() == name) {
        Some(theme) => Some(*theme),
        _ => None,
    }
}

pub fn themes(cfg: &mut app::web::ServiceConfig) {
    cfg.service(actix_web_static_files::ResourceFiles::new(
        "/theme",
        assets()
    ));

    for t in THEMES.read().unwrap().iter() {
        t.configure_theme(cfg);
    }
}

