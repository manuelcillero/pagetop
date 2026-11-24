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

// A row button that opens a shared dialog (as `pagetop_htmx::hx_confirm::confirm()` prepares it)
// must open a Bootstrap modal, and its hidden label must use the class Bootstrap hides.
#[pagetop::test]
async fn a_row_button_opens_a_bootstrap_modal_with_its_hidden_label() {
    let mut cx =
        Context::new(web::test::TestRequest::get().to_http_request()).with_theme(&Bootsier);
    let html = bs::Button::delete()
        .with_hidden_label(Lc::n("Admins"))
        .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
        .with_prop(PropsOp::set("data-dialog-target", "#hx-confirm-dialog"))
        .render(&mut cx)
        .await
        .into_string();
    assert!(html.contains(r#"data-bs-toggle="modal""#), "{html}");
    assert!(
        html.contains(r##"data-bs-target="#hx-confirm-dialog""##),
        "{html}"
    );
    assert!(!html.contains("data-dialog-"), "{html}");
    assert!(
        html.contains(r#"<span class="visually-hidden"> Admins</span>"#),
        "{html}"
    );
}
