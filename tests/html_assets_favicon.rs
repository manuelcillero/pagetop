use pagetop::prelude::*;

// Empty icon declared when the page has none.
const DEFAULT_ICON: &str = r#"<link rel="icon" href="data:,">"#;

#[pagetop::test]
async fn pages_without_favicon_declare_an_empty_icon() {
    // Without any declared icon, browsers would request `/favicon.ico` on every page.
    let assets = Context::default().render_assets().into_string();
    assert!(assets.contains(DEFAULT_ICON));
}

#[pagetop::test]
async fn a_declared_favicon_replaces_the_default_icon() {
    let mut cx = Context::default().with_assets(Favicon::new().with_icon("/app/icon.png"));
    let assets = cx.render_assets().into_string();
    assert!(assets.contains("/app/icon.png"));
    assert!(!assets.contains(DEFAULT_ICON));
}
