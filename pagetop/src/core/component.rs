mod definition;
pub use definition::{component_mut, component_ref, AnyComponent, BaseComponent, ComponentTrait};

mod bundle;
pub use bundle::ComponentsBundle;

mod all;
pub use all::add_component_to;
pub(crate) use all::common_components;

use crate::response::page::PageContext;

pub type Renderable = fn(_: &PageContext) -> bool;

pub fn render_always(_: &PageContext) -> bool {
    true
}

pub fn render_never(_: &PageContext) -> bool {
    false
}
