use crate::core::component::RenderResources;

pub type IsRenderable = fn(&RenderResources) -> bool;

pub struct Renderable {
    pub check: IsRenderable,
}

impl Default for Renderable {
    fn default() -> Self {
        Renderable {
            check: render_always,
        }
    }
}

fn render_always(_: &RenderResources) -> bool {
    true
}
