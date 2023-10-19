use pagetop::prelude::*;

new_handle!(THEME_BULMIX);

static_files!(bulmix);

pub struct Bulmix;

impl ModuleTrait for Bulmix {
    fn handle(&self) -> Handle {
        THEME_BULMIX
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bulmix)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, "/bulmix", bulmix);
    }
}

impl ThemeTrait for Bulmix {
    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/bulmix/css/bulma.min.css")
                    .with_version("0.9.4")
                    .with_weight(-99),
            ))
            .alter_context(ContextOp::AddBaseAssets);
    }

    #[rustfmt::skip]
    fn before_prepare_component(
        &self,
        component: &mut dyn ComponentTrait,
        _cx: &mut Context,
    ) {
        match component.handle() {
            COMPONENT_ANCHOR => {
                let a = component_as_mut::<Anchor>(component);
                a.alter_classes(
                    ClassesOp::SetDefault,
                    match a.anchor_type() {
                        AnchorType::Button => "button is-primary",
                        _ => "",
                    },
                );
            }
            COMPONENT_HEADING => {
                let h = component_as_mut::<Heading>(component);
                h.alter_classes(
                    ClassesOp::SetDefault,
                    match h.display() {
                        HeadingDisplay::XxLarge  => "title is-1",
                        HeadingDisplay::Large    => "title is-2",
                        HeadingDisplay::Medium   => "title is-3",
                        HeadingDisplay::Small    => "title is-4",
                        HeadingDisplay::XxSmall  => "title is-5",
                        HeadingDisplay::Normal   => "title",
                        HeadingDisplay::Subtitle => "subtitle",
                    },
                );
            }
            COMPONENT_PARAGRAPH => {
                let p = component_as_mut::<Paragraph>(component);
                p.alter_classes(
                    ClassesOp::SetDefault,
                    match p.display() {
                        ParagraphDisplay::XxLarge => "is-size-2",
                        ParagraphDisplay::Large   => "is-size-3",
                        ParagraphDisplay::Medium  => "is-size-4",
                        ParagraphDisplay::Small   => "is-size-5",
                        ParagraphDisplay::XxSmall => "is-size-6",
                        ParagraphDisplay::Normal  => "",
                    },
                );
            }
            _ => {}
        }
    }

    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        _cx: &mut Context,
    ) -> Option<Markup> {
        match component.handle() {
            COMPONENT_ICON => {
                let icon = component_as_ref::<Icon>(component);
                Some(html! {
                    span class="icon" {
                        i class=[icon.classes().get()] {};
                    }
                })
            }
            _ => None,
        }
    }
}
