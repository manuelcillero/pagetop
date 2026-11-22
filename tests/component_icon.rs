use pagetop::prelude::*;

async fn render(mut icon: Icon) -> String {
    icon.render(&mut Context::default()).await.into_string()
}

#[pagetop::test]
async fn icon_without_source_renders_nothing() {
    assert!(render(Icon::new()).await.is_empty());
    assert!(
        render(Icon::with(icon::Core::Check).with_source(None))
            .await
            .is_empty()
    );
}

#[pagetop::test]
async fn svg_icon_renders_an_inline_svg_scaled_to_the_font() {
    let html = render(Icon::with(icon::Core::Check)).await;

    assert!(html.starts_with("<svg"));
    assert!(html.contains(r#"class="icon""#));
    assert!(html.contains(r#"viewBox="0 0 16 16""#));
    assert!(html.contains(r#"width="1em" height="1em""#));
    assert!(html.contains("<path"));
}

#[pagetop::test]
async fn font_icon_renders_an_i_element_with_its_classes() {
    let html = render(Icon::with(icon::Source::font("my-icons my-icons-star"))).await;

    assert!(html.starts_with("<i"));
    assert!(html.contains(r#"class="icon my-icons my-icons-star""#));
    assert!(html.ends_with("</i>"));
}

#[pagetop::test]
async fn icon_without_label_is_decorative() {
    for icon in [
        Icon::with(icon::Core::Close),
        Icon::with(icon::Source::font("my-icons-star")),
    ] {
        let html = render(icon).await;

        assert!(html.contains(r#"aria-hidden="true""#));
        assert!(!html.contains("role="));
        assert!(!html.contains("aria-label"));
    }
}

#[pagetop::test]
async fn icon_with_a_blank_label_is_decorative() {
    let html = render(Icon::with(icon::Core::Close).with_label(Lc::n(" "))).await;

    assert!(html.contains(r#"aria-hidden="true""#), "{html}");
    assert!(!html.contains("role="), "{html}");
    assert!(!html.contains("aria-label"), "{html}");
}

#[pagetop::test]
async fn icon_with_label_is_announced_as_an_image() {
    for icon in [
        Icon::with(icon::Core::Close),
        Icon::with(icon::Source::font("my-icons-star")),
    ] {
        let html = render(icon.with_label(Lc::n("Close"))).await;

        assert!(html.contains(r#"role="img""#));
        assert!(html.contains(r#"aria-label="Close""#));
        assert!(!html.contains("aria-hidden"));
    }
}

#[pagetop::test]
async fn icon_size_sets_the_font_size() {
    let sized = render(Icon::with(icon::Core::Plus).with_size(UnitValue::RelRem(2.0))).await;
    let unsized_icon = render(Icon::with(icon::Core::Plus)).await;

    assert!(sized.contains(r#"style="font-size: 2rem""#));
    assert!(!unsized_icon.contains("style="));
}

#[pagetop::test]
async fn custom_classes_are_kept_after_the_icon_class() {
    let html =
        render(Icon::with(icon::Core::Minus).with_prop(PropsOp::add_classes("text-red"))).await;

    assert!(html.contains(r#"class="icon text-red""#));
}

#[pagetop::test]
async fn any_type_convertible_into_a_source_is_an_icon_set() {
    enum MyIcons {
        Dot,
    }

    impl From<MyIcons> for icon::Source {
        fn from(icon: MyIcons) -> Self {
            match icon {
                MyIcons::Dot => {
                    icon::Source::svg("0 0 10 10", html! { circle cx="5" cy="5" r="2" {} })
                }
            }
        }
    }

    let html = render(Icon::with(MyIcons::Dot)).await;

    assert!(html.contains(r#"viewBox="0 0 10 10""#));
    assert!(html.contains("<circle"));
}

#[pagetop::test]
async fn icon_intent_adds_the_theme_color_class() {
    let html = render(Icon::with(icon::Core::Check).with_intent(Intent::Success)).await;

    assert!(html.contains(r#"class="icon icon-success""#));
}

#[pagetop::test]
async fn button_renders_its_start_icon_before_the_label() {
    let mut button = Button::plain(Lc::n("Add")).with_icon_start(Icon::with(icon::Core::Plus));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("</svg> Add</button>"));
}

#[pagetop::test]
async fn icon_only_button_takes_its_name_from_the_icon_label() {
    let mut button = Button::plain(Lc::none())
        .with_icon_start(Icon::with(icon::Core::Close).with_label(Lc::n("Close")));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"aria-label="Close""#));
    // No separator without a label.
    assert!(html.contains("</svg></button>"));
}

#[pagetop::test]
async fn button_renders_its_end_icon_after_the_label() {
    let mut button =
        Button::plain(Lc::n("Next")).with_icon_end(Icon::with(icon::Core::ChevronRight));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("Next <svg"));
}

#[pagetop::test]
async fn button_separates_both_icons_from_the_label_with_one_space_each() {
    let start = || Icon::with(icon::Core::Plus).with_id("start");
    let end = || Icon::with(icon::Core::ChevronDown).with_id("end");

    let mut labeled = Button::plain(Lc::n("Add"))
        .with_icon_start(start())
        .with_icon_end(end());
    let mut unlabeled = Button::plain(Lc::none())
        .with_icon_start(start())
        .with_icon_end(end());
    let labeled = labeled.render(&mut Context::default()).await.into_string();
    let unlabeled = unlabeled
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(labeled.contains("</svg> Add <svg"));
    assert!(labeled.find(r#"id="start""#) < labeled.find(r#"id="end""#));
    assert!(unlabeled.contains("</svg> <svg"));
    assert!(!unlabeled.contains("  "));
}

#[pagetop::test]
async fn nav_link_renders_its_icons_around_the_label() {
    let mut item = nav::Item::link(Lc::n("Home"), "/")
        .with_icon_start(Icon::with(icon::Core::Check))
        .with_icon_end(Icon::with(icon::Core::ChevronRight));
    let html = item.render(&mut Context::default()).await.into_string();

    let start = html.find(r#"class="icon icon-start""#).unwrap();
    let label = html.find("Home").unwrap();
    let end = html.find(r#"class="icon icon-end""#).unwrap();
    assert!(start < label && label < end);
}

#[pagetop::test]
async fn nav_dropdown_title_renders_its_icons() {
    let mut item = nav::Item::dropdown(
        Dropdown::new()
            .with_title(Lc::n("More"))
            .with_item(dropdown::Item::link(Lc::n("One"), "/one")),
    )
    .with_icon_start(Icon::with(icon::Core::Plus));
    let html = item.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="icon icon-start""#));
}

#[pagetop::test]
async fn dropdown_items_render_their_icons_around_the_label() {
    for item in [
        dropdown::Item::label(Lc::n("Text")),
        dropdown::Item::link(Lc::n("Text"), "/"),
        dropdown::Item::button(Lc::n("Text")),
    ] {
        let mut item = item
            .with_icon_start(Icon::with(icon::Core::Check))
            .with_icon_end(Icon::with(icon::Core::ChevronRight));
        let html = item.render(&mut Context::default()).await.into_string();

        let start = html.find(r#"class="icon icon-start""#).unwrap();
        let label = html.find("Text").unwrap();
        let end = html.find(r#"class="icon icon-end""#).unwrap();
        assert!(start < label && label < end);
    }
}

#[pagetop::test]
async fn menu_item_without_icons_renders_as_before() {
    let mut item = dropdown::Item::link(Lc::n("Text"), "/");
    let html = item.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("<svg"));
    assert!(html.contains(">Text</a>"));
}

#[pagetop::test]
async fn core_icon_keeps_its_identity_until_rendered() {
    let icon = Icon::with(icon::Core::Gear);

    assert!(matches!(
        icon.source(),
        Some(icon::Source::Core(icon::Core::Gear))
    ));
    assert!(render(icon).await.starts_with("<svg"));
}
