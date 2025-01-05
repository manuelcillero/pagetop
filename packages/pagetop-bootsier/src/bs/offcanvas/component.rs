use pagetop::prelude::*;

use crate::bs::{
    BreakPoint, OffcanvasBackdrop, OffcanvasBodyScroll, OffcanvasPlacement, OffcanvasVisibility,
};
use crate::LOCALES_BOOTSIER;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Offcanvas {
    id        : OptionId,
    classes   : OptionClasses,
    title     : OptionTranslated,
    breakpoint: BreakPoint,
    placement : OffcanvasPlacement,
    visibility: OffcanvasVisibility,
    scrolling : OffcanvasBodyScroll,
    backdrop  : OffcanvasBackdrop,
    children  : Children,
}

impl ComponentTrait for Offcanvas {
    fn new() -> Self {
        Offcanvas::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                self.breakpoint().breakpoint_class("offcanvas"),
                self.placement().to_string(),
                self.visibility().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let body = self.children().render(cx);
        if body.is_empty() {
            return PrepareMarkup::None;
        }

        let id = cx.required_id::<Self>(self.id());
        let id_label = join_string!(id, "-label");
        let id_target = join_string!("#", id);

        let body_scroll = match self.body_scroll() {
            OffcanvasBodyScroll::Disabled => None,
            OffcanvasBodyScroll::Enabled => Some("true".to_string()),
        };

        let backdrop = match self.backdrop() {
            OffcanvasBackdrop::Disabled => Some("true".to_string()),
            OffcanvasBackdrop::Enabled => None,
            OffcanvasBackdrop::Static => Some("static".to_string()),
        };

        PrepareMarkup::With(html! {
            div
                id=(id)
                class=[self.classes().get()]
                tabindex="-1"
                data-bs-scroll=[body_scroll]
                data-bs-backdrop=[backdrop]
                aria-labelledby=(id_label)
            {
                div class="offcanvas-header" {
                    h5 class="offcanvas-title" id=(id_label) {
                        (self.title().escaped(cx.langid()))
                    }
                    button
                        type="button"
                        class="btn-close"
                        data-bs-dismiss="offcanvas"
                        data-bs-target=(id_target)
                        aria-label=[L10n::t("close", &LOCALES_BOOTSIER).using(cx.langid())]
                    {}
                }
                div class="offcanvas-body" {
                    (body)
                }
            }
        })
    }
}

impl Offcanvas {
    // Offcanvas BUILDER.

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
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title.alter_value(title);
        self
    }

    #[fn_builder]
    pub fn with_breakpoint(mut self, bp: BreakPoint) -> Self {
        self.breakpoint = bp;
        self
    }

    #[fn_builder]
    pub fn with_placement(mut self, placement: OffcanvasPlacement) -> Self {
        self.placement = placement;
        self
    }

    #[fn_builder]
    pub fn with_visibility(mut self, visibility: OffcanvasVisibility) -> Self {
        self.visibility = visibility;
        self
    }

    #[fn_builder]
    pub fn with_body_scroll(mut self, scrolling: OffcanvasBodyScroll) -> Self {
        self.scrolling = scrolling;
        self
    }

    #[fn_builder]
    pub fn with_backdrop(mut self, backdrop: OffcanvasBackdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    pub fn with_child(mut self, child: impl ComponentTrait) -> Self {
        self.children.add(Child::with(child));
        self
    }

    #[fn_builder]
    pub fn with_children(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }

    // Offcanvas GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn title(&self) -> &OptionTranslated {
        &self.title
    }

    pub fn breakpoint(&self) -> &BreakPoint {
        &self.breakpoint
    }

    pub fn placement(&self) -> &OffcanvasPlacement {
        &self.placement
    }

    pub fn visibility(&self) -> &OffcanvasVisibility {
        &self.visibility
    }

    pub fn body_scroll(&self) -> &OffcanvasBodyScroll {
        &self.scrolling
    }

    pub fn backdrop(&self) -> &OffcanvasBackdrop {
        &self.backdrop
    }

    pub fn children(&self) -> &Children {
        &self.children
    }
}
