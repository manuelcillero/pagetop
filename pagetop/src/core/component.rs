mod context;
pub use context::{InContext, InContextOp};

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

pub type Renderable = fn(_: &InContext) -> bool;

pub fn render_always(_: &InContext) -> bool { true }

pub fn render_never(_: &InContext) -> bool { false }