use pagetop::prelude::*;

// **< Construction & invariants >******************************************************************

#[pagetop::test]
async fn props_default_renders_nothing() {
    assert_eq!(
        html! { span (Props::default()) {} }.into_string(),
        "<span></span>"
    );
}

#[pagetop::test]
async fn props_new_creates_first_attr() {
    let p = Props::new("hx-get", "/api");
    assert_eq!(p.get_prop("hx-get"), Some("/api".to_string()));
}

#[pagetop::test]
async fn props_get_missing_key_returns_none() {
    let p = Props::new("hx-get", "/api");
    assert_eq!(p.get_prop("hx-post"), None);
    assert_eq!(p.get_prop(""), None);
}

// **< Props::classes >*****************************************************************************

#[pagetop::test]
async fn props_classes_renders_class_attribute() {
    let p = Props::classes("btn btn-primary");
    assert_eq!(
        html! { button (p) { "OK" } }.into_string(),
        r#"<button class="btn btn-primary">OK</button>"#
    );
}

#[pagetop::test]
async fn props_classes_empty_input_renders_no_class_attribute() {
    let p = Props::classes("   ");
    assert_eq!(
        html! { button (p) { "OK" } }.into_string(),
        "<button>OK</button>"
    );
}

#[pagetop::test]
async fn props_classes_can_be_extended_with_with_prop() {
    let p = Props::classes("btn").with_prop(PropsOp::add_classes("active"));
    assert_eq!(
        html! { button (p) { "OK" } }.into_string(),
        r#"<button class="btn active">OK</button>"#
    );
}

// **< PropsOp::set >*******************************************************************************

#[pagetop::test]
async fn props_set_adds_new_attrs() {
    let p = Props::default()
        .with_prop(PropsOp::set("hx-get", "/api"))
        .with_prop(PropsOp::set("hx-swap", "outerHTML"));
    assert_eq!(p.get_prop("hx-get"), Some("/api".to_string()));
    assert_eq!(p.get_prop("hx-swap"), Some("outerHTML".to_string()));
}

#[pagetop::test]
async fn props_set_replaces_existing_value() {
    let p = Props::new("hx-get", "/old").with_prop(PropsOp::set("hx-get", "/new"));
    assert_eq!(p.get_prop("hx-get"), Some("/new".to_string()));
}

#[pagetop::test]
async fn props_set_does_not_create_duplicate_key() {
    // Reasignar la misma clave debe reemplazar el valor, no añadir una entrada duplicada.
    let p = Props::new("key", "v1").with_prop(PropsOp::set("key", "v2"));
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span key="v2"></span>"#
    );
}

#[pagetop::test]
async fn props_set_preserves_insertion_order() {
    let p = Props::new("a", "1")
        .with_prop(PropsOp::set("b", "2"))
        .with_prop(PropsOp::set("c", "3"));
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span a="1" b="2" c="3"></span>"#
    );
}

// **< PropsOp::remove >****************************************************************************

#[pagetop::test]
async fn props_remove_existing_attr() {
    let p = Props::new("a", "1")
        .with_prop(PropsOp::set("b", "2"))
        .with_prop(PropsOp::remove("a"));
    assert_eq!(p.get_prop("a"), None);
    assert_eq!(p.get_prop("b"), Some("2".to_string()));
}

#[pagetop::test]
async fn props_remove_nonexistent_key_is_noop() {
    let p = Props::new("a", "1").with_prop(PropsOp::remove("missing"));
    assert_eq!(p.get_prop("a"), Some("1".to_string()));
    assert_eq!(p.get_prop("missing"), None);
}

#[pagetop::test]
async fn props_renders_nothing_after_removing_last_attr() {
    let p = Props::new("only", "one").with_prop(PropsOp::remove("only"));
    assert_eq!(html! { span (p) {} }.into_string(), "<span></span>");
}

// **< HTML Escaped >*******************************************************************************

