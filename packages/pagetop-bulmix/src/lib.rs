use pagetop::prelude::*;

static_files!(bulmix);

pub struct Bulmix;

impl PackageTrait for Bulmix {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bulmix)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        service_for_static_files!(scfg, bulmix => "/bulmix");
    }
}

impl ThemeTrait for Bulmix {
    /*
        #[rustfmt::skip]
        fn builtin_classes(&self, builtin: ThemeBuiltInClasses) -> Option<String> {
            match builtin {
                ThemeBuiltInClasses::BodyWrapper     => Some(String::from("container")),
                ThemeBuiltInClasses::FlexWrapper  => Some(String::from("container")),
                ThemeBuiltInClasses::RegionContainer => Some(String::from("container")),
                _ => Some(builtin.to_string()),
            }
        }
    */
    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .alter_assets(AssetsOp::AddStyleSheet(
                StyleSheet::at("/bulmix/css/bulma.min.css")
                    .with_version("0.9.4")
                    .with_weight(-99),
            ))
            .alter_assets(AssetsOp::AddBaseAssets)
            .alter_assets(AssetsOp::AddStyleSheet(
                StyleSheet::at("/bulmix/css/styles.css").with_version("0.0.1"),
            ));
    }

    #[rustfmt::skip]
    fn before_prepare_component(&self, component: &mut dyn ComponentTrait, _cx: &mut Context) {
        if let Some(i) = component.downcast_mut::<Icon>() {
            i.alter_classes(
                ClassesOp::Replace(i.font_size().to_string()),
                with_font(i.font_size()),
            );
        } else if let Some(b) = component.downcast_mut::<Button>() {
            b.alter_classes(ClassesOp::Replace("button__tap".to_owned()), "button");
            b.alter_classes(
                ClassesOp::Replace(b.style().to_string()),
                match b.style() {
                    StyleBase::Default => "is-primary",
                    StyleBase::Info    => "is-info",
                    StyleBase::Success => "is-success",
                    StyleBase::Warning => "is-warning",
                    StyleBase::Danger  => "is-danger",
                    StyleBase::Light   => "is-light",
                    StyleBase::Dark    => "is-dark",
                    StyleBase::Link    => "is-text",
                },
            );
            b.alter_classes(
                ClassesOp::Replace(b.font_size().to_string()),
                with_font(b.font_size()),
            );
        } else if let Some(h) = component.downcast_mut::<Heading>() {
            match h.size() {
                HeadingSize::Subtitle => {
                    h.alter_classes(ClassesOp::Replace(h.size().to_string()), "subtitle")
                }
                _ => h.alter_classes(ClassesOp::Add, "title"),
            };
        } else if let Some(p) = component.downcast_mut::<Paragraph>() {
            p.alter_classes(ClassesOp::Add, "block");
            p.alter_classes(
                ClassesOp::Replace(p.font_size().to_string()),
                with_font(p.font_size()),
            );
        }
    }

    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        _cx: &mut Context,
    ) -> Option<Markup> {
        if let Some(i) = component.downcast_ref::<Icon>() {
            return match i.icon_name().get() {
                None => None,
                _ => Some(html! { span class="icon" { i class=[i.classes().get()] {} } }),
            };
        } else {
            None
        }
    }
}

#[rustfmt::skip]
fn with_font(font_size: &FontSize) -> String {
    String::from(match font_size {
        FontSize::ExtraLarge => "is-size-1",
        FontSize::XxLarge    => "is-size-2",
        FontSize::XLarge     => "is-size-3",
        FontSize::Large      => "is-size-4",
        FontSize::Medium     => "is-size-5",
        _ => "",
    })
}
