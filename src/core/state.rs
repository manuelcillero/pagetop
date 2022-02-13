use crate::Lazy;
use crate::config::SETTINGS;
use crate::core::theme::Theme;
use crate::core::module::Module;
use crate::core::response::page::{PageComponent, PageContainer};
use crate::base;

use std::sync::RwLock;
use std::collections::HashMap;

// -----------------------------------------------------------------------------
// Temas registrados.
// -----------------------------------------------------------------------------

pub static THEMES: Lazy<RwLock<Vec<&dyn Theme>>> = Lazy::new(|| {
    RwLock::new(vec![
        &base::theme::minimal::MinimalTheme,
    ])
});

pub static THEME: Lazy<&dyn Theme> = Lazy::new(|| {
    for t in THEMES.read().unwrap().iter() {
        if t.name().to_lowercase() == SETTINGS.app.theme.to_lowercase() {
            return *t;
        }
    }
    &base::theme::minimal::MinimalTheme
});

pub fn register_theme(t: &'static (dyn Theme + 'static)) {
    THEMES.write().unwrap().push(t);
}

// -----------------------------------------------------------------------------
// Módulos registrados.
// -----------------------------------------------------------------------------

pub static MODULES: Lazy<RwLock<Vec<&dyn Module>>> = Lazy::new(|| {
    RwLock::new(vec![])
});

pub fn register_module(m: &'static (dyn Module + 'static)) {
    MODULES.write().unwrap().push(m);
}

// -----------------------------------------------------------------------------
// Componentes globales.
// -----------------------------------------------------------------------------

pub static COMPONENTS: Lazy<RwLock<HashMap<&str, PageContainer>>> = Lazy::new(
    || { RwLock::new(HashMap::new()) }
);

#[allow(dead_code)]
pub fn add_component_to(region: &'static str, component: impl PageComponent) {
    let mut hmap = COMPONENTS.write().unwrap();
    if let Some(regions) = hmap.get_mut(region) {
        regions.add(component);
    } else {
        hmap.insert(region, PageContainer::new_with(component));
    }
}
