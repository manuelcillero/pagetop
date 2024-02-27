use crate::core::component::Context;
use crate::AutoDefault;

pub type FnIsRenderable = fn(cx: &Context) -> bool;

#[derive(AutoDefault)]
pub struct Renderable {
    #[default(_code = "render_always")]
    pub check: FnIsRenderable,
}

fn render_always(_cx: &Context) -> bool {
    true
}