#[pagetop::test]
async fn props_escapes_ampersand_and_angle_brackets_in_value() {
    let p = Props::new("data-info", "a&b<c>d");
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span data-info="a&amp;b&lt;c&gt;d"></span>"#
    );
}

#[pagetop::test]
async fn props_escapes_double_quotes_in_value() {
    let p = Props::new("data-label", r#"say "hello""#);
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span data-label="say &quot;hello&quot;"></span>"#
    );
}

// **< Integration with html! >*********************************************************************

#[pagetop::test]
async fn props_empty_in_html_macro_produces_no_attributes() {
    // Una Props vacía no debe emitir ni siquiera un espacio en blanco extra.
    let p = Props::default();
    assert_eq!(
        html! { button (p) { "x" } }.into_string(),
        "<button>x</button>"
    );
}

#[pagetop::test]
async fn props_single_attr_in_html_macro() {
    let p = Props::new("hx-get", "/api");
    assert_eq!(
        html! { button (p) { "Load" } }.into_string(),
        r#"<button hx-get="/api">Load</button>"#
    );
}

#[pagetop::test]
async fn props_multiple_attrs_preserve_order_in_html_macro() {
    let p = Props::new("hx-get", "/api")
        .with_prop(PropsOp::set("hx-target", "#result"))
        .with_prop(PropsOp::set("hx-swap", "outerHTML"));
    assert_eq!(
        html! { button (p) {} }.into_string(),
        r##"<button hx-get="/api" hx-target="#result" hx-swap="outerHTML"></button>"##
    );
}

