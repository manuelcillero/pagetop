use pagetop::prelude::*;

static_files!(bulmix);

pub struct Bulmix;

impl PackageTrait for Bulmix {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bulmix)
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::theme::BeforePrepare::<Icon>::new(&Self, before_prepare_icon),
            action::theme::BeforePrepare::<Button>::new(&Self, before_prepare_button),
            action::theme::BeforePrepare::<Heading>::new(&Self, before_prepare_heading),
            action::theme::BeforePrepare::<Paragraph>::new(&Self, before_prepare_paragraph),
            action::theme::RenderComponent::<Icon>::new(&Self, render_icon),
        ]
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
}

fn before_prepare_icon(i: &mut Icon, _cx: &mut Context) {
    i.alter_classes(
        ClassesOp::Replace(i.font_size().to_string()),
        with_font(i.font_size()),
    );
}

#[rustfmt::skip]
fn before_prepare_button(b: &mut Button, _cx: &mut Context) {
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
}

#[rustfmt::skip]
fn before_prepare_heading(h: &mut Heading, _cx: &mut Context) {
    match h.size() {
        HeadingSize::Subtitle => {
            h.alter_classes(ClassesOp::Replace(h.size().to_string()), "subtitle")
        }
        _ => h.alter_classes(ClassesOp::Add, "title"),
    };
}

fn before_prepare_paragraph(p: &mut Paragraph, _cx: &mut Context) {
    p.alter_classes(ClassesOp::Add, "block");
    p.alter_classes(
        ClassesOp::Replace(p.font_size().to_string()),
        with_font(p.font_size()),
    );
}

fn render_icon(i: &Icon, _cx: &mut Context) -> Option<Markup> {
    return match i.icon_name().get() {
        None => None,
        _ => Some(html! { span class="icon" { i class=[i.classes().get()] {} } }),
    };
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
