use pagetop::prelude::*;

static_files!(bulmix);

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
            .alter_context(ContextOp::AddBaseAssets)
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/bulmix/css/styles.css").with_version("0.0.1"),
            ));
    }

    fn before_prepare_component(&self, component: &mut dyn ComponentTrait, _cx: &mut Context) {
        match component.handle() {
            COMPONENT_BASE_ICON => {
                if let Some(icon) = component_as_mut::<Icon>(component) {
                    match icon.font_size() {
                        FontSize::ExtraLarge => {
                            icon.alter_classes(
                                ClassesOp::Replace(icon.font_size().to_string()),
                                "is-size-1",
                            );
                        }
                        FontSize::XxLarge => {
                            icon.alter_classes(
                                ClassesOp::Replace(icon.font_size().to_string()),
                                "is-size-2",
                            );
                        }
                        FontSize::XLarge => {
                            icon.alter_classes(
                                ClassesOp::Replace(icon.font_size().to_string()),
                                "is-size-3",
                            );
                        }
                        FontSize::Large => {
                            icon.alter_classes(
                                ClassesOp::Replace(icon.font_size().to_string()),
                                "is-size-4",
                            );
                        }
                        FontSize::Medium => {
                            icon.alter_classes(
                                ClassesOp::Replace(icon.font_size().to_string()),
                                "is-size-5",
                            );
                        }
                        _ => {}
                    };
                }
            }
            COMPONENT_BASE_BUTTON => {
                if let Some(button) = component_as_mut::<Button>(component) {
                    match button.font_size() {
                        FontSize::ExtraLarge => {
                            button.alter_classes(
                                ClassesOp::Replace(button.font_size().to_string()),
                                "is-size-1",
                            );
                        }
                        FontSize::XxLarge => {
                            button.alter_classes(
                                ClassesOp::Replace(button.font_size().to_string()),
                                "is-size-2",
                            );
                        }
                        FontSize::XLarge => {
                            button.alter_classes(
                                ClassesOp::Replace(button.font_size().to_string()),
                                "is-size-3",
                            );
                        }
                        FontSize::Large => {
                            button.alter_classes(
                                ClassesOp::Replace(button.font_size().to_string()),
                                "is-size-4",
                            );
                        }
                        FontSize::Medium => {
                            button.alter_classes(
                                ClassesOp::Replace(button.font_size().to_string()),
                                "is-size-5",
                            );
                        }
                        _ => {}
                    };
                    match button.button_type() {
                        ButtonType::Link => {
                            button.alter_classes(
                                ClassesOp::Replace(button.button_type().to_string()),
                                "button is-text",
                            );
                        }
                        ButtonType::Primary => {
                            button.alter_classes(
                                ClassesOp::Replace(button.button_type().to_string()),
                                "button is-primary",
                            );
                        }
                    };
                }
            }
            COMPONENT_BASE_HEADING => {
                if let Some(heading) = component_as_mut::<Heading>(component) {
                    match heading.display() {
                        HeadingDisplay::Subtitle => heading.alter_classes(
                            ClassesOp::Replace(heading.display().to_string()),
                            "subtitle",
                        ),
                        _ => heading.alter_classes(ClassesOp::Add, "title"),
                    };
                }
            }
            COMPONENT_BASE_PARAGRAPH => {
                if let Some(paragraph) = component_as_mut::<Paragraph>(component) {
                    paragraph.alter_classes(ClassesOp::Add, "block");
                    match paragraph.font_size() {
                        FontSize::ExtraLarge => {
                            paragraph.alter_classes(
                                ClassesOp::Replace(paragraph.font_size().to_string()),
                                "is-size-1",
                            );
                        }
                        FontSize::XxLarge => {
                            paragraph.alter_classes(
                                ClassesOp::Replace(paragraph.font_size().to_string()),
                                "is-size-2",
                            );
                        }
                        FontSize::XLarge => {
                            paragraph.alter_classes(
                                ClassesOp::Replace(paragraph.font_size().to_string()),
                                "is-size-3",
                            );
                        }
                        FontSize::Large => {
                            paragraph.alter_classes(
                                ClassesOp::Replace(paragraph.font_size().to_string()),
                                "is-size-4",
                            );
                        }
                        FontSize::Medium => {
                            paragraph.alter_classes(
                                ClassesOp::Replace(paragraph.font_size().to_string()),
                                "is-size-5",
                            );
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
            COMPONENT_BASE_ICON => {
                if let Some(icon) = component_as_ref::<Icon>(component) {
                    return match icon.icon_name().get() {
                        None => None,
                        _ => {
                            Some(html! { span class="icon" { i class=[icon.classes().get()] {} } })
                        }
                    };
                }
                None
            }
            _ => None,
        }
    }
}