#[pagetop::test]
async fn props_alongside_class_and_id_in_html_macro() {
    // El splice siempre se emite después de class e id, independientemente del orden escrito.
    let p = Props::new("hx-get", "/api");
    assert_eq!(
        html! { button #mybtn .btn (p) { "Go" } }.into_string(),
        r#"<button class="btn" id="mybtn" hx-get="/api">Go</button>"#
    );
}

#[pagetop::test]
async fn props_alongside_named_attr_renders_after_it() {
    let p = Props::new("hx-get", "/api");
    assert_eq!(
        html! { button type="button" (p) {} }.into_string(),
        r#"<button type="button" hx-get="/api"></button>"#
    );
}

#[pagetop::test]
async fn props_multiple_splices_in_same_element() {
    let p1 = Props::new("hx-get", "/api");
    let p2 = Props::new("hx-swap", "outerHTML");
    assert_eq!(
        html! { button (p1) (p2) {} }.into_string(),
        r#"<button hx-get="/api" hx-swap="outerHTML"></button>"#
    );
}

#[pagetop::test]
async fn props_inline_construction_in_html_macro() {
    assert_eq!(
        html! { button (Props::new("hx-get", "/api")) { "Go" } }.into_string(),
        r#"<button hx-get="/api">Go</button>"#
    );
}

#[pagetop::test]
async fn props_conditional_expression_in_html_macro() {
    for (active, expected) in [
        (true, r#"<button hx-get="/api">x</button>"#),
        (false, "<button>x</button>"),
    ] {
        let markup = html! {
            button (if active { Props::new("hx-get", "/api") } else { Props::default() }) { "x" }
        };
        assert_eq!(markup.into_string(), expected);
    }
}

#[pagetop::test]
async fn props_splice_empty_string_emits_nothing() {
    // Un splice vacío no emite ningún atributo ni espacio extra.
    assert_eq!(html! { span ("") { "x" } }.into_string(), "<span>x</span>");
}

// **< is_attrs_empty / is_classes_empty / is_empty >***********************************************

#[pagetop::test]
async fn props_is_attrs_empty_on_default() {
    assert!(Props::default().is_attrs_empty());
}

#[pagetop::test]
async fn props_is_attrs_empty_false_after_set() {
    assert!(!Props::new("hx-get", "/api").is_attrs_empty());
}

#[pagetop::test]
async fn props_is_attrs_empty_true_after_removing_last_attr() {
    let p = Props::new("only", "one").with_prop(PropsOp::remove("only"));
    assert!(p.is_attrs_empty());
}

#[pagetop::test]
async fn props_is_empty_on_default() {
    assert!(Props::default().is_empty());
}

#[pagetop::test]
async fn props_is_empty_false_with_id() {
    assert!(!Props::default().with_id("main").is_empty());
}

#[pagetop::test]
async fn props_is_empty_false_with_attr() {
    assert!(!Props::new("hx-get", "/api").is_empty());
}

#[pagetop::test]
async fn props_is_empty_false_with_class() {
    assert!(!Props::classes("btn").is_empty());
}

#[pagetop::test]
async fn props_is_classes_empty_on_default() {
    assert!(Props::default().is_classes_empty());
}

#[pagetop::test]
async fn props_is_classes_empty_false_after_add_classes() {
    assert!(!Props::classes("btn").is_classes_empty());
}

#[pagetop::test]
async fn props_is_classes_empty_true_after_remove_class() {
    let p = Props::classes("btn").with_prop(PropsOp::remove("class"));
    assert!(p.is_classes_empty());
}

// **< get_prop("id") / get_prop("class") >*********************************************************

#[pagetop::test]
async fn get_prop_id_returns_none_by_default() {
    assert_eq!(Props::default().get_prop("id"), None);
}

#[pagetop::test]
async fn get_prop_id_returns_normalized_value() {
    let p = Props::default().with_id("My Button");
    assert_eq!(p.get_prop("id"), Some("my_button".to_string()));
}

#[pagetop::test]
async fn get_prop_id_matches_get_id() {
    let p = Props::default().with_id("Header");
    assert_eq!(p.get_prop("id"), p.get_id());
}

#[pagetop::test]
async fn get_prop_class_returns_none_by_default() {
    assert_eq!(Props::default().get_prop("class"), None);
}

#[pagetop::test]
async fn get_prop_class_returns_joined_classes() {
    let p = Props::classes("btn btn-primary").with_prop(PropsOp::add_classes("active"));
    assert_eq!(
        p.get_prop("class"),
        Some("btn btn-primary active".to_string())
    );
}

#[pagetop::test]
async fn get_prop_class_matches_get_classes() {
    let p = Props::classes("btn active");
    assert_eq!(p.get_prop("class"), p.get_classes());
}

// **< Regression & edge cases >********************************************************************

#[pagetop::test]
async fn props_hx_target_value_with_hash_renders_correctly() {
    // Regresión: r#"..."# se cerraba prematuramente al encontrar `"#lista"`.
    let p = Props::new("hx-target", "#list");
    assert_eq!(
        html! { button (p) {} }.into_string(),
        r##"<button hx-target="#list"></button>"##
    );
}

#[pagetop::test]
async fn props_with_empty_value_renders_attr_with_empty_value() {
    let p = Props::new("data-expanded", "");
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span data-expanded=""></span>"#
    );
}

#[pagetop::test]
async fn props_chained_set_and_remove_yields_expected_state() {
    let p = Props::new("a", "1")
        .with_prop(PropsOp::set("b", "2"))
        .with_prop(PropsOp::set("c", "3"))
        .with_prop(PropsOp::remove("b"))
        .with_prop(PropsOp::set("a", "updated"));
    assert_eq!(p.get_prop("a"), Some("updated".to_string()));
    assert_eq!(p.get_prop("b"), None);
    assert_eq!(p.get_prop("c"), Some("3".to_string()));
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span a="updated" c="3"></span>"#
    );
}

#[pagetop::test]
async fn props_with_empty_attr_name_renders_without_validation() {
    // Comportamiento documentado: los nombres no se validan; el HTML resultante no es estándar.
    let p = Props::new("", "val");
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span ="val"></span>"#
    );
}
