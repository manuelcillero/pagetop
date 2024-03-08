use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Error403;

impl ComponentTrait for Error403 {
    fn new() -> Self {
        Error403::default()
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div {
                h1 { ("FORBIDDEN ACCESS") }
            }
        })
    }
}
