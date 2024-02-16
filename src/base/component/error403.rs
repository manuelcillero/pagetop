use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};

pub struct Error403;

impl ComponentTrait for Error403 {
    fn new() -> Self {
        Error403
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("FORBIDDEN ACCESS") }
            }
        })
    }
}
