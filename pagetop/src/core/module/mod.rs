use crate::core::global;

mod definition;
pub use definition::Module;

pub fn register_module(m: &'static (dyn Module + 'static)) {
    global::MODULES.write().unwrap().push(m);
}

pub fn find_module(name: &str) -> Option<&'static (dyn Module + 'static)> {
    let modules = global::MODULES.write().unwrap();
    match modules.iter().find(|t| t.name() == name) {
        Some(module) => Some(*module),
        _ => None,
    }
}
