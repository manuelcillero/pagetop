use pagetop::prelude::*;

new_handle!(THEME_BULMIX);

new_static_files!(bulmix);

pub struct Bulmix;

impl ModuleTrait for Bulmix {
    fn handle(&self) -> Handle {
        THEME_BULMIX
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bulmix)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        service_for_static_files!(scfg, bulmix => "/bulmix");
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
            COMPONENT_BASE_ANCHOR => {
                let a = component_as_mut::<Anchor>(component);
                a.alter_classes(
                    ClassesOp::SetDefault,
                    match a.anchor_type() {
                        AnchorType::Button => "button is-primary",
                        _ => "",
                    },
                );
            }
            COMPONENT_BASE_HEADING => {
                let h = component_as_mut::<Heading>(component);
                match h.display() {
                    HeadingDisplay::Subtitle => h.alter_classes(
                        ClassesOp::SetDefault, "subtitle"
                    ),
                    _ => h.alter_classes(
                        ClassesOp::AddDefault, "title"
                    ),
                };
            }
            COMPONENT_BASE_PARAGRAPH => {
                let p = component_as_mut::<Paragraph>(component);
                let original = concat_string!("block ", p.font_size().to_string());
                p.alter_classes(
                    ClassesOp::SetDefault,
                    match p.font_size() {
                        FontSize::ExtraLarge => "block is-size-1",
                        FontSize::XxLarge    => "block is-size-2",
                        FontSize::XLarge     => "block is-size-3",
                        FontSize::Large      => "block is-size-4",
                        FontSize::Medium     => "block is-size-5",
                        _                    => original.as_str(),
                    },
                );

            }
            _ => {}
        }
    }

    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        cx: &mut Context,
    ) -> Option<Markup> {
        match component.handle() {
            COMPONENT_BASE_ICON => {
                let icon = component_as_ref::<Icon>(component);
                if icon.icon_name().is_empty() {
                    return None
                };
                cx.set_param::<bool>(PARAM_BASE_INCLUDE_ICONS, true);
                Some(html! {
                    span class="icon" {
                        i class=[icon.classes().get()] {}
                    }
                })
            }
            _ => None,
        }
    }
}
