use pagetop::prelude::*;

use std::borrow::Cow;

// `button_delete` is a core translation: "Delete" in en-US and "Eliminar" in es-ES.
fn delete() -> Lc {
    Lc::l("button_delete")
}

fn cx_with_lang(lang: &str) -> Context {
    Context::default().with_langid(&Locale::resolve(lang))
}

fn render(props: &Props, cx: &mut Context) -> String {
    html! { button (props.unpack(cx)) {} }.into_string()
}

// **< Translation >********************************************************************************

#[pagetop::test]
async fn set_with_lc_translates_into_the_language_of_the_context() {
    let props = Props::default().with_prop(PropsOp::set("title", delete()));
    for (lang, text) in [("en-US", "Delete"), ("es-ES", "Eliminar")] {
        assert_eq!(
            render(&props, &mut cx_with_lang(lang)),
            format!(r#"<button title="{text}"></button>"#),
            "{lang}"
        );
    }
}

#[pagetop::test]
async fn set_with_lc_escapes_the_translation() {
    let props = Props::default().with_prop(PropsOp::set("title", Lc::n(r#"a "b" <c>"#)));
    assert_eq!(
        render(&props, &mut Context::default()),
        r#"<button title="a &quot;b&quot; &lt;c&gt;"></button>"#
    );
}

// `lookup()` leaves the arguments unescaped, so they are escaped once, when written.
#[pagetop::test]
async fn set_with_lc_escapes_the_arguments_once() {
    let mut cx = cx_with_lang("en-US");
    let text = Props::default().with_prop(PropsOp::set(
        "title",
        Lc::l("test_hello_user").with_arg("userName", r#"<a "b">&"#),
    ));
    assert_eq!(
        render(&text, &mut cx),
        r#"<button title="Hello, &lt;a &quot;b&quot;&gt;&amp;!"></button>"#
    );

    // Trusted markup is not inserted as markup in an attribute either.
    let markup = Props::default().with_prop(PropsOp::set(
        "title",
        Lc::l("test_hello_user").with_markup("userName", html! { b { "x" } }),
    ));
    assert_eq!(
        render(&markup, &mut cx),
        r#"<button title="Hello, &lt;b&gt;x&lt;/b&gt;!"></button>"#
    );
}

#[pagetop::test]
async fn set_with_lc_without_translation_leaves_the_attribute_out() {
    let props = Props::default()
        .with_prop(PropsOp::set("title", Lc::l("no_such_key")))
        .with_prop(PropsOp::set("aria-label", Lc::none()))
        .with_prop(PropsOp::set("data-x", "y"));
    assert_eq!(
        render(&props, &mut Context::default()),
        r#"<button data-x="y"></button>"#
    );
}

#[pagetop::test]
async fn set_with_lc_keeps_its_place_among_the_attributes() {
    let props = Props::default()
        .with_prop(PropsOp::set("hx-post", "/a"))
        .with_prop(PropsOp::set("title", delete()))
        .with_prop(PropsOp::set("hx-target", "#t"));
    assert_eq!(
        render(&props, &mut cx_with_lang("en-US")),
        r##"<button hx-post="/a" title="Delete" hx-target="#t"></button>"##
    );
}

// **< Combination with other operations >**********************************************************

#[pagetop::test]
async fn the_last_set_wins_whatever_its_kind() {
    let mut cx = cx_with_lang("en-US");
    let text_last = Props::default()
        .with_prop(PropsOp::set("title", delete()))
        .with_prop(PropsOp::set("title", "plain"));
    assert_eq!(
        render(&text_last, &mut cx),
        r#"<button title="plain"></button>"#
    );

    let lc_last = Props::default()
        .with_prop(PropsOp::set("title", "plain"))
        .with_prop(PropsOp::set("title", delete()));
    assert_eq!(
        render(&lc_last, &mut cx),
        r#"<button title="Delete"></button>"#
    );
}

#[pagetop::test]
async fn remove_and_rename_apply_to_translatable_attributes() {
    let mut cx = cx_with_lang("en-US");
    let removed = Props::default()
        .with_prop(PropsOp::set("title", delete()))
        .with_prop(PropsOp::remove("title"));
    assert_eq!(render(&removed, &mut cx), "<button></button>");

    let renamed = Props::default()
        .with_prop(PropsOp::set("title", delete()))
        .with_prop(PropsOp::rename("title", "aria-label"));
    assert_eq!(
        render(&renamed, &mut cx),
        r#"<button aria-label="Delete"></button>"#
    );

    // The target keeps its own value, as with text attributes.
    let kept = Props::default()
        .with_prop(PropsOp::set("aria-label", delete()))
        .with_prop(PropsOp::set("title", "plain"))
        .with_prop(PropsOp::rename("title", "aria-label"));
    assert_eq!(
        render(&kept, &mut cx),
        r#"<button aria-label="Delete"></button>"#
    );
}

#[pagetop::test]
async fn a_literal_attribute_wins_over_a_translatable_one() {
    let props = Props::default().with_prop(PropsOp::set("title", delete()));
    let mut cx = cx_with_lang("en-US");
    assert_eq!(
        html! { button title="literal" (props.unpack(&mut cx)) {} }.into_string(),
        r#"<button title="literal"></button>"#
    );
}

// **< Getters >************************************************************************************

#[pagetop::test]
async fn get_prop_returns_translatable_attributes_untranslated() {
    let props = Props::default().with_prop(PropsOp::set("title", delete()));
    assert!(matches!(props.get_prop("title"), Some(PropValue::Lc(_))));
    assert!(!props.is_attrs_empty());
    assert!(!props.is_empty());
}

#[pagetop::test]
async fn lookup_translates_into_the_given_language() {
    let props = Props::new("hx-get", "/items").with_prop(PropsOp::set("title", delete()));
    let es = cx_with_lang("es-ES");
    let en = cx_with_lang("en-US");
    let title = props.get_prop("title").unwrap();
    assert_eq!(title.lookup(&es).as_deref(), Some("Eliminar"));
    assert_eq!(title.lookup(&en).as_deref(), Some("Delete"));
    // Plain text is borrowed, not copied.
    let hx_get = props.get_prop("hx-get").unwrap().lookup(&es);
    assert!(matches!(hx_get, Some(Cow::Borrowed("/items"))));
}

#[pagetop::test]
async fn lookup_without_translation_returns_none() {
    let es = cx_with_lang("es-ES");
    assert!(PropValue::from(Lc::l("no_such_key")).lookup(&es).is_none());
    assert!(PropValue::from(Lc::none()).lookup(&es).is_none());
}

// **< Misuse >*************************************************************************************

#[pagetop::test]
#[should_panic(expected = "has its own PropsOp variants")]
async fn class_does_not_accept_a_translatable_value() {
    let _ = Props::default().with_prop(PropsOp::set("class", delete()));
}
