mod definition;
pub use definition::{
    BaseModule,
    ModuleTrait,
};

pub(crate) mod all;
pub use all::register_module;