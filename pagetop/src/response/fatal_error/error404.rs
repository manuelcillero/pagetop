use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};
use crate::impl_handle;

pub struct Error404;

impl_handle!(ERROR_404 for Error404);

impl ComponentTrait for Error404 {
    fn new() -> Self {
        Self
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("RESOURCE NOT FOUND") }
            }
        })
    }
}
