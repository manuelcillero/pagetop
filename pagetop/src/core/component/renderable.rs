use crate::html::RenderContext;

pub type IsRenderable = fn(&RenderContext) -> bool;

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

fn render_always(_: &RenderContext) -> bool {
    true
}
