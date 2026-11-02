use pagetop::prelude::*;

// Rendering regions looks up localized labels, so the localization subsystem must be loaded.
async fn setup() {
    Application::new().await;
}

fn request() -> HttpRequest {
    web::test::TestRequest::get().to_http_request()
}

/// Pushes a message while it renders, like a form that reports its own errors.
#[derive(AutoDefault, Clone)]
struct Pusher(&'static str);

#[async_trait]
impl Component for Pusher {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        cx.push_message(MessageLevel::Error, Lc::n(self.0));
        Ok(html! { p { "pusher-" (self.0) } })
    }
}

/// Deferred component that shows how many messages the context holds when it renders.
#[derive(Clone)]
struct Counter;

#[async_trait]
impl Component for Counter {
    fn new() -> Self {
        Self
    }

    fn is_deferred(&self) -> bool {
        true
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! { span { "count-" (cx.messages().len()) } })
    }
}

/// Deferred component that adds a stylesheet to the page while it renders.
#[derive(Clone)]
struct Styled;

#[async_trait]
impl Component for Styled {
    fn new() -> Self {
        Self
    }

    fn is_deferred(&self) -> bool {
        true
    }

    fn setup(&mut self, cx: &mut Context) {
        cx.alter_assets(StyleSheet::from("/deferred.css"));
    }
}

/// Renders a deferred component and throws its output away, so its mark never reaches the body.
#[derive(Clone)]
struct Discarder;

#[async_trait]
impl Component for Discarder {
    fn new() -> Self {
        Self
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Counter.render(cx).await;
        Ok(html! { "discarded" })
    }
}

async fn render(mut page: Page) -> String {
    page.render().await.expect("page renders").into_string()
}

#[pagetop::test]
async fn messages_pushed_while_rendering_are_shown_first_in_the_content() {
    setup().await;
    let page = Page::new(request())
        .with_child(Pusher("from-content"))
        .with_child_in(&CoreRegions::Footer, Pusher("from-footer"));
    let body = render(page).await;

    let messages = body.find(r#"class="messages""#).expect("messages");
    assert!(body.find("region-content").unwrap() < messages);
    assert!(messages < body.find("pusher-from-content").unwrap());
    assert!(messages < body.find("pusher-from-footer").unwrap());
    assert_eq!(body.matches("message message-error").count(), 2);
    assert!(!body.contains("<!-- placeholder"));
}

#[pagetop::test]
async fn messages_of_the_page_take_precedence_over_the_content_ones() {
    setup().await;
    let page = Page::new(request())
        .with_child(Pusher("shown-by-the-page"))
        .with_child(Messages::new())
        .with_child_in(&CoreRegions::Footer, Pusher("left-for-the-content"));
    let body = render(page).await;

    // Each message is shown once: by the page if it was already pushed, else by the deferred
    // `Messages` that opens the content, which shows up first although it renders last.
    assert_eq!(body.matches(r#"class="messages""#).count(), 2);
    assert_eq!(body.matches("message message-error").count(), 2);
    let content = body.find("region-content").unwrap();
    let left = body.find(">left-for-the-content<").unwrap();
    let shown = body.find(">shown-by-the-page<").unwrap();
    assert!(content < left && left < shown);
}

#[pagetop::test]
async fn deferred_messages_placed_before_the_content_take_them_all() {
    setup().await;
    let page = Page::new(request())
        .with_child_in(&CoreRegions::Header, Messages::new().with_deferred(true))
        .with_child(Pusher("from-content"))
        .with_child_in(&CoreRegions::Footer, Pusher("from-footer"));
    let body = render(page).await;

    // Deferred components render in order, so the one in the header goes before the content one.
    assert_eq!(body.matches(r#"class="messages""#).count(), 1);
    assert_eq!(body.matches("message message-error").count(), 2);
    let messages = body.find(r#"class="messages""#).unwrap();
    assert!(messages < body.find("region-content").unwrap());
}

#[pagetop::test]
async fn without_messages_only_the_mark_is_left() {
    setup().await;
    let body = render(Page::new(request()).with_child(Html::with(|_| html! { "Done" }))).await;

    // Nothing to show: the mark stays as an HTML comment, so the body is not copied again.
    assert!(!body.contains(r#"class="messages""#));
    let mark = body.find("<!-- placeholder Messages ").expect("mark left");
    assert!(body.find("region-content").unwrap() < mark);
    assert!(mark < body.find("Done").unwrap());
}

#[pagetop::test]
async fn deferred_component_sees_what_is_rendered_after_it() {
    setup().await;
    let page = Page::new(request())
        .with_child_in(&CoreRegions::Header, Counter)
        .with_child(Pusher("late"));
    let body = render(page).await;

    let count = body.find("count-1").expect("counter sees the late message");
    assert!(count < body.find("pusher-late").unwrap());
}

#[pagetop::test]
async fn assets_of_deferred_components_reach_the_head() {
    setup().await;
    let body = render(Page::new(request()).with_child(Styled)).await;

    let stylesheet = body
        .find("/deferred.css")
        .expect("stylesheet of the deferred component");
    assert!(stylesheet < body.find("</head>").unwrap());
}

#[pagetop::test]
async fn deferred_component_without_its_mark_is_not_shown() {
    setup().await;
    let body = render(Page::new(request()).with_child(Discarder)).await;

    assert!(body.contains("discarded"));
    assert!(!body.contains("count-"));
    assert!(!body.contains("<!-- placeholder Counter"));
}

#[pagetop::test]
async fn content_messages_render_at_once_outside_a_page() {
    setup().await;
    let mut cx = Context::default();
    cx.push_message(MessageLevel::Info, Lc::n("fragment"));
    let markup = layout::Region::content()
        .render(&mut cx)
        .await
        .into_string();

    assert!(markup.contains(r#"class="messages""#));
    assert!(markup.contains("message message-info"));
    assert!(!markup.contains("<!-- placeholder"));
    assert!(!cx.has_messages());
}

#[pagetop::test]
async fn deferred_component_renders_at_once_outside_a_page() {
    setup().await;
    let mut cx = Context::default();
    cx.push_message(MessageLevel::Info, Lc::n("now"));

    assert_eq!(
        Counter.render(&mut cx).await.into_string(),
        "<span>count-1</span>"
    );
}
