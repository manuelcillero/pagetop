use pagetop::prelude::*;

use crate::bs::navbar;
use crate::bs::{BreakPoint, Offcanvas};
use crate::LOCALES_BOOTSIER;

#[derive(AutoDefault)]
pub enum NavbarType {
    #[default]
    Default,
    Basic,
    Offcanvas(Typed<Offcanvas>),
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Navbar {
    id         : OptionId,
    classes    : OptionClasses,
    navbar_type: NavbarType,
    expand     : BreakPoint,
    elements   : Children,
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
                self.expand().breakpoint_class("navbar-expand"),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let elements = self.elements().render(cx);
        if elements.is_empty() {
            return PrepareMarkup::None;
        }

        let id = cx.required_id::<Self>(self.id());
        let (output, id_content) = if let NavbarType::Offcanvas(oc) = self.navbar_type() {
            (
                oc.writable()
                    .alter_children(ChildOp::Prepend(Child::with(Html::with(elements))))
                    .render(cx),
                cx.required_id::<Offcanvas>(oc.id()),
            )
        } else {
            (elements, join_string!(id, "-content"))
        };
        let id_content_target = join_string!("#", id_content);

        PrepareMarkup::With(html! {
            nav id=(id) class=[self.classes().get()] {
                div class="container-fluid" {
                    @match self.navbar_type() {
                        NavbarType::Default => {
                            button
                                type="button"
                                class="navbar-toggler"
                                data-bs-toggle="collapse"
                                data-bs-target=(id_content_target)
                                aria-controls=(id_content)
                                aria-expanded="false"
                                aria-label=[L10n::t("toggle", &LOCALES_BOOTSIER).using(cx.langid())]
                            {
                                span class="navbar-toggler-icon" {}
                            }
                            div id=(id_content) class="collapse navbar-collapse" {
                                (output)
                            }
                        },
                        NavbarType::Basic => {
                            (output)
                        },
                        NavbarType::Offcanvas(_) => {
                            button
                                type="button"
                                class="navbar-toggler"
                                data-bs-toggle="offcanvas"
                                data-bs-target=(id_content_target)
                                aria-controls=(id_content)
                                aria-label=[L10n::t("toggle", &LOCALES_BOOTSIER).using(cx.langid())]
                            {
                                span class="navbar-toggler-icon" {}
                            }
                            (output)
                        },
                    }
                }
            }
        })
    }
}

impl Navbar {
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
    pub fn with_type(mut self, navbar_type: NavbarType) -> Self {
        self.navbar_type = navbar_type;
        self
    }

    #[fn_builder]
    pub fn with_expand(mut self, bp: BreakPoint) -> Self {
        self.expand = bp;
        self
    }

    #[fn_builder]
    pub fn with_nav(mut self, op: TypedOp<navbar::Nav>) -> Self {
        self.elements.alter_typed(op);
        self
    }

    // Navbar GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn navbar_type(&self) -> &NavbarType {
        &self.navbar_type
    }

    pub fn expand(&self) -> &BreakPoint {
        &self.expand
    }

    pub fn elements(&self) -> &Children {
        &self.elements
    }
}
