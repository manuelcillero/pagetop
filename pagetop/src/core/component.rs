mod hook;
pub use hook::{
    BEFORE_RENDER_COMPONENT_HOOK,
    BeforeRenderComponentHook,
};

mod context;
pub use context::InContext;

mod definition;
pub use definition::{
    AnyComponent,
    ComponentTrait,
    component_ref,
    component_mut,
};
use definition::render_component;

mod bundle;
pub use bundle::ComponentsBundle;

mod all;
pub use all::add_component_to;
pub(crate) use all::common_components;

pub fn render_always() -> bool { true }

pub fn render_never() -> bool { false }