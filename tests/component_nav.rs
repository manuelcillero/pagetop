use pagetop::prelude::*;

async fn render_item(mut item: nav::Item) -> String {
    item.render(&mut Context::default()).await.into_string()
}

#[pagetop::test]
async fn a_disabled_link_has_no_href() {
    let html = render_item(nav::Item::link(Lc::n("Shop"), "/shop").with_disabled(true)).await;
    assert!(html.contains(r#"class="nav-link disabled""#), "{html}");
    assert!(html.contains(r#"aria-disabled="true""#));
    assert!(!html.contains("href="));

    // Opening in a new window does not apply to a link that cannot be followed.
    let html =
        render_item(nav::Item::link_blank(Lc::n("Docs"), "https://docs.rs").with_disabled(true))
            .await;
    assert!(!html.contains("target="), "{html}");
    assert!(!html.contains("rel="));

    let html = render_item(nav::Item::link_blank(Lc::n("Docs"), "https://docs.rs")).await;
    assert!(html.contains(r#"target="_blank""#), "{html}");
}

#[pagetop::test]
async fn disabled_can_be_turned_off() {
    let html = render_item(
        nav::Item::link(Lc::n("Shop"), "/shop")
            .with_disabled(true)
            .with_disabled(false),
    )
    .await;
    assert!(html.contains(r#"href="/shop""#), "{html}");
    assert!(!html.contains("disabled"));
}

#[pagetop::test]
async fn disabled_does_not_apply_to_labels() {
    let label = render_item(nav::Item::label(Lc::n("Note"))).await;
    let disabled = render_item(nav::Item::label(Lc::n("Note")).with_disabled(true)).await;
    assert_eq!(label, disabled);
}
