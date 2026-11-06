use pagetop::prelude::*;

fn request() -> HttpRequest {
    web::test::TestRequest::get().to_http_request()
}

// Sets the document description and metadata from its own context while it renders.
#[derive(Clone)]
struct Article;

#[async_trait]
impl Component for Article {
    fn new() -> Self {
        Self
    }

    fn setup(&mut self, cx: &mut Context) {
        cx.alter_description(Lc::n("Article summary"))
            .alter_metadata("author", String::from("Article author"))
            .alter_property("og:type", "article");
    }
}

#[pagetop::test]
async fn title_and_description_are_absent_by_default() {
    let page = Page::new(request());

    assert!(page.title().is_none());
    assert!(page.description().is_none());
}

#[pagetop::test]
async fn title_and_description_reflect_the_values_set() {
    let page = Page::new(request())
        .with_title(Lc::n("Dashboard"))
        .with_description(Lc::n("Overview of recent activity"));

    assert_eq!(page.title().lookup(&page), Some("Dashboard".to_string()));
    assert_eq!(
        page.description().lookup(&page),
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

    assert!(page.title().is_none());
    assert!(page.description().is_none());
}

#[pagetop::test]
async fn page_and_its_context_share_the_same_state() {
    let mut page = Page::new(request())
        .with_title(Lc::n("Dashboard"))
        .with_metadata("robots", "noindex");
    page.context_mut().alter_property("og:title", "Dashboard");

    let cx = page.context();
    assert_eq!(cx.title().lookup(cx), Some("Dashboard".to_string()));
    assert_eq!(cx.metadata(), &[("robots", CowStr::from("noindex"))]);
    assert_eq!(
        page.properties(),
        &[("og:title", CowStr::from("Dashboard"))]
    );
}

#[pagetop::test]
async fn components_can_set_the_description_and_metadata_of_the_page() {
    Application::new().await;

    let mut page = Page::new(request()).with_child(Article);
    let html = page.render().await.expect("page renders").into_string();

    assert!(html.contains(r#"<meta name="description" content="Article summary">"#));
    assert!(html.contains(r#"<meta name="author" content="Article author">"#));
    assert!(html.contains(r#"<meta property="og:type" content="article">"#));
}
