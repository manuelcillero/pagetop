use pagetop::prelude::*;

#[pagetop::test]
async fn is_not_rendered_when_empty() {
    let mut container = Container::new();
    let html = container.render(&mut Context::default()).await;

    assert!(html.is_empty());
}

#[pagetop::test]
async fn default_kind_renders_a_div_element() {
    let mut container = Container::new().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.starts_with("<div"));
    assert!(html.ends_with("</div>"));
}

#[pagetop::test]
async fn main_kind_renders_a_main_element() {
    let mut container = Container::main().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.starts_with("<main"));
    assert!(html.ends_with("</main>"));
}
