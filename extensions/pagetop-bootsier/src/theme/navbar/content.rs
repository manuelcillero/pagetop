use pagetop::prelude::*;

use crate::theme::navbar;

#[derive(AutoDefault)]
pub enum ContentType {
    #[default]
    None,
    Brand(Typed<navbar::Brand>),
    Nav(Typed<navbar::Nav>),
    Text(L10n),
}

// Item.

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Content {
    content: ContentType,
}

impl Component for Content {
    fn new() -> Self {
        Content::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.content() {
            ContentType::None => PrepareMarkup::None,
            ContentType::Brand(brand) => PrepareMarkup::With(html! {
                (brand.render(cx))
            }),
            ContentType::Nav(nav) => PrepareMarkup::With(html! {
                (nav.render(cx))
            }),
            ContentType::Text(text) => PrepareMarkup::With(html! {
                span class="navbar-text" {
                    (text.using(cx))
                }
            }),
        }
    }
}

impl Content {
    pub fn brand(content: navbar::Brand) -> Self {
        Content {
            content: ContentType::Brand(Typed::with(content)),
        }
    }

    pub fn nav(content: navbar::Nav) -> Self {
        Content {
            content: ContentType::Nav(Typed::with(content)),
        }
    }

    pub fn text(content: L10n) -> Self {
        Content {
            content: ContentType::Text(content),
        }
    }

    // Content GETTERS.

    pub fn content(&self) -> &ContentType {
        &self.content
    }
}
