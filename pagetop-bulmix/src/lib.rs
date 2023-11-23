use pagetop::prelude::*;

static_files!(bulmix);

#[derive(BindHandle)]
pub struct Bulmix;

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
            .alter_context(ContextOp::AddBaseAssets)
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/bulmix/css/styles.css").with_version("0.0.1"),
            ));
    }

    fn before_prepare_component(&self, component: &mut dyn ComponentTrait, _cx: &mut Context) {
        match component.handle() {
            h if Icon::matches_handle(h) => {
                if let Some(i) = component_as_mut::<Icon>(component) {
                    match i.font_size() {
                        FontSize::ExtraLarge => {
                            i.replace_classes(i.font_size().to_string(), "is-size-1");
                        }
                        FontSize::XxLarge => {
                            i.replace_classes(i.font_size().to_string(), "is-size-2");
                        }
                        FontSize::XLarge => {
                            i.replace_classes(i.font_size().to_string(), "is-size-3");
                        }
                        FontSize::Large => {
                            i.replace_classes(i.font_size().to_string(), "is-size-4");
                        }
                        FontSize::Medium => {
                            i.replace_classes(i.font_size().to_string(), "is-size-5");
                        }
                        _ => {}
                    };
                }
            }
            h if Button::matches_handle(h) => {
                if let Some(b) = component_as_mut::<Button>(component) {
                    match b.style() {
                        ButtonStyle::Default => {
                            b.replace_classes(b.style().to_string(), "button is-primary");
                        }
                        ButtonStyle::Info => {
                            b.replace_classes(b.style().to_string(), "button is-info");
                        }
                        ButtonStyle::Success => {
                            b.replace_classes(b.style().to_string(), "button is-success");
                        }
                        ButtonStyle::Warning => {
                            b.replace_classes(b.style().to_string(), "button is-warning");
                        }
                        ButtonStyle::Danger => {
                            b.replace_classes(b.style().to_string(), "button is-danger");
                        }
                        ButtonStyle::Light => {
                            b.replace_classes(b.style().to_string(), "button is-light");
                        }
                        ButtonStyle::Dark => {
                            b.replace_classes(b.style().to_string(), "button is-dark");
                        }
                        ButtonStyle::Link => {
                            b.replace_classes(b.style().to_string(), "button is-text");
                        }
                    };
                    match b.font_size() {
                        FontSize::ExtraLarge => {
                            b.replace_classes(b.font_size().to_string(), "is-size-1");
                        }
                        FontSize::XxLarge => {
                            b.replace_classes(b.font_size().to_string(), "is-size-2");
                        }
                        FontSize::XLarge => {
                            b.replace_classes(b.font_size().to_string(), "is-size-3");
                        }
                        FontSize::Large => {
                            b.replace_classes(b.font_size().to_string(), "is-size-4");
                        }
                        FontSize::Medium => {
                            b.replace_classes(b.font_size().to_string(), "is-size-5");
                        }
                        _ => {}
                    };
                }
            }
            h if Heading::matches_handle(h) => {
                if let Some(h) = component_as_mut::<Heading>(component) {
                    match h.size() {
                        HeadingSize::Subtitle => {
                            h.replace_classes(h.size().to_string(), "subtitle")
                        }
                        _ => h.add_classes("title"),
                    };
                }
            }
            h if Paragraph::matches_handle(h) => {
                if let Some(p) = component_as_mut::<Paragraph>(component) {
                    p.add_classes("block");
                    match p.font_size() {
                        FontSize::ExtraLarge => {
                            p.replace_classes(p.font_size().to_string(), "is-size-1");
                        }
                        FontSize::XxLarge => {
                            p.replace_classes(p.font_size().to_string(), "is-size-2");
                        }
                        FontSize::XLarge => {
                            p.replace_classes(p.font_size().to_string(), "is-size-3");
                        }
                        FontSize::Large => {
                            p.replace_classes(p.font_size().to_string(), "is-size-4");
                        }
                        FontSize::Medium => {
                            p.replace_classes(p.font_size().to_string(), "is-size-5");
                        }
                        _ => {}
                    };
                }
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
            h if Icon::matches_handle(h) => {
                if let Some(i) = component_as_ref::<Icon>(component) {
                    return match i.icon_name().get() {
                        None => None,
                        _ => Some(html! { span class="icon" { i class=[i.classes().get()] {} } }),
                    };
                }
                None
            }
            _ => None,
        }
    }
}
