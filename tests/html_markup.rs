use pagetop::prelude::*;

/// Componente mínimo para probar `Markup` pasando por el ciclo real de renderizado de componentes
/// (`ComponentRender`). El parámetro de contexto `"renderable"` se usará para controlar si el
/// componente se renderiza (`true` por defecto).
#[derive(AutoDefault, Clone)]
struct TestMarkupComponent {
    markup: Markup,
}

impl Component for TestMarkupComponent {
    fn new() -> Self {
        Self::default()
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        cx.param_or::<bool>("renderable", true)
    }

    fn prepare(&self, _cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(self.markup.clone())
    }
}

// **< Comportamiento de Markup >*******************************************************************

#[pagetop::test]
async fn string_in_html_macro_escapes_html_entities() {
    let markup = html! { ("<b>& \" ' </b>") };
    assert_eq!(markup.into_string(), "&lt;b&gt;&amp; &quot; ' &lt;/b&gt;");
}

#[pagetop::test]
async fn preescaped_in_html_macro_is_inserted_verbatim() {
    let markup = html! { (PreEscaped("<b>bold</b><script>1<2</script>")) };
    assert_eq!(markup.into_string(), "<b>bold</b><script>1<2</script>");
}

#[pagetop::test]
async fn unicode_is_preserved_in_markup() {
    // Texto con acentos y emojis: sólo se escapan los signos HTML.
    let esc = html! { ("Hello, tomorrow coffee ☕ & donuts!") };
    assert_eq!(esc.into_string(), "Hello, tomorrow coffee ☕ &amp; donuts!");

    // PreEscaped debe pasar íntegro.
    let raw = html! { (PreEscaped("Title — section © 2025")) };
    assert_eq!(raw.into_string(), "Title — section © 2025");
}

#[pagetop::test]
async fn markup_is_empty_semantics() {
    assert!(html! {}.is_empty());

    assert!(html! { ("") }.is_empty());
    assert!(!html! { ("x") }.is_empty());

    assert!(html! { (PreEscaped(String::new())) }.is_empty());
    assert!(!html! { (PreEscaped("a")) }.is_empty());

    assert!(html! { (String::new()) }.is_empty());

    assert!(!html! { span { "!" } }.is_empty());

    // Espacios NO se consideran vacíos.
    assert!(!html! { (" ") }.is_empty());
    assert!(!html! { (PreEscaped(" ")) }.is_empty());
}

// **< Markup a través del ciclo de componente >****************************************************

#[pagetop::test]
async fn non_renderable_component_produces_empty_markup() {
    let mut cx = Context::default().with_param("renderable", false);
    let mut comp = TestMarkupComponent {
        markup: html! { p { "Should never be rendered" } },
    };
    assert_eq!(comp.render(&mut cx).into_string(), "");
}

#[pagetop::test]
async fn markup_from_component_equals_markup_reinjected_in_html_macro() {
    let cases = [
        html! {},
        html! { ("<b>x</b>") },
        html! { (PreEscaped("<b>x</b>")) },
        html! { b { "x" } },
    ];

    for markup in cases {
        // Vía 1: renderizamos a través del ciclo de componente.
        let via_component = {
            let mut cx = Context::default();
            let mut comp = TestMarkupComponent {
                markup: markup.clone(),
            };
            comp.render(&mut cx).into_string()
        };

        // Vía 2: reinyectamos el Markup en `html!` directamente.
        let via_macro = html! { (markup) }.into_string();

        assert_eq!(
            via_component, via_macro,
            "The output of component render and (Markup) inside html! must match"
        );
    }
}
