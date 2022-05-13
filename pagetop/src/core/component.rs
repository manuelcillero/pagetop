mod hook;
pub use hook::{
    BEFORE_RENDER_COMPONENT_HOOK,
    BeforeRenderComponentHook,
};

mod assets;
pub use assets::{
    Assets,
    Favicon,
    JavaScript, JSMode,
    StyleSheet,
};

mod definition;
pub use definition::{
    AnyComponent,
    ComponentTrait,
    component_ref,
    component_mut,
};
use definition::render_component;

mod holder;
pub use holder::ComponentsHolder;

mod all;
pub use all::add_component_to;
pub(crate) use all::common_components;

pub fn render_always() -> bool { true }

pub fn render_never() -> bool { false }