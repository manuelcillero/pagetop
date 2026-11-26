use pagetop::prelude::*;

async fn render(mut intro: Intro) -> String {
    intro.render(&mut Context::default()).await.into_string()
}

// The opening tag of the call to action button.
fn button_link(html: &str) -> &str {
    let start = html
        .find(r#"<a class="intro-button-link""#)
        .unwrap_or_else(|| panic!("the button is missing in {html}"));
    let end = start + html[start..].find('>').expect("unclosed tag");
    &html[start..=end]
}

#[pagetop::test]
async fn default_button_opens_in_a_new_window() {
    let html = render(Intro::default()).await;
    let link = button_link(&html);
    assert!(
        link.contains(r#"href="https://pagetop.cillero.es""#),
        "{link}"
    );
    assert!(link.contains(r#"target="_blank""#));
    assert!(link.contains(r#"rel="noopener noreferrer""#));
}

#[pagetop::test]
async fn button_opens_in_the_same_window() {
    let html = render(Intro::default().with_button(Some((Lc::n("Start"), "/start".into())))).await;
    let link = button_link(&html);
    assert!(link.contains(r#"href="/start""#), "{link}");
    assert!(!link.contains("target="));
    assert!(!link.contains("rel="));
}

#[pagetop::test]
async fn button_blank_opens_in_a_new_window() {
    let html =
        render(Intro::custom().with_button_blank(Some((Lc::n("Docs"), "https://docs.rs".into()))))
            .await;
    let link = button_link(&html);
    assert!(link.contains(r#"target="_blank""#), "{link}");
    assert!(link.contains(r#"rel="noopener noreferrer""#));
}

#[pagetop::test]
async fn default_shows_the_version_pills() {
    let html = render(Intro::default()).await;
    assert!(html.contains(r#"<ul class="intro-pills">"#), "{html}");
    assert!(html.contains(PAGETOP_VERSION), "{html}");
    assert!(html.contains("MIT/Apache-2.0"), "{html}");
}

#[pagetop::test]
async fn default_needs_no_script_nor_external_services() {
    let mut cx = Context::default();
    let html = Intro::default().render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();
    assert!(assets.contains("/pagetop/css/intro.min.css"), "{assets}");
    assert!(!assets.contains("<script"), "{assets}");
    assert!(
        !html.contains("shields.io") && !html.contains("crates.io/api"),
        "{html}"
    );
}

#[pagetop::test]
async fn release_date_pill_follows_the_changelog() {
    let html = Intro::version_pills(&Context::default()).into_string();
    // Version and license always; the release date only when `build.rs` found it.
    let expected = if env!("PAGETOP_RELEASE_DATE").is_empty() {
        2
    } else {
        3
    };
    assert_eq!(
        html.matches(r#"<li class="intro-pill">"#).count(),
        expected,
        "{html}"
    );
}

#[pagetop::test]
async fn custom_has_no_presentation_of_pagetop() {
    let html = render(Intro::custom()).await;
    assert!(!html.contains("intro-pills"), "{html}");
    assert!(!html.contains("intro-text-lead"), "{html}");
}

#[pagetop::test]
async fn title_and_slogan_go_in_their_own_spans() {
    let html = render(
        Intro::custom()
            .with_title(Lc::n("Title"))
            .with_slogan(Lc::n("Slogan")),
    )
    .await;
    assert!(
        html.contains(r#"<span class="intro-header-name">Title</span>"#),
        "{html}"
    );
    assert!(
        html.contains(r#"<span class="intro-header-slogan">Slogan</span>"#),
        "{html}"
    );
}

#[pagetop::test]
async fn header_body_is_omitted_without_title_slogan_or_button() {
    let html = render(Intro::custom()).await;
    assert!(!html.contains("intro-header-body"), "{html}");

    // With only a button, the header body shows it but has no heading.
    let html = render(Intro::custom().with_button(Some((Lc::n("Go"), "/go".into())))).await;
    assert!(html.contains("intro-header-body"), "{html}");
    assert!(!html.contains("<h1"), "{html}");
}
