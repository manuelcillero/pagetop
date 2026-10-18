// The action registry is built once per test binary, so the custom layout lives in its own file.

use pagetop::prelude::*;
use pagetop_mail::outbox;
use pagetop_mail::prelude::*;

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_mail::Mail]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            RenderMailLayout::new(heavier_layout).with_weight(10),
            RenderMailLayout::new(custom_layout),
        ]
    }
}

fn custom_layout(cx: &Context, email: &Email, content: &Markup) -> Markup {
    html! {
        html lang=(cx.langid().to_string()) {
            body { h1 { (email.subject()) } div class="custom" { (content) } }
        }
    }
}

fn heavier_layout(_cx: &Context, _email: &Email, _content: &Markup) -> Markup {
    html! { "heavier layout must not be used" }
}

#[pagetop::test]
async fn lightest_registered_layout_replaces_the_default_one() {
    Application::prepare(&TestApp).await;

    Email::new()
        .with_to("custom-layout@example.com")
        .with_subject("Custom")
        .with_html(html! { p { "Content" } })
        .send()
        .await
        .unwrap();

    let sent = outbox::take_to("custom-layout@example.com");
    assert_eq!(
        sent[0].email().html().unwrap().as_str(),
        concat!(
            r#"<html lang="en-US"><body><h1>Custom</h1>"#,
            r#"<div class="custom"><p>Content</p></div></body></html>"#
        )
    );
}
