use crate::core::component::{AnyComponent, ComponentTrait, RenderContext};
use crate::html::{html, Markup};
use crate::{define_handle, Handle};

define_handle!(ERROR_403);

pub struct Error403;

impl ComponentTrait for Error403 {
    fn new() -> Self {
        Self
    }

    fn handle(&self) -> Handle {
        ERROR_403
    }

    fn default_render(&self, _rcx: &mut RenderContext) -> Markup {
        html! {
            div {
                h1 { ("FORBIDDEN ACCESS") }
            }
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}
