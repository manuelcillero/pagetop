// Verifies that a `Navbar` with its content in an `Offcanvas` renders a working panel: the toggle
// button must point to the `id` of the panel, and the panel must carry its own placement class.

use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

fn cx() -> Context {
    Context::default().with_theme(&pagetop_bootsier::Bootsier)
}

fn panel(placement: bs::offcanvas::Placement) -> bs::Offcanvas {
    bs::Offcanvas::new().with_placement(placement)
}

#[pagetop::test]
async fn offcanvas_button_points_to_the_panel_id() {
    let brand = || Brand::new().with_title(Lc::n("PageTop"));
    let navbars = [
        bs::Navbar::offcanvas(panel(bs::offcanvas::Placement::Start)),
        bs::Navbar::offcanvas_brand_left(brand(), panel(bs::offcanvas::Placement::Start)),
        bs::Navbar::offcanvas_brand_right(brand(), panel(bs::offcanvas::Placement::Start)),
    ];

    for mut navbar in navbars {
        navbar = navbar.with_item(bs::navbar::Item::nav(
            bs::Nav::new().with_item(bs::nav::Item::link(Lc::n("Home"), "/")),
        ));
        let html = navbar.render(&mut cx()).await.into_string();

        // The panel is rendered by the navbar, so the navbar must set its `id` up.
        let target = html
            .split(r##"data-bs-target="#"##)
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .unwrap_or_default();
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
    ] {
        let mut navbar = bs::Navbar::offcanvas(panel(placement)).with_item(bs::navbar::Item::nav(
            bs::Nav::new().with_item(bs::nav::Item::link(Lc::n("Home"), "/")),
        ));
        let html = navbar.render(&mut cx()).await.into_string();

        // Without them Bootstrap would show the panel inline instead of hiding and placing it.
        assert!(
            html.contains(&format!(r#"class="offcanvas {class}""#)),
            "missing `offcanvas {class}` in {html}"
        );
    }
}
