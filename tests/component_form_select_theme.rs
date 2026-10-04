// With more than one enabled theme. The single-theme case lives in its own test binary, because the
// theme registry is global to the process.

use pagetop::prelude::*;

struct Aurora;

impl Extension for Aurora {
    fn name(&self) -> Lc {
        Lc::n("Aurora")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Aurora)
    }
}

impl Theme for Aurora {}

async fn setup() {
    let _ = Application::prepare(&Aurora).await;
}

#[pagetop::test]
async fn themes_are_sorted_by_name_and_the_selected_one_is_marked() {
    setup().await;
    let mut field = form::SelectTheme::new()
        .with_name("theme")
        .with_selected("Basic");
    let html = field.render(&mut Context::default()).await.into_string();

    let aurora = html.find(r#"<option value="Aurora""#).expect("Aurora");
    let basic = html
        .find(r#"<option value="Basic" selected>"#)
        .expect("Basic");
    assert!(aurora < basic);
    assert!(!html.contains("disabled"));
}

#[pagetop::test]
async fn optional_field_offers_the_site_theme_first() {
    setup().await;
    let mut field = form::SelectTheme::new();
    let html = field.render(&mut Context::default()).await.into_string();

    let site = html.find(r#"<option value="" selected>Use the site theme: "#);
    let first_theme = html.find(r#"<option value="Aurora""#);
    assert!(site.is_some());
    assert!(site < first_theme);
}

#[pagetop::test]
async fn required_field_asks_to_choose_only_when_nothing_valid_is_selected() {
    setup().await;
    let mut empty = form::SelectTheme::new().with_required(true);
    let html = empty.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="" selected>Choose a theme...</option>"#));

    let mut chosen = form::SelectTheme::new()
        .with_required(true)
        .with_selected("Aurora");
    let html = chosen.render(&mut Context::default()).await.into_string();
    assert!(!html.contains(r#"value="""#));
}
