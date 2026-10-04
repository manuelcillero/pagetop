// With only the default theme enabled. See `component_form_select_theme.rs` for several themes.

use pagetop::prelude::*;

#[pagetop::test]
async fn single_theme_is_shown_disabled_and_selected() {
    Application::new().await;
    let mut field = form::SelectTheme::new().with_name("theme");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains("disabled"));
    assert!(html.contains(r#"<option value="Basic" selected>"#));
    assert!(!html.contains(r#"value="""#));
}
