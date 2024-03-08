use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Error404;

impl ComponentTrait for Error404 {
    fn new() -> Self {
        Error404::default()
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("RESOURCE NOT FOUND") }
            }
        })
    }
}
