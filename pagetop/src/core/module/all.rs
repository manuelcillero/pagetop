use crate::{Lazy, app, run_now, trace};
use crate::db::*;
use super::{ExtensionTrait, ModuleTrait};

use std::sync::{Arc, RwLock};
use std::collections::HashMap;

// Módulos registrados.
static MODULES: Lazy<RwLock<Vec<&dyn ModuleTrait>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

// Extensiones registradas.
static EXTENSIONS: Lazy<RwLock<HashMap<&str, Arc<Vec<&dyn ExtensionTrait>>>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn register_module(module: &'static dyn ModuleTrait) {
    let mut list: Vec<&dyn ModuleTrait> = Vec::new();
    add_to(&mut list, module);
    list.reverse();
    MODULES.write().unwrap().append(&mut list);
}

fn add_to(list: &mut Vec<&dyn ModuleTrait>, module: &'static dyn ModuleTrait) {
    if !MODULES.read().unwrap().iter().any(|m| m.type_name() == module.type_name()) {
        if !list.iter().any(|m| m.type_name() == module.type_name()) {
            trace::debug!("Registering \"{}\" module", module.single_name());
            list.push(module);


            let mut hmap = EXTENSIONS.write().unwrap();
            for e in module.extensions().iter() {
                if let Some(extensions) = hmap.get_mut(e.type_name()) {
                    let v = Arc::get_mut(extensions).unwrap();
                    v.push(*e);
                    v.sort_by_key(|e| e.weight());
                } else {
                    hmap.insert(e.type_name(), Arc::new(vec![*e]));
                }
            }


            let mut dependencies = module.dependencies();
            dependencies.reverse();
            for d in dependencies.iter() {
                add_to(list, *d);
            }
        }
    }
}

pub fn modules(cfg: &mut app::web::ServiceConfig) {
    for m in MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}

pub fn extensions(type_name: &'static str) -> Option<Arc<Vec<&dyn ExtensionTrait>>> {
    match EXTENSIONS.read().unwrap().get(type_name) {
        Some(extensions) => Some(extensions.clone()),
        _ => None,
    }
}

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub fn migrations() {
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
