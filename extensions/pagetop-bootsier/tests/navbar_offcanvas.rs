// Verifies that a `Navbar` with its content in a `navbar::Panel` renders a working side panel: the
// toggle button points to the panel, the panel carries its own classes and attributes, and the
// items of the navbar are its only content.

use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

fn cx() -> Context {
    Context::default().with_theme(&pagetop_bootsier::Bootsier)
}

fn panel(placement: bs::offcanvas::Placement) -> bs::navbar::Panel {
    bs::navbar::Panel::new().with_placement(placement)
}

fn home() -> bs::navbar::Item {
    bs::navbar::Item::nav(bs::Nav::new().with_item(bs::nav::Item::link(Lc::n("Home"), "/")))
}

async fn render(navbar: Navbar) -> String {
    let mut navbar = navbar.with_item(home());
    navbar.render(&mut cx()).await.into_string()
}

// The `id` the toggle button points to.
fn target(html: &str) -> &str {
    html.split(r##"data-bs-target="#"##)
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .unwrap_or_default()
}

#[pagetop::test]
async fn offcanvas_button_points_to_the_panel_id() {
    let brand = || Brand::new().with_title(Lc::n("PageTop"));
    let start = bs::offcanvas::Placement::Start;
    let navbars = [
        bs::Navbar::offcanvas(panel(start)),
        bs::Navbar::offcanvas_brand_left(brand(), panel(start)),
        bs::Navbar::offcanvas_brand_right(brand(), panel(start)),
    ];

    for navbar in navbars {
        let html = render(navbar).await;

        // The panel is rendered by the navbar, which derives its `id` from its own.
        let target = target(&html);
        assert!(!target.is_empty(), "the button has no target in {html}");
        assert!(html.contains(&format!(r#"aria-controls="{target}""#)));
        assert!(
            html.contains(&format!(r#"id="{target}""#)),
            "no panel with id `{target}` in {html}"
        );
    }
}

#[pagetop::test]
async fn offcanvas_panel_gets_its_own_classes() {
    for (placement, class) in [
        (bs::offcanvas::Placement::Start, "offcanvas-start"),
        (bs::offcanvas::Placement::End, "offcanvas-end"),
        (bs::offcanvas::Placement::Top, "offcanvas-top"),
        (bs::offcanvas::Placement::Bottom, "offcanvas-bottom"),
    ] {
        let html = render(bs::Navbar::offcanvas(panel(placement))).await;

        // Without them Bootstrap would show the panel inline instead of hiding and placing it.
        assert!(
            html.contains(&format!(r#"class="offcanvas {class}""#)),
            "missing `offcanvas {class}` in {html}"
        );
    }
}

#[pagetop::test]
async fn offcanvas_panel_follows_the_expand_of_the_navbar() {
    let navbar = bs::Navbar::offcanvas(panel(bs::offcanvas::Placement::Start));
    let html = render(navbar.with_expand(Breakpoint::Lg)).await;

    // Bootstrap shows the panel inline from `navbar-expand-lg`; the panel has no breakpoint of its
    // own that could contradict it, and it is not open on load.
    assert!(html.contains("navbar-expand-lg"));
    assert!(html.contains(r#"class="offcanvas offcanvas-start""#));
    assert!(!html.contains("offcanvas-lg") && !html.contains("offcanvas-md"));
}

#[pagetop::test]
async fn offcanvas_panel_title_labels_the_panel() {
    let start = bs::offcanvas::Placement::Start;

    let html = render(bs::Navbar::offcanvas(
        bs::navbar::Panel::new()
            .with_title(Lc::n("Main menu"))
            .with_placement(start),
    ))
    .await;
    let target = target(&html);
    assert!(html.contains(&format!(
        r#"<h5 id="{target}-label" class="offcanvas-title">Main menu</h5>"#
    )));
    assert!(html.contains(&format!(r#"aria-labelledby="{target}-label""#)));

    // Without a title there is nothing to point `aria-labelledby` to.
    let html = render(bs::Navbar::offcanvas(panel(start))).await;
    assert!(!html.contains("<h5"));
    assert!(!html.contains("aria-labelledby"));
}

#[pagetop::test]
async fn offcanvas_panel_backdrop_and_body_scroll_are_attributes() {
    let panel = || bs::navbar::Panel::new();

    let html = render(bs::Navbar::offcanvas(panel())).await;
    assert!(!html.contains("data-bs-backdrop") && !html.contains("data-bs-scroll"));

    for (backdrop, value) in [
        (bs::offcanvas::Backdrop::Disabled, "false"),
        (bs::offcanvas::Backdrop::Static, "static"),
    ] {
        let html = render(bs::Navbar::offcanvas(panel().with_backdrop(backdrop))).await;
        assert!(html.contains(&format!(r#"data-bs-backdrop="{value}""#)));
    }

    let scrolling = panel().with_body_scroll(bs::offcanvas::BodyScroll::Enabled);
    let html = render(bs::Navbar::offcanvas(scrolling)).await;
    assert!(html.contains(r#"data-bs-scroll="true""#));
}

#[pagetop::test]
async fn offcanvas_panel_contains_the_items_once_and_can_be_closed() {
    let html = render(bs::Navbar::offcanvas(panel(bs::offcanvas::Placement::End))).await;
    let target = target(&html);

    // The items are the body of the panel and are not rendered anywhere else.
    assert_eq!(html.matches(">Home<").count(), 1);
    assert!(html.contains(r#"<div class="offcanvas-body"><ul"#));

    // The close button of the header points to the same panel.
    assert!(html.contains(&format!(
        r##"data-bs-dismiss="offcanvas" data-bs-target="#{target}""##
    )));
}
