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
    // Reassigning the same key must replace the value, not add a duplicate entry.
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
    // An empty Props must not emit even an extra blank space.
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
    // The splice is always emitted after class and id, regardless of the order they are written in.
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
    // An empty splice emits no attribute nor extra space.
    assert_eq!(html! { span ("") { "x" } }.into_string(), "<span>x</span>");
}

// **< is_attrs_empty / is_empty >******************************************************************

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

// **< get_prop("id") >*****************************************************************************

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

// **< Regression & edge cases >********************************************************************

#[pagetop::test]
async fn props_hx_target_value_with_hash_renders_correctly() {
    // Regression: r#"..."# used to close prematurely when it found `"#list"`.
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
    // Documented behavior: names are not validated; the resulting HTML is not standard.
    let p = Props::new("", "val");
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span ="val"></span>"#
    );
}
