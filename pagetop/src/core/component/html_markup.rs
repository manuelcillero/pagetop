use crate::core::component::{AnyComponent, ComponentTrait};
use crate::html::{html, Markup, RenderContext};
use crate::{define_handle, Handle};

define_handle!(HTML_MARKUP);

pub struct HtmlMarkup {
    pub markup: Markup,
}

impl Default for HtmlMarkup {
    fn default() -> Self {
        HtmlMarkup { markup: html! {} }
    }
}

impl ComponentTrait for HtmlMarkup {
    fn new() -> Self {
        HtmlMarkup::default()
    }

    fn handle(&self) -> Handle {
        HTML_MARKUP
    }

    fn default_render(&self, _rcx: &mut RenderContext) -> Markup {
        html! {
            (self.markup)
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl HtmlMarkup {
    pub fn with(mut self, html: Markup) -> Self {
        self.markup = html;
        self
    }
}
