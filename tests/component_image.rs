use pagetop::prelude::*;

async fn render(mut image: Image) -> String {
    image.render(&mut Context::default()).await.into_string()
}

fn logo() -> Image {
    Image::with(image::Source::logo(PageTopSvg::Color))
}

// **< Logo size >**********************************************************************************

#[pagetop::test]
async fn logo_without_size_gets_the_default_dimensions() {
    let html = render(logo()).await;

    assert!(html.contains(r#"style="width: 1.25rem; height: 1.25rem""#));
}

#[pagetop::test]
async fn logo_with_only_width_does_not_get_a_default_height() {
    let html = render(logo().with_size(image::Size::Width(UnitValue::Px(320)))).await;

    assert!(html.contains(r#"style="width: 320px""#));
    assert!(!html.contains("height"));
}

#[pagetop::test]
async fn logo_with_only_height_does_not_get_a_default_width() {
    let html = render(logo().with_size(image::Size::Height(UnitValue::Px(40)))).await;

    assert!(html.contains(r#"style="height: 40px""#));
    assert!(!html.contains("width:"));
}

#[pagetop::test]
async fn logo_with_a_percentage_keeps_a_single_dimension() {
    // A percentage height would refer to the container height, not to the width.
    let html = render(logo().with_size(image::Size::Width(UnitValue::RelPct(33.0)))).await;

    assert!(html.contains(r#"style="width: 33%""#));
    assert!(!html.contains("height"));
}

#[pagetop::test]
async fn logo_with_both_dimensions_uses_them() {
    let both = render(logo().with_size(image::Size::Both(UnitValue::Px(36)))).await;
    let different = render(logo().with_size(image::Size::Dimensions(
        UnitValue::Px(10),
        UnitValue::Px(20),
    )))
    .await;

    assert!(both.contains(r#"style="width: 36px; height: 36px""#));
    // Both dimensions are set explicitly, so the default is not applied.
    assert!(different.contains(r#"style="width: 10px; height: 20px""#));
}

#[pagetop::test]
async fn logo_keeps_a_custom_style_instead_of_the_default_dimensions() {
    let width = render(logo().with_prop(PropsOp::add_style("width", "3em"))).await;
    let both = render(
        logo()
            .with_prop(PropsOp::add_style("width", "3em"))
            .with_prop(PropsOp::add_style("height", "4em")),
    )
    .await;

    assert!(width.contains(r#"style="width: 3em""#));
    assert!(!width.contains("height"));
    assert!(both.contains(r#"style="width: 3em; height: 4em""#));
    assert!(!both.contains("1.25rem"));
}

#[pagetop::test]
async fn logo_with_a_size_without_value_gets_the_default_dimensions() {
    // `UnitValue::None` writes no declaration, so no dimension is actually set.
    for size in [
        image::Size::Width(UnitValue::None),
        image::Size::Height(UnitValue::None),
        image::Size::Both(UnitValue::None),
        image::Size::Dimensions(UnitValue::None, UnitValue::None),
    ] {
        let html = render(logo().with_size(size)).await;

        assert!(html.contains(r#"style="width: 1.25rem; height: 1.25rem""#));
    }
}

// **< Other sources >******************************************************************************

#[pagetop::test]
async fn responsive_image_without_size_has_no_style() {
    let html = render(Image::with(image::Source::responsive("/photo.jpg"))).await;

    assert!(html.starts_with(r#"<img src="/photo.jpg""#));
    assert!(!html.contains("style="));
}

#[pagetop::test]
async fn responsive_image_with_only_width_does_not_get_a_height() {
    let html = render(
        Image::with(image::Source::responsive("/photo.jpg"))
            .with_size(image::Size::Width(UnitValue::Px(320))),
    )
    .await;

    assert!(html.contains(r#"style="width: 320px""#));
    assert!(!html.contains("height"));
}

#[pagetop::test]
async fn responsive_image_with_a_size_without_value_has_no_style() {
    let html = render(
        Image::with(image::Source::responsive("/photo.jpg"))
            .with_size(image::Size::Width(UnitValue::None)),
    )
    .await;

    assert!(!html.contains("style="));
}

// **< Alternative text >***************************************************************************

#[pagetop::test]
async fn logo_alternative_is_an_accessible_label_or_hides_the_svg() {
    let labeled = render(logo().with_alternative(Lc::n("PageTop"))).await;
    let decorative = render(logo()).await;

    assert!(labeled.contains(r#"role="img" aria-label="PageTop""#));
    assert!(decorative.contains(r#"aria-hidden="true""#));
    assert!(!decorative.contains("aria-label"));
}
