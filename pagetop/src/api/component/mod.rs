mod action;
pub use action::{
    ACTION_BEFORE_RENDER_COMPONENT,
    ActionBeforeRenderComponent,
};

mod assets;
pub use assets::{
    Assets,
    Favicon,
    StyleSheet,
    JavaScript, JSMode,
};

mod definition;
pub use definition::{
    AnyComponent,
    BaseComponent,
    ComponentTrait,
    component_ref,
    component_mut,
};
use definition::{
    render_component,
};

mod holder;
pub use holder::{
    ComponentsHolder,
};

mod all;
pub use all::{
    add_component_to,
    common_components,
};

pub fn render_always() -> bool { true }

pub fn render_never() -> bool { false }