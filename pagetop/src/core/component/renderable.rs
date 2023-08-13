use crate::core::component::Context;

pub type FnIsRenderable = fn(cx: &Context) -> bool;

pub struct Renderable {
    pub check: FnIsRenderable,
}

impl Default for Renderable {
    fn default() -> Self {
        Renderable {
            check: render_always,
        }
    }
}

fn render_always(_: &Context) -> bool {
    true
}
