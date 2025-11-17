use pagetop::prelude::*;

/// Componente mínimo para probar `PrepareMarkup` pasando por el ciclo real
/// de renderizado de componentes (`ComponentRender`).
#[derive(AutoDefault)]
struct TestPrepareComponent {
    pm: PrepareMarkup,
}

impl Component for TestPrepareComponent {
    fn new() -> Self {
        Self {
            pm: PrepareMarkup::None,
        }
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        self.pm.clone()
    }
}

impl TestPrepareComponent {
    fn render_pm(pm: PrepareMarkup) -> String {
        let mut c = TestPrepareComponent { pm };
        c.render(&mut Context::default()).into_string()
    }
}

#[pagetop::test]
async fn prepare_markup_none_is_empty_string() {
    assert_eq!(PrepareMarkup::None.into_string(), "");
}

#[pagetop::test]
async fn prepare_markup_escaped_escapes_html_and_ampersands() {
    let pm = PrepareMarkup::Escaped("<b>& \" ' </b>".to_string());
    assert_eq!(pm.into_string(), "&lt;b&gt;&amp; &quot; ' &lt;/b&gt;");
}

#[pagetop::test]
async fn prepare_markup_raw_is_inserted_verbatim() {
    let pm = PrepareMarkup::Raw("<b>bold</b><script>1<2</script>".to_string());
    assert_eq!(pm.into_string(), "<b>bold</b><script>1<2</script>");
}

#[pagetop::test]
async fn prepare_markup_with_keeps_structure() {
    let pm = PrepareMarkup::With(html! {
        h2 { "Sample title" }
        p  { "This is a paragraph." }
    });
    assert_eq!(
        pm.into_string(),
        "<h2>Sample title</h2><p>This is a paragraph.</p>"
    );
}

#[pagetop::test]
async fn prepare_markup_unicode_is_preserved() {
    // Texto con acentos y emojis debe conservarse (salvo el escape HTML de signos).
    let esc = PrepareMarkup::Escaped("Hello, tomorrow coffee ☕ & donuts!".into());
    assert_eq!(esc.into_string(), "Hello, tomorrow coffee ☕ &amp; donuts!");

    // Raw debe pasar íntegro.
    let raw = PrepareMarkup::Raw("Title — section © 2025".into());
    assert_eq!(raw.into_string(), "Title — section © 2025");
}

#[pagetop::test]
async fn prepare_markup_is_empty_semantics() {
    assert!(PrepareMarkup::None.is_empty());

    assert!(PrepareMarkup::Escaped(String::new()).is_empty());
    assert!(PrepareMarkup::Escaped("".to_string()).is_empty());
    assert!(!PrepareMarkup::Escaped("x".to_string()).is_empty());

    assert!(PrepareMarkup::Raw(String::new()).is_empty());
    assert!(PrepareMarkup::Raw("".to_string()).is_empty());
    assert!(!PrepareMarkup::Raw("a".into()).is_empty());

    assert!(PrepareMarkup::With(html! {}).is_empty());
    assert!(!PrepareMarkup::With(html! { span { "!" } }).is_empty());

    // Ojo: espacios NO deberían considerarse vacíos (comportamiento actual).
    assert!(!PrepareMarkup::Escaped(" ".into()).is_empty());
    assert!(!PrepareMarkup::Raw(" ".into()).is_empty());
}

#[pagetop::test]
async fn prepare_markup_does_not_double_escape_when_markup_is_reinjected_in_html_macro() {
    let mut cx = Context::default();

    // Escaped: dentro de `html!` no debe volver a escaparse.
    let mut comp = TestPrepareComponent {
        pm: PrepareMarkup::Escaped("<i>x</i>".into()),
    };
    let markup = comp.render(&mut cx); // Markup
    let wrapped_escaped = html! { div { (markup) } }.into_string();
    assert_eq!(wrapped_escaped, "<div>&lt;i&gt;x&lt;/i&gt;</div>");

    // Raw: tampoco debe escaparse al integrarlo.
    let mut comp = TestPrepareComponent {
        pm: PrepareMarkup::Raw("<i>x</i>".into()),
    };
    let markup = comp.render(&mut cx);
    let wrapped_raw = html! { div { (markup) } }.into_string();
    assert_eq!(wrapped_raw, "<div><i>x</i></div>");

    // With: debe incrustar el Markup tal cual.
    let mut comp = TestPrepareComponent {
        pm: PrepareMarkup::With(html! { span.title { "ok" } }),
    };
    let markup = comp.render(&mut cx);
    let wrapped_with = html! { div { (markup) } }.into_string();
    assert_eq!(wrapped_with, "<div><span class=\"title\">ok</span></div>");
}

#[pagetop::test]
async fn prepare_markup_equivalence_between_component_render_and_markup_reinjected_in_html_macro() {
    let cases = [
        PrepareMarkup::None,
        PrepareMarkup::Escaped("<b>x</b>".into()),
        PrepareMarkup::Raw("<b>x</b>".into()),
        PrepareMarkup::With(html! { b { "x" } }),
    ];

    for pm in cases {
        // Vía 1: renderizamos y obtenemos directamente el String.
        let via_component = TestPrepareComponent::render_pm(pm.clone());

        // Vía 2: renderizamos, reinyectamos el Markup en `html!` y volvemos a obtener String.
        let via_macro = {
            let mut cx = Context::default();
            let mut comp = TestPrepareComponent { pm };
            let markup = comp.render(&mut cx);
            html! { (markup) }.into_string()
        };

        assert_eq!(
            via_component, via_macro,
            "The output of component render and (Markup) inside html! must match"
        );
    }
}
