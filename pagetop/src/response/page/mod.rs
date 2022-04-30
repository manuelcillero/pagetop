mod assets;
pub use assets::{
    Favicon,
    StyleSheet,
    JavaScript, JSMode,
    PageAssets,
};

mod component;
pub use component::{
    AnyComponent,
    ComponentTrait,
    component_ref,
    component_mut,
};

mod container;
pub use container::PageContainer;

mod page;
pub use page::Page;
pub use page::render_component;
pub use page::add_component_to;

pub fn render_always() -> bool { true }

pub fn render_never() -> bool { false }