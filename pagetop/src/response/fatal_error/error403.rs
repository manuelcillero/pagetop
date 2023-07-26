use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, PrepareMarkup};
use crate::{new_handle, Handle};

new_handle!(ERROR_403);

pub struct Error403;

impl ComponentTrait for Error403 {
    fn new() -> Self {
        Self
    }

    fn handle(&self) -> Handle {
        ERROR_403
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("FORBIDDEN ACCESS") }
            }
        })
    }
}
