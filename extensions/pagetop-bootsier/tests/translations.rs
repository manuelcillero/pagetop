// Verifies that the texts added by the theme on its own come translated from their own catalog.

use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;
use pagetop_bootsier::theme::*;

fn request() -> HttpRequest {
    web::test::TestRequest::get().to_http_request()
}

#[pagetop::test]
async fn a_dropdown_without_title_gets_the_translated_default() {
    let mut nav = Nav::new().with_item(bs::nav::Item::dropdown(
        Dropdown::new().with_item(dropdown::Item::link(Lc::n("x"), "/x")),
    ));
    let mut cx = Context::new(request())
        .with_theme(&Bootsier)
        .with_langid(&Locale::resolve("es-ES"));
    let html = nav.render(&mut cx).await.into_string();
    assert!(html.contains("Menú desplegable"), "{html}");
}

#[pagetop::test]
async fn the_confirmation_buttons_are_translated() {
    let html = Page::new(request())
        .with_theme(&Bootsier)
        .with_langid(&Locale::resolve("es-ES"))
        .render()
        .await
        .unwrap()
        .into_string();
    assert!(html.contains(r#"data-confirm-ok="Aceptar""#), "{html}");
    assert!(html.contains(r#"data-confirm-cancel="Cancelar""#), "{html}");
}
