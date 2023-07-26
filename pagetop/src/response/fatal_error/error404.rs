use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};
use crate::{new_handle, Handle};

new_handle!(ERROR_404);

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
}
