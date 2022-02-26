use crate::core::all::MODULES;

mod api;
pub use api::Module;

pub fn register_module(m: &'static (dyn Module + 'static)) {
    MODULES.write().unwrap().push(m);
}

pub fn find_module(name: &str) -> Option<&'static (dyn Module + 'static)> {
    let modules = MODULES.write().unwrap();
    match modules.iter().find(|t| t.name() == name) {
        Some(module) => Some(*module),
        _ => None,
    }
}
