use pagetop::prelude::*;

new_static_files!(bulmix);

pub struct Bulmix;

impl_handle!(THEME_BULMIX for Bulmix);

impl ModuleTrait for Bulmix {
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

    fn before_prepare_component(&self, component: &mut dyn ComponentTrait, _cx: &mut Context) {
        match component.handle() {
            COMPONENT_BASE_ICON => {
                let i = component_as_mut::<Icon>(component);
                match i.font_size() {
                    FontSize::ExtraLarge => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "is-size-1");
                    }
                    FontSize::XxLarge => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "is-size-2");
                    }
                    FontSize::XLarge => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "is-size-3");
                    }
                    FontSize::Large => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "is-size-4");
                    }
                    FontSize::Medium => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "is-size-5");
                    }
                    _ => {}
                };
            }
            COMPONENT_BASE_ANCHOR => {
                let a = component_as_mut::<Anchor>(component);
                match a.font_size() {
                    FontSize::ExtraLarge => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "is-size-1");
                    }
                    FontSize::XxLarge => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "is-size-2");
                    }
                    FontSize::XLarge => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "is-size-3");
                    }
                    FontSize::Large => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "is-size-4");
                    }
                    FontSize::Medium => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "is-size-5");
                    }
                    _ => {}
                };
                if let AnchorType::Button = a.anchor_type() {
                    a.alter_classes(
                        ClassesOp::Replace(a.anchor_type().to_string()),
                        "button is-primary",
                    );
                };
            }
            COMPONENT_BASE_HEADING => {
                let h = component_as_mut::<Heading>(component);
                match h.display() {
                    HeadingDisplay::Subtitle => {
                        h.alter_classes(ClassesOp::Replace(h.display().to_string()), "subtitle")
                    }
                    _ => h.alter_classes(ClassesOp::Add, "title"),
                };
            }
            COMPONENT_BASE_PARAGRAPH => {
                let p = component_as_mut::<Paragraph>(component);
                p.alter_classes(ClassesOp::Add, "block");
                match p.font_size() {
                    FontSize::ExtraLarge => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "is-size-1");
                    }
                    FontSize::XxLarge => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "is-size-2");
                    }
                    FontSize::XLarge => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "is-size-3");
                    }
                    FontSize::Large => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "is-size-4");
                    }
                    FontSize::Medium => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "is-size-5");
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }

    fn render_component(&self, component: &dyn ComponentTrait, cx: &mut Context) -> Option<Markup> {
        match component.handle() {
            COMPONENT_BASE_ICON => {
                let icon = component_as_ref::<Icon>(component);
                if icon.icon_name().is_empty() {
                    return None;
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
