use crate::core::component::Context;
use crate::SmartDefault;

pub type FnIsRenderable = fn(cx: &Context) -> bool;

#[derive(SmartDefault)]
pub struct Renderable {
    #[default(_code = "render_always")]
    pub check: FnIsRenderable,
}

fn render_always(_cx: &Context) -> bool {
    true
}
