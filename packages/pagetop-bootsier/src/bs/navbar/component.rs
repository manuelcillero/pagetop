use pagetop::prelude::*;

use crate::bs::navbar;
use crate::bs::{BreakPoint, Offcanvas};
use crate::LOCALES_BOOTSIER;

const TOGGLE_COLLAPSE: &str = "collapse";
const TOGGLE_OFFCANVAS: &str = "offcanvas";

#[derive(AutoDefault)]
pub enum NavbarToggler {
    #[default]
    Enabled,
    Disabled,
}

#[derive(AutoDefault)]
pub enum NavbarContent {
    #[default]
    None,
    Nav(Typed<navbar::Nav>),
    Offcanvas(Typed<Offcanvas>),
    Text(L10n),
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Navbar {
    id     : OptionId,
    classes: OptionClasses,
    expand : BreakPoint,
    toggler: NavbarToggler,
    content: NavbarContent,
}

impl ComponentTrait for Navbar {
    fn new() -> Self {
        Navbar::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                "navbar".to_string(),
                self.expand().try_class("navbar-expand").unwrap_or_default(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id = cx.required_id::<Self>(self.id());

        let content = match self.content() {
            NavbarContent::None => return PrepareMarkup::None,
            NavbarContent::Nav(nav) => {
                let id_content = join_string!(id, "-content");
                match self.toggler() {
                    NavbarToggler::Enabled => self.toggler_wrapper(
                        TOGGLE_COLLAPSE,
                        L10n::t("toggle", &LOCALES_BOOTSIER).using(cx.langid()),
                        id_content,
                        nav.render(cx),
                    ),
                    NavbarToggler::Disabled => nav.render(cx),
                }
            }
            NavbarContent::Offcanvas(oc) => {
                let id_content = oc.id().unwrap_or_default();
                self.toggler_wrapper(
                    TOGGLE_OFFCANVAS,
                    L10n::t("toggle", &LOCALES_BOOTSIER).using(cx.langid()),
                    id_content,
                    oc.render(cx),
                )
            }
            NavbarContent::Text(text) => html! {
                span class="navbar-text" {
                    (text.escaped(cx.langid()))
                }
            },
        };

        self.nav_wrapper(id, content)
    }
}

impl Navbar {
    pub fn with_nav(nav: navbar::Nav) -> Self {
        Navbar::default().with_content(NavbarContent::Nav(Typed::with(nav)))
    }

    pub fn with_offcanvas(offcanvas: Offcanvas) -> Self {
        Navbar::default().with_content(NavbarContent::Offcanvas(Typed::with(offcanvas)))
    }

    // Navbar BUILDER.

    #[fn_builder]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl Into<String>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn with_expand(mut self, bp: BreakPoint) -> Self {
        self.expand = bp;
        self
    }

    #[fn_builder]
    pub fn with_toggler(mut self, toggler: NavbarToggler) -> Self {
        self.toggler = toggler;
        self
    }

    #[fn_builder]
    pub fn with_content(mut self, content: NavbarContent) -> Self {
        self.content = content;
        self
    }

    // Navbar GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn expand(&self) -> &BreakPoint {
        &self.expand
    }

    pub fn toggler(&self) -> &NavbarToggler {
        &self.toggler
    }

    pub fn content(&self) -> &NavbarContent {
        &self.content
    }

    // Navbar HELPERS.

    fn nav_wrapper(&self, id: String, content: Markup) -> PrepareMarkup {
        if content.is_empty() {
            PrepareMarkup::None
        } else {
            PrepareMarkup::With(html! {
                nav id=(id) class=[self.classes().get()] {
                    div class="container-fluid" {
                        (content)
                    }
                }
            })
        }
    }

    fn toggler_wrapper(
        &self,
        data_bs_toggle: &str,
        aria_label: Option<String>,
        id_content: String,
        content: Markup,
    ) -> Markup {
        if content.is_empty() {
            html! {}
        } else {
            let id_content_target = join_string!("#", id_content);
            let aria_expanded = if data_bs_toggle == TOGGLE_COLLAPSE {
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
                    aria-label=[aria_label]
                {
                    span class="navbar-toggler-icon" {}
                }
                div id=(id_content) class="collapse navbar-collapse" {
                    (content)
                }
            }
        }
    }
}
