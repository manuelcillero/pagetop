// Verifies how `UserName` renders when nobody can view the profile. Rendering the link needs a
// request with a session or with `user:view_profiles` granted, so it is not covered here.

use pagetop_user::prelude::*;

use pagetop::prelude::*;

#[pagetop::test]
async fn without_a_request_the_name_is_plain_text() {
    let mut cx = Context::default();
    let html = UserName::of(42, "Ana").render(&mut cx).await.into_string();

    assert_eq!(html, r#"<span class="user-name">Ana</span>"#);
}

#[pagetop::test]
async fn the_name_is_escaped() {
    let mut cx = Context::default();
    let html = UserName::of(42, "<b>Ana</b>")
        .render(&mut cx)
        .await
        .into_string();

    assert!(html.contains("&lt;b&gt;Ana&lt;/b&gt;"));
    assert!(!html.contains("<b>"));
}

#[pagetop::test]
async fn props_are_applied() {
    let mut cx = Context::default();
    let html = UserName::of(42, "Ana")
        .with_prop(PropsOp::add_classes("author"))
        .render(&mut cx)
        .await
        .into_string();

    assert!(html.contains(r#"class="user-name author""#));
}
