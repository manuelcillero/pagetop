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
