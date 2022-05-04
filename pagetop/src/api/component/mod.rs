pub mod action;

mod assets;
pub use assets::{
    Favicon,
    StyleSheet,
    JavaScript, JSMode,
    PageAssets,
};

mod definition;
pub use definition::{
    AnyComponent,
    BaseComponent,
    ComponentTrait,
    component_ref,
    component_mut,
};
use definition::render_component;

mod container;
pub use container::PageContainer;

mod all;
pub use all::{
    add_component_to,
    common_components,
};

pub fn render_always() -> bool { true }

pub fn render_never() -> bool { false }