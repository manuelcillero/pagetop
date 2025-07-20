use pagetop::prelude::*;

#[pagetop::test]
async fn prepare_markup_is_empty() {
    let _app = service::test::init_service(Application::new().test()).await;

    assert!(PrepareMarkup::None.is_empty());

    assert!(PrepareMarkup::Text(String::from("")).is_empty());
    assert!(!PrepareMarkup::Text(String::from("x")).is_empty());

    assert!(PrepareMarkup::Escaped(String::new()).is_empty());
    assert!(!PrepareMarkup::Escaped("a".into()).is_empty());

    assert!(PrepareMarkup::With(html! {}).is_empty());
    assert!(!PrepareMarkup::With(html! { span { "!" } }).is_empty());
}
