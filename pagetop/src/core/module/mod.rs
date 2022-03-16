use crate::core::all;

mod definition;
pub use definition::ModuleTrait;

pub fn register_module(m: &'static (dyn ModuleTrait + 'static)) {
    all::MODULES.write().unwrap().push(m);
}

pub fn find_module(name: &str) -> Option<&'static (dyn ModuleTrait + 'static)> {
    let modules = all::MODULES.write().unwrap();
    match modules.iter().find(|t| t.name() == name) {
        Some(module) => Some(*module),
        _ => None,
    }
}
