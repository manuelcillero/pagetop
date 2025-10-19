use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;

use std::fmt;

#[derive(AutoDefault, PartialEq)]
pub(crate) enum Toggle {
    #[default]
    Collapse,
    Offcanvas,
}

#[rustfmt::skip]
impl fmt::Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Toggle::Collapse  => write!(f, "collapse"),
            Toggle::Offcanvas => write!(f, "offcanvas"),
        }
    }
}

#[derive(AutoDefault)]
pub struct ButtonToggler;

impl Component for ButtonToggler {
    fn new() -> Self {
        ButtonToggler::default()
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            button
                type="button"
                class="navbar-toggler"
            {
                span class="navbar-toggler-icon" {}
            }
        })
    }
}

impl ButtonToggler {
    // ButtonToggler PRIVATE RENDER.

    pub(crate) fn render(
        &self,
        cx: &mut Context,
        id_content: String,
        data_bs_toggle: Toggle,
    ) -> Markup {
        let id_content_target = join!("#", id_content);
        let aria_expanded = if data_bs_toggle == Toggle::Collapse {
            Some("false")
        } else {
            None
        };
        html! {
            button
                type="button"
                class="navbar-toggler"
                data-bs-toggle=(data_bs_toggle)
                data-bs-target=(id_content_target)
                aria-controls=(id_content)
                aria-expanded=[aria_expanded]
                aria-label=[L10n::t("toggle", &LOCALES_BOOTSIER).lookup(cx)]
            {
                span class="navbar-toggler-icon" {}
            }
        }
    }
}
