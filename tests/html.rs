use pagetop::prelude::*;

#[pagetop::test]
async fn prepare_markup_render_none_is_empty_string() {
    assert_eq!(render(&PrepareMarkup::None), "");
}

#[pagetop::test]
async fn prepare_markup_render_escaped_escapes_html_and_ampersands() {
    let pm = PrepareMarkup::Escaped(String::from("<b>& \" ' </b>"));
    assert_eq!(render(&pm), "&lt;b&gt;&amp; &quot; ' &lt;/b&gt;");
}

#[pagetop::test]
async fn prepare_markup_render_raw_is_inserted_verbatim() {
    let pm = PrepareMarkup::Raw(String::from("<b>bold</b><script>1<2</script>"));
    assert_eq!(render(&pm), "<b>bold</b><script>1<2</script>");
}

#[pagetop::test]
async fn prepare_markup_render_with_keeps_structure() {
    let pm = PrepareMarkup::With(html! {
        h2 { "Sample title" }
        p { "This is a paragraph." }
    });
    assert_eq!(
        render(&pm),
        "<h2>Sample title</h2><p>This is a paragraph.</p>"
    );
}

#[pagetop::test]
async fn prepare_markup_does_not_double_escape_when_wrapped_in_html_macro() {
    // Escaped: dentro de `html!` no debe volver a escaparse.
    let escaped = PrepareMarkup::Escaped("<i>x</i>".into());
    let wrapped_escaped = html! { div { (escaped) } };
    assert_eq!(
        wrapped_escaped.into_string(),
        "<div>&lt;i&gt;x&lt;/i&gt;</div>"
    );

    // Raw: tampoco debe escaparse al integrarlo.
    let raw = PrepareMarkup::Raw("<i>x</i>".into());
    let wrapped_raw = html! { div { (raw) } };
    assert_eq!(wrapped_raw.into_string(), "<div><i>x</i></div>");

    // With: debe incrustar el Markup tal cual.
    let with = PrepareMarkup::With(html! { span.title { "ok" } });
    let wrapped_with = html! { div { (with) } };
    assert_eq!(
        wrapped_with.into_string(),
        "<div><span class=\"title\">ok</span></div>"
    );
}

#[pagetop::test]
async fn prepare_markup_unicode_is_preserved() {
    // Texto con acentos y emojis debe conservarse (salvo el escape HTML de signos).
    let esc = PrepareMarkup::Escaped("Hello, tomorrow coffee ☕ & donuts!".into());
    assert_eq!(render(&esc), "Hello, tomorrow coffee ☕ &amp; donuts!");

    // Raw debe pasar íntegro.
    let raw = PrepareMarkup::Raw("Title — section © 2025".into());
    assert_eq!(render(&raw), "Title — section © 2025");
}

#[pagetop::test]
async fn prepare_markup_is_empty_semantics() {
    assert!(PrepareMarkup::None.is_empty());

    assert!(PrepareMarkup::Escaped(String::new()).is_empty());
    assert!(PrepareMarkup::Escaped(String::from("")).is_empty());
    assert!(!PrepareMarkup::Escaped(String::from("x")).is_empty());

    assert!(PrepareMarkup::Raw(String::new()).is_empty());
    assert!(PrepareMarkup::Raw(String::from("")).is_empty());
    assert!(!PrepareMarkup::Raw("a".into()).is_empty());

    assert!(PrepareMarkup::With(html! {}).is_empty());
    assert!(!PrepareMarkup::With(html! { span { "!" } }).is_empty());

    // Ojo: espacios NO deberían considerarse vacíos (comportamiento actual).
    assert!(!PrepareMarkup::Escaped(" ".into()).is_empty());
    assert!(!PrepareMarkup::Raw(" ".into()).is_empty());
}

#[pagetop::test]
async fn prepare_markup_equivalence_between_render_and_inline_in_html_macro() {
    let cases = [
        PrepareMarkup::None,
        PrepareMarkup::Escaped("<b>x</b>".into()),
        PrepareMarkup::Raw("<b>x</b>".into()),
        PrepareMarkup::With(html! { b { "x" } }),
    ];

    for pm in cases {
        let rendered = render(&pm);
        let in_macro = html! { (pm) }.into_string();
        assert_eq!(
            rendered, in_macro,
            "The output of Render and (pm) inside html! must match"
        );
    }
}

// HELPERS *****************************************************************************************

fn render(x: &impl Render) -> String {
    x.render().into_string()
}
