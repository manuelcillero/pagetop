use crate::response::page::PageContext;

pub type IsRenderable = fn(&PageContext) -> bool;

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

fn render_always(_: &PageContext) -> bool {
    true
}
