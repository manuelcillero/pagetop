use pagetop::prelude::*;

fn assert_styles(p: &Props, expected: Option<&str>) {
    let got = p.get_styles();
    assert_eq!(
        got.as_deref(),
        expected,
        "Expected {:?}, got {:?}",
        expected,
        got
    );
}

// **< PropsOp::add_style >*************************************************************************

#[pagetop::test]
async fn add_style_basic_adds_declaration() {
    let p = Props::default().with_prop(PropsOp::add_style("color", "red"));
    assert_styles(&p, Some("color: red"));
}

#[pagetop::test]
async fn add_style_multiple_calls_accumulate_in_order() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::add_style("font-weight", "bold"));
    assert_styles(&p, Some("color: red; font-weight: bold"));
}

#[pagetop::test]
async fn add_style_overrides_existing_property_preserving_position() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::add_style("font-weight", "bold"))
        .with_prop(PropsOp::add_style("color", "blue"));
    assert_styles(&p, Some("color: blue; font-weight: bold"));
}

#[pagetop::test]
async fn add_style_property_name_is_case_insensitive() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("Color", "red"))
        .with_prop(PropsOp::add_style("COLOR", "blue"));
    assert_styles(&p, Some("color: blue"));
}

#[pagetop::test]
async fn add_style_value_preserves_case_and_non_ascii() {
    let p = Props::default().with_prop(PropsOp::add_style("font-family", "'Alegreya Sans', ARIAL"));
    assert_styles(&p, Some("font-family: 'Alegreya Sans', ARIAL"));

    let p = Props::default().with_prop(PropsOp::add_style("content", "'Ñoño'"));
    assert_styles(&p, Some("content: 'Ñoño'"));
}

#[pagetop::test]
async fn add_style_value_may_contain_semicolons() {
    // A diferencia de PropsOp::Set("style", ...), que interpreta la cadena como declaraciones
    // separadas por ";", AddStyle recibe la propiedad y el valor ya separados, así que un ";"
    // dentro del valor (p. ej. una data URI) no supone ningún problema.
    let p = Props::default().with_prop(PropsOp::add_style(
        "background",
        "url(data:image/png;base64,AAAA)",
    ));
    assert_styles(&p, Some("background: url(data:image/png;base64,AAAA)"));
}

#[pagetop::test]
async fn add_style_trims_whitespace() {
    let p = Props::default().with_prop(PropsOp::add_style("  color  ", "  red  "));
    assert_styles(&p, Some("color: red"));
}

#[pagetop::test]
async fn add_style_ignores_empty_property_or_value() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("", "red"))
        .with_prop(PropsOp::add_style("color", ""))
        .with_prop(PropsOp::add_style("   ", "   "));
    assert_styles(&p, None);
}

// **< PropsOp::remove_style >**********************************************************************

#[pagetop::test]
async fn remove_style_removes_indicated_property() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::add_style("font-weight", "bold"))
        .with_prop(PropsOp::remove_style("font-weight"));
    assert_styles(&p, Some("color: red"));
}

#[pagetop::test]
async fn remove_style_is_case_insensitive() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::remove_style("COLOR"));
    assert_styles(&p, None);
}

#[pagetop::test]
async fn remove_style_trims_whitespace() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::remove_style("  color  "));
    assert_styles(&p, None);
}

#[pagetop::test]
async fn remove_style_non_existing_is_noop() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::remove_style("font-weight"));
    assert_styles(&p, Some("color: red"));
}

// **< PropsOp::set / remove ("style") >************************************************************

#[pagetop::test]
async fn styles_reset_replaces_entire_list() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::add_style("font-weight", "bold"))
        .with_prop(PropsOp::set("style", "margin: 0"));
    assert_styles(&p, Some("margin: 0"));
}

#[pagetop::test]
async fn styles_reset_parses_multiple_declarations() {
    let p = Props::default().with_prop(PropsOp::set("style", "color: red; font-weight: bold"));
    assert_styles(&p, Some("color: red; font-weight: bold"));
}

#[pagetop::test]
async fn styles_reset_ignores_declaration_without_colon() {
    let p = Props::default().with_prop(PropsOp::set(
        "style",
        "color: red; not-a-declaration; font-weight: bold",
    ));
    assert_styles(&p, Some("color: red; font-weight: bold"));
}

#[pagetop::test]
async fn styles_reset_ignores_empty_property_or_value() {
    let p = Props::default().with_prop(PropsOp::set("style", ": red; color: ; ok: yes"));
    assert_styles(&p, Some("ok: yes"));
}

#[pagetop::test]
async fn styles_reset_trims_whitespace_around_declarations() {
    let p = Props::default().with_prop(PropsOp::set(
        "style",
        "  color :  red  ;   font-weight:bold  ",
    ));
    assert_styles(&p, Some("color: red; font-weight: bold"));
}

#[pagetop::test]
async fn styles_reset_with_empty_input_clears() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::set("style", ""));
    assert_styles(&p, None);
}

#[pagetop::test]
async fn styles_reset_does_not_split_semicolon_inside_parens() {
    let p = Props::default().with_prop(PropsOp::set(
        "style",
        "background: url(data:image/png;base64,AAAA)",
    ));
    assert_styles(&p, Some("background: url(data:image/png;base64,AAAA)"));
}

