use pagetop::prelude::*;

struct Ext;

impl Extension for Ext {}

#[pagetop::test]
async fn without_declared_pages_the_panel_is_not_mounted() {
    let app = Application::prepare(&Ext).await.test();
    for uri in ["/admin", "/admin/config"] {
        let request = web::test::TestRequest::get().uri(uri).to_request();
        let response = web::test::send_request(&app, request).await;
        assert_eq!(response.status().as_u16(), 404, "{uri}");
    }
}

#[pagetop::test]
async fn without_access_to_the_panel_its_pages_show_the_main_menu() {
    let _ = Application::prepare(&Ext).await;
    let navbar = Navbar::simple()
        .with_item(navbar::Item::nav(
            Nav::new()
                .with_item(nav::Item::link(Lc::n("Home"), "/home"))
                .with_renderable(|cx| !is_admin_panel(cx)),
        ))
        .with_item(navbar::Item::nav(Nav::admin()));
    // A page with the admin template that nobody can access, as an error 403 would be.
    let request = web::test::TestRequest::get()
        .uri("/admin")
        .to_http_request();
    let mut page = Page::admin(request).with_child_in(&CoreRegions::Header, navbar);
    let html = page.render().await.unwrap().into_string();

    assert!(html.contains(r#"href="/home""#), "{html}");
    assert!(!html.contains("nav-admin"), "{html}");
}
