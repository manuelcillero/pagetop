use crate::{all, trace};

mod definition;
pub use definition::ModuleTrait;

pub mod homepage;

pub fn register_module(m: &'static dyn ModuleTrait) {
    let mut modules = all::MODULES.write().unwrap();
    match modules.iter().find(|t| t.name() == m.name()) {
        None => {
            trace::info!("{}", m.name());
            modules.push(m);
        },
        Some(_) => {},
    }
}