#[pagetop::test]
async fn styles_reset_does_not_split_semicolon_inside_quotes() {
    let p = Props::default().with_prop(PropsOp::set("style", r#"content: "a;b""#));
    assert_styles(&p, Some(r#"content: "a;b""#));
}

#[pagetop::test]
async fn styles_reset_mixes_declarations_with_and_without_parens() {
    let p = Props::default().with_prop(PropsOp::set(
        "style",
        "color: red; background: url(a;b); margin: 0",
    ));
    assert_styles(&p, Some("color: red; background: url(a;b); margin: 0"));
}

// Límite conocido de PropsOp::Set("style", ...): no es un análisis CSS completo. Unos paréntesis
// sin cerrar arrastran el resto de la cadena a la misma declaración. Este test fija el
// comportamiento actual para que un cambio futuro sea deliberado, no accidental.
#[pagetop::test]
async fn styles_reset_unbalanced_parens_swallows_rest_of_string() {
    let p = Props::default().with_prop(PropsOp::set(
        "style",
        "background: url(data:image/png; margin: 0",
    ));
    assert_styles(&p, Some("background: url(data:image/png; margin: 0"));
}

#[pagetop::test]
async fn styles_remove_attr_clears_all_styles() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::remove("style"));
    assert_styles(&p, None);
}

// **< is_styles_empty >****************************************************************************

#[pagetop::test]
async fn props_is_styles_empty_on_default() {
    assert!(Props::default().is_styles_empty());
}

#[pagetop::test]
async fn props_is_styles_empty_false_after_add_style() {
    let p = Props::default().with_prop(PropsOp::add_style("color", "red"));
    assert!(!p.is_styles_empty());
}

#[pagetop::test]
async fn props_is_styles_empty_true_after_remove_style() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::remove("style"));
    assert!(p.is_styles_empty());
}

// **< get_styles / get_style / get_prop("style") >*************************************************

#[pagetop::test]
async fn styles_get_returns_none_when_empty_some_when_not() {
    assert_styles(&Props::default(), None);
    let p = Props::default().with_prop(PropsOp::add_style("color", "red"));
    assert_styles(&p, Some("color: red"));
}

#[pagetop::test]
async fn get_style_returns_value_for_existing_property() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::add_style("font-weight", "bold"));
    assert_eq!(p.get_style("color"), Some("red".to_string()));
    assert_eq!(p.get_style("font-weight"), Some("bold".to_string()));
}

#[pagetop::test]
async fn get_style_is_case_insensitive_and_trims_input() {
    let p = Props::default().with_prop(PropsOp::add_style("color", "red"));
    assert_eq!(p.get_style("COLOR"), Some("red".to_string()));
    assert_eq!(p.get_style("  Color  "), Some("red".to_string()));
}

#[pagetop::test]
async fn get_style_returns_none_for_missing_property() {
    let p = Props::default().with_prop(PropsOp::add_style("color", "red"));
    assert_eq!(p.get_style("margin"), None);
}

#[pagetop::test]
async fn get_prop_style_matches_get_styles() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::add_style("font-weight", "bold"));
    assert_eq!(p.get_prop("style"), p.get_styles());
}

// **< HTML rendering >*****************************************************************************

#[pagetop::test]
async fn props_styles_renders_style_attribute() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::add_style("font-weight", "bold"));
    assert_eq!(
        html! { button (p) { "OK" } }.into_string(),
        r#"<button style="color: red; font-weight: bold">OK</button>"#
    );
}

#[pagetop::test]
async fn props_styles_render_after_class_and_before_other_attrs() {
    let p = Props::default()
        .with_id("main")
        .with_prop(PropsOp::add_classes("btn"))
        .with_prop(PropsOp::add_style("color", "red"))
        .with_prop(PropsOp::set("data-x", "1"));
    assert_eq!(
        html! { button (p) { "OK" } }.into_string(),
        r#"<button id="main" class="btn" style="color: red" data-x="1">OK</button>"#
    );
}

#[pagetop::test]
async fn props_styles_escapes_double_quotes_in_value() {
    let p = Props::default().with_prop(PropsOp::add_style("content", r#""hi""#));
    assert_eq!(
        html! { span (p) {} }.into_string(),
        r#"<span style="content: &quot;hi&quot;"></span>"#
    );
}

// **< Combined sequences >*************************************************************************

#[pagetop::test]
async fn styles_sequence_preserves_position_through_updates_and_removals() {
    let p = Props::default()
        .with_prop(PropsOp::add_style("color", "red")) // color: red
        .with_prop(PropsOp::add_style("font-weight", "bold")) // color: red; font-weight: bold
        .with_prop(PropsOp::add_style("margin", "0")) // color: red; font-weight: bold; margin: 0
        .with_prop(PropsOp::remove_style("font-weight")) // color: red; margin: 0
        .with_prop(PropsOp::add_style("color", "blue")); // color: blue; margin: 0
    assert_styles(&p, Some("color: blue; margin: 0"));
}
