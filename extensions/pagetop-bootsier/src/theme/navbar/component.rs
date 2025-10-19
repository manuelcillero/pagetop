use pagetop::prelude::*;

use crate::prelude::*;
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
pub enum NavbarType {
    #[default]
    None,
    Nav(Typed<navbar::Nav>),
    Offcanvas(Typed<Offcanvas>),
    Text(L10n),
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Navbar {
    id         : AttrId,
    classes    : AttrClasses,
    expand     : BreakPoint,
    toggler    : NavbarToggler,
    navbar_type: NavbarType,
    contents   : Children,
    brand      : Typed<navbar::Brand>,
}

impl Component for Navbar {
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

        let navbar_type = match self.navbar_type() {
            NavbarType::None => return PrepareMarkup::None,
            NavbarType::Nav(nav) => {
                let id_content = join!(id, "-content");
                match self.toggler() {
                    NavbarToggler::Enabled => self.toggler_wrapper(
                        TOGGLE_COLLAPSE,
                        L10n::t("toggle", &LOCALES_BOOTSIER).lookup(cx),
                        id_content,
                        self.brand().render(cx),
                        nav.render(cx),
                    ),
                    NavbarToggler::Disabled => nav.render(cx),
                }
            }
            NavbarType::Offcanvas(oc) => {
                let id_content = oc.id().unwrap_or_default();
                self.toggler_wrapper(
                    TOGGLE_OFFCANVAS,
                    L10n::t("toggle", &LOCALES_BOOTSIER).lookup(cx),
                    id_content,
                    self.brand().render(cx),
                    oc.render(cx),
                )
            }
            NavbarType::Text(text) => html! {
                span class="navbar-text" {
                    (text.using(cx))
                }
            },
        };

        self.nav_wrapper(id, self.brand().render(cx), navbar_type)
    }
}

impl Navbar {
    pub fn with_nav(nav: navbar::Nav) -> Self {
        Navbar::default().with_navbar_type(NavbarType::Nav(Typed::with(nav)))
    }

    pub fn with_offcanvas(offcanvas: Offcanvas) -> Self {
        Navbar::default().with_navbar_type(NavbarType::Offcanvas(Typed::with(offcanvas)))
    }

    // Navbar BUILDER.

    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[builder_fn]
    pub fn with_expand(mut self, bp: BreakPoint) -> Self {
        self.expand = bp;
        self
    }

    #[builder_fn]
    pub fn with_toggler(mut self, toggler: NavbarToggler) -> Self {
        self.toggler = toggler;
        self
    }

    #[builder_fn]
    pub fn with_navbar_type(mut self, navbar_type: NavbarType) -> Self {
        self.navbar_type = navbar_type;
        self
    }

    pub fn with_content(mut self, content: navbar::Content) -> Self {
        self.contents.add(Child::with(content));
        self
    }

    #[builder_fn]
    pub fn with_contents(mut self, op: TypedOp<navbar::Content>) -> Self {
        self.contents.alter_typed(op);
        self
    }

    #[builder_fn]
    pub fn with_brand(mut self, brand: Option<navbar::Brand>) -> Self {
        self.brand.alter_component(brand);
        self
    }

    // Navbar GETTERS.

    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    pub fn expand(&self) -> &BreakPoint {
        &self.expand
    }

    pub fn toggler(&self) -> &NavbarToggler {
        &self.toggler
    }

    pub fn navbar_type(&self) -> &NavbarType {
        &self.navbar_type
    }

    pub fn contents(&self) -> &Children {
        &self.contents
    }

    pub fn brand(&self) -> &Typed<navbar::Brand> {
        &self.brand
    }

    // Navbar HELPERS.

    fn nav_wrapper(&self, id: String, brand: Markup, content: Markup) -> PrepareMarkup {
        if content.is_empty() {
            PrepareMarkup::None
        } else {
            PrepareMarkup::With(html! {
                (brand)
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
        brand: Markup,
        content: Markup,
    ) -> Markup {
        if content.is_empty() {
            html! {}
        } else {
            let id_content_target = join!("#", id_content);
            let aria_expanded = if data_bs_toggle == TOGGLE_COLLAPSE {
                Some("false")
            } else {
                None
            };
            html! {
                (brand)
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
