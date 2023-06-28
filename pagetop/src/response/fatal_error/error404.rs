use crate::core::component::{AnyComponent, ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};
use crate::{use_handle, Handle};

use_handle!(ERROR_404);

pub struct Error404;

impl ComponentTrait for Error404 {
    fn new() -> Self {
        Self
    }

    fn handle(&self) -> Handle {
        ERROR_404
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("RESOURCE NOT FOUND") }
            }
        })
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}
