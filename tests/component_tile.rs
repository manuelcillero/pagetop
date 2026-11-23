use pagetop::prelude::*;

async fn render(mut tile: Tile) -> String {
    tile.render(&mut Context::default()).await.into_string()
}

enum TestRoute {
    Open,
    Signed,
}

impl Endpoint for TestRoute {
    fn path(&self) -> CowStr {
        match self {
            Self::Open => "/open".into(),
            Self::Signed => "/signed".into(),
        }
    }

    fn access(&self) -> Access {
        match self {
            Self::Open => Access::Public,
            Self::Signed => Access::Authenticated,
        }
    }
}

#[pagetop::test]
async fn tile_is_a_link_with_its_title_and_accent() {
    let html = render(Tile::link(Lc::n("Orders"), "/orders")).await;

    assert!(
        html.starts_with(r#"<a href="/orders" class="tile accent-blue">"#),
        "{html}"
    );
    assert!(html.contains(r#"<span class="tile-title">Orders</span>"#));
    assert!(html.ends_with("</a>"));

    let html = render(Tile::link(Lc::n("Orders"), "/orders").with_accent(Accent::Orange)).await;
    assert!(html.contains(r#"class="tile accent-orange""#));
}

#[pagetop::test]
async fn description_icon_and_image_are_optional() {
    let html = render(Tile::link(Lc::n("Orders"), "/orders")).await;
    assert!(!html.contains("tile-description"));
    assert!(!html.contains("tile-icon"));
    assert!(!html.contains("<img"));

    let html = render(
        Tile::link(Lc::n("Orders"), "/orders")
            .with_description(Lc::n("Check the orders"))
            .with_icon(Icon::with(icon::Core::Check))
            .with_image(image::Source::plain("/img/orders.jpg")),
    )
    .await;
    assert!(html.contains(r#"<span class="tile-description">Check the orders</span>"#));
    assert!(html.contains(r#"<span class="tile-icon"><svg"#));
    assert!(html.contains(r#"src="/img/orders.jpg""#));
    assert!(html.contains(r#"class="image tile-image""#));
}

#[pagetop::test]
async fn background_image_is_decorative() {
    let html = render(
        Tile::link(Lc::n("Orders"), "/orders").with_image(image::Source::plain("/img/orders.jpg")),
    )
    .await;
    assert!(html.contains(r#"alt="""#));

    // Also when the image comes with an alternative text.
    let html = render(
        Tile::link(Lc::n("Orders"), "/orders").with_image(
            Image::with(image::Source::plain("/img/orders.jpg"))
                .with_alternative(Lc::n("Boxes in a warehouse")),
        ),
    )
    .await;
    assert!(html.contains(r#"alt="""#), "{html}");
    assert!(!html.contains("Boxes in a warehouse"));
}

#[pagetop::test]
async fn blank_tile_opens_in_a_new_window() {
    let html = render(Tile::link(Lc::n("Docs"), "https://docs.rs")).await;
    assert!(!html.contains("target="));

    let html = render(Tile::link_blank(Lc::n("Docs"), "https://docs.rs")).await;
    assert!(html.starts_with(r#"<a href="https://docs.rs" "#), "{html}");
    assert!(html.contains(r#"target="_blank""#));
    assert!(html.contains(r#"rel="noopener noreferrer""#));

    // Disabled, it has no link to open.
    let html = render(Tile::link_blank(Lc::n("Docs"), "https://docs.rs").with_disabled(true)).await;
    assert!(!html.contains("target="));
    assert!(!html.contains("rel="));

    // Neither without a route.
    let html = render(Tile::link_blank(Lc::n("Docs"), Route::default())).await;
    assert!(!html.contains("target="), "{html}");
    assert!(!html.contains("rel="));
}

#[pagetop::test]
async fn disabled_tile_has_no_link() {
    let html = render(Tile::link(Lc::n("Orders"), "/orders").with_disabled(true)).await;
    assert!(
        html.starts_with(r#"<a aria-disabled="true" class="tile accent-blue disabled">"#),
        "{html}"
    );
    assert!(!html.contains("href"));
}

#[pagetop::test]
async fn tile_is_only_shown_with_access_to_its_route() {
    let html = render(Tile::link(Lc::n("Open"), TestRoute::Open)).await;
    assert!(html.contains(r#"href="/open""#));

    // Without a request nobody is signed in.
    assert!(
        render(Tile::link(Lc::n("Signed"), TestRoute::Signed))
            .await
            .is_empty()
    );
}

#[pagetop::test]
async fn tile_keeps_a_target_set_with_props() {
    let html =
        render(Tile::link(Lc::n("Orders"), "/orders").with_prop(PropsOp::set("target", "_top")))
            .await;
    assert!(html.contains(r#"target="_top""#), "{html}");
}
