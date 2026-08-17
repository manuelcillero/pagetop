use pagetop::prelude::*;

fn request() -> HttpRequest {
    web::test::TestRequest::get().to_http_request()
}

#[pagetop::test]
async fn title_and_description_are_absent_by_default() {
    let page = Page::new(request());

    assert_eq!(page.title(), None);
    assert_eq!(page.description(), None);
}

#[pagetop::test]
async fn title_and_description_reflect_the_values_set() {
    let page = Page::new(request())
        .with_title(Lc::n("Dashboard"))
        .with_description(Lc::n("Overview of recent activity"));

    assert_eq!(page.title(), Some("Dashboard".to_string()));
    assert_eq!(
        page.description(),
        Some("Overview of recent activity".to_string())
    );
}

#[pagetop::test]
async fn lc_none_clears_a_previously_set_title_and_description() {
    let page = Page::new(request())
        .with_title(Lc::n("Dashboard"))
        .with_description(Lc::n("Overview of recent activity"))
        .with_title(Lc::none())
        .with_description(Lc::none());

    assert_eq!(page.title(), None);
    assert_eq!(page.description(), None);
}
