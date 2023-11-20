use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};
use crate::BaseHandle;

#[derive(BaseHandle)]
pub struct Error404;

impl ComponentTrait for Error404 {
    fn new() -> Self {
        Error404
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("RESOURCE NOT FOUND") }
            }
        })
    }
}
