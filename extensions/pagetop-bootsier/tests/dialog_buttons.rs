// The usual buttons of a dialog keep working with the theme: Bootstrap closes a modal with
// `data-bs-dismiss`, so the core attribute must be renamed.

use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;
use pagetop_bootsier::theme::*;

#[pagetop::test]
async fn the_dismiss_button_closes_a_bootstrap_modal() {
    let mut cx =
        Context::new(web::test::TestRequest::get().to_http_request()).with_theme(&Bootsier);
    let html = bs::Button::dismiss().render(&mut cx).await.into_string();
    assert!(html.contains(r#"data-bs-dismiss="modal""#), "{html}");
    assert!(!html.contains("data-dialog-dismiss"), "{html}");
}

#[pagetop::test]
async fn save_and_new_is_reachable_through_the_facade() {
    let save_and_new = bs::button::SaveAndNew::default();
    assert!(!save_and_new.is_requested());
}
