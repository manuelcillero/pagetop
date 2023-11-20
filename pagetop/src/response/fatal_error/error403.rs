use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};
use crate::BaseHandle;

#[derive(BaseHandle)]
pub struct Error403;

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
