use crate::Lazy;
use crate::core::module::Module;

use std::sync::RwLock;

// -----------------------------------------------------------------------------
// Módulos registrados.
// -----------------------------------------------------------------------------

pub static MODULES: Lazy<RwLock<Vec<&dyn Module>>> = Lazy::new(|| {
    RwLock::new(vec![])
});

pub fn register_module(m: &'static (dyn Module + 'static)) {
    MODULES.write().unwrap().push(m);
}
