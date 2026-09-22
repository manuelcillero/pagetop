// Verifies the rendering of the standalone `Offcanvas`: its accessible name comes from the title,
// and it is not rendered without content.

use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

fn cx() -> Context {
    Context::default().with_theme(&pagetop_bootsier::Bootsier)
}

fn content() -> Html {
    Html::with(|_| html! { p { "Content" } })
}

async fn render(mut offcanvas: bs::Offcanvas) -> String {
    offcanvas.render(&mut cx()).await.into_string()
}

#[pagetop::test]
async fn title_labels_the_panel() {
    let html = render(
        bs::Offcanvas::new()
            .with_id("panel")
            .with_title(Lc::n("Menu"))
            .with_child(content()),
    )
    .await;

    assert!(html.contains(r#"<h5 id="panel-label" class="offcanvas-title">Menu</h5>"#));
    assert!(html.contains(r#"aria-labelledby="panel-label""#));
}

#[pagetop::test]
async fn without_a_title_there_is_nothing_to_point_aria_labelledby_to() {
    let html = render(bs::Offcanvas::new().with_id("panel").with_child(content())).await;

    assert!(!html.contains("<h5"));
    assert!(!html.contains("aria-labelledby"));
    // The close button and the body are still there.
    assert!(html.contains(r##"data-bs-target="#panel""##));
    assert!(html.contains(r#"<div class="offcanvas-body"><p>Content</p></div>"#));
}

#[pagetop::test]
async fn is_not_rendered_without_content() {
    let html = render(bs::Offcanvas::new().with_title(Lc::n("Menu"))).await;

    assert!(html.is_empty());
}

#[pagetop::test]
async fn getters_of_the_options_return_their_values() {
    let offcanvas = bs::Offcanvas::new()
        .with_placement(bs::offcanvas::Placement::End)
        .with_backdrop(bs::offcanvas::Backdrop::Static)
        .with_body_scroll(bs::offcanvas::BodyScroll::Enabled)
        .with_visibility(bs::offcanvas::Visibility::Show);

    assert_eq!(offcanvas.placement(), bs::offcanvas::Placement::End);
    assert_eq!(offcanvas.backdrop(), bs::offcanvas::Backdrop::Static);
    assert_eq!(offcanvas.body_scroll(), bs::offcanvas::BodyScroll::Enabled);
    assert_eq!(offcanvas.visibility(), bs::offcanvas::Visibility::Show);
}
