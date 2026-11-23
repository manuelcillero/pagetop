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
