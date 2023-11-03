use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};
use crate::impl_handle;

pub struct Error403;

impl_handle!(ERROR_403 for Error403);

impl ComponentTrait for Error403 {
    fn new() -> Self {
        Self
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("FORBIDDEN ACCESS") }
            }
        })
    }
}
