use pagetop::prelude::*;

// **< ResponsiveStyles::add_style >****************************************************************

#[pagetop::test]
async fn add_style_basic_adds_declaration() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

#[pagetop::test]
async fn add_style_multiple_calls_accumulate_in_order() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    r.add_style(Breakpoint::Md, "col", "margin-inline-start", "0");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%; margin-inline-start: 0".to_string())
    );
}

#[pagetop::test]
async fn add_style_keeps_first_value_when_property_already_exists() {
    // First-write-wins: a later call for the same (breakpoint, classes, property) is a no-op,
    // it does not overwrite the value already stored.
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    r.add_style(Breakpoint::Md, "col", "margin-inline-start", "0");
    r.add_style(Breakpoint::Md, "col", "flex-basis", "33%");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%; margin-inline-start: 0".to_string())
    );
}

#[pagetop::test]
async fn add_style_property_name_is_case_insensitive() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "Flex-Basis", "50%");
    r.add_style(Breakpoint::Md, "col", "FLEX-BASIS", "33%");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

#[pagetop::test]
async fn add_style_value_preserves_case() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "font-family", "Arial");
    assert_eq!(
        r.get_style(Breakpoint::Md, "col", "font-family"),
        Some("Arial".to_string())
    );
}

#[pagetop::test]
async fn add_style_trims_whitespace_in_property_and_value() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "  flex-basis  ", "  50%  ");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

#[pagetop::test]
async fn add_style_repeated_call_never_looks_at_value_once_property_exists() {
    // The already-exists path returns before normalizing `value`, so even a blank value on a
    // repeated call has no effect on the declaration already stored.
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    r.add_style(Breakpoint::Md, "col", "flex-basis", "   ");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

#[pagetop::test]
async fn add_style_ignores_empty_property_or_value() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "", "50%");
    r.add_style(Breakpoint::Md, "col", "flex-basis", "");
    r.add_style(Breakpoint::Md, "col", "   ", "   ");
    assert!(r.is_empty());
}

#[pagetop::test]
async fn add_style_ignores_empty_or_blank_classes() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "", "flex-basis", "50%");
    r.add_style(Breakpoint::Md, "   ", "flex-basis", "50%");
    assert!(r.is_empty());
}

#[pagetop::test]
async fn add_style_ignores_non_ascii_classes() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "cañón", "flex-basis", "50%");
    assert!(r.is_empty());
}

// **< Class normalization >************************************************************************

#[pagetop::test]
async fn add_style_normalizes_classes_case_and_whitespace() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "  Foo   BAR  ", "color", "red");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "foo bar"),
        Some("color: red".to_string())
    );
}

#[pagetop::test]
async fn add_style_does_not_reorder_classes_tokens() {
    // Documented behavior: classes are stored as normalized but NOT sorted, so declaring the
    // same classes in a different token order creates a separate entry instead of merging.
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "foo bar", "color", "red");
    r.add_style(Breakpoint::Md, "bar foo", "font-weight", "bold");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "foo bar"),
        Some("color: red".to_string())
    );
    assert_eq!(
        r.get_styles(Breakpoint::Md, "bar foo"),
        Some("font-weight: bold".to_string())
    );
}

// **< Breakpoint isolation >***********************************************************************

#[pagetop::test]
async fn add_style_same_classes_different_breakpoints_do_not_mix() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Sm, "col", "flex-basis", "100%");
    r.add_style(Breakpoint::Lg, "col", "flex-basis", "50%");
    assert_eq!(
        r.get_styles(Breakpoint::Sm, "col"),
        Some("flex-basis: 100%".to_string())
    );
    assert_eq!(
        r.get_styles(Breakpoint::Lg, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

// **< No breakpoint (None) >***********************************************************************

#[pagetop::test]
async fn add_style_accepts_none_as_breakpoint() {
    let mut r = ResponsiveStyles::new();
    r.add_style(None, "col", "flex-basis", "50%");
    assert_eq!(
        r.get_styles(None, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

#[pagetop::test]
async fn add_style_none_and_xs_do_not_mix() {
    let mut r = ResponsiveStyles::new();
    r.add_style(None, "col", "flex-basis", "100%");
    r.add_style(Breakpoint::Xs, "col", "flex-basis", "50%");
    assert_eq!(
        r.get_styles(None, "col"),
        Some("flex-basis: 100%".to_string())
    );
    assert_eq!(
        r.get_styles(Breakpoint::Xs, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

// **< is_empty >***********************************************************************************

#[pagetop::test]
async fn responsive_styles_is_empty_on_default() {
    assert!(ResponsiveStyles::new().is_empty());
}

#[pagetop::test]
async fn responsive_styles_is_empty_false_after_add_style() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    assert!(!r.is_empty());
}

// **< get_style / get_styles >*********************************************************************

#[pagetop::test]
async fn get_style_returns_none_for_missing_breakpoint_or_classes() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    assert_eq!(r.get_style(Breakpoint::Lg, "col", "flex-basis"), None);
    assert_eq!(r.get_style(Breakpoint::Md, "other", "flex-basis"), None);
}

#[pagetop::test]
async fn get_style_returns_none_for_missing_property() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    assert_eq!(r.get_style(Breakpoint::Md, "col", "margin"), None);
}

#[pagetop::test]
async fn get_style_is_case_insensitive_and_trims_input() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    assert_eq!(
        r.get_style(Breakpoint::Md, "col", "FLEX-BASIS"),
        Some("50%".to_string())
    );
    assert_eq!(
        r.get_style(Breakpoint::Md, "col", "  flex-basis  "),
        Some("50%".to_string())
    );
}

#[pagetop::test]
async fn get_styles_returns_none_when_nothing_stored() {
    assert_eq!(
        ResponsiveStyles::new().get_styles(Breakpoint::Md, "col"),
        None
    );
}

#[pagetop::test]
async fn get_styles_matches_classes_after_normalization() {
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "foo bar", "color", "red");
    assert_eq!(
        r.get_styles(Breakpoint::Md, "  FOO   BAR  "),
        Some("color: red".to_string())
    );
}

// **< ResponsiveStyles::render >********************************************************************

#[pagetop::test]
async fn render_is_empty_when_nothing_stored() {
    let cx = Context::default();
    let r = ResponsiveStyles::new();
    assert_eq!(r.render(&cx).into_string(), "");
}

#[pagetop::test]
async fn render_zero_min_width_breakpoint_has_no_media_query() {
    // The default theme resolves `Breakpoint::Xs` to `UnitValue::Zero`.
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Xs, "col", "flex-basis", "100%");
    assert_eq!(r.render(&cx).into_string(), ".col{flex-basis:100%}");
}

#[pagetop::test]
async fn render_none_breakpoint_has_no_media_query() {
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(None, "col", "flex-basis", "100%");
    assert_eq!(r.render(&cx).into_string(), ".col{flex-basis:100%}");
}

#[pagetop::test]
async fn render_none_comes_before_every_breakpoint() {
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    r.add_style(None, "row", "display", "flex");
    assert_eq!(
        r.render(&cx).into_string(),
        ".row{display:flex}@media(min-width:768px){.col{flex-basis:50%}}"
    );
}

#[pagetop::test]
async fn render_non_zero_breakpoint_wraps_in_media_query() {
    // The default theme resolves `Breakpoint::Md` to `768px`.
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    assert_eq!(
        r.render(&cx).into_string(),
        "@media(min-width:768px){.col{flex-basis:50%}}"
    );
}

#[pagetop::test]
async fn render_groups_multiple_properties_in_the_same_rule() {
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    r.add_style(Breakpoint::Md, "col", "margin-inline-start", "0");
    assert_eq!(
        r.render(&cx).into_string(),
        "@media(min-width:768px){.col{flex-basis:50%;margin-inline-start:0}}"
    );
}

#[pagetop::test]
async fn render_concatenates_rules_of_different_selectors_in_the_same_breakpoint() {
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    r.add_style(Breakpoint::Md, "row", "display", "flex");
    assert_eq!(
        r.render(&cx).into_string(),
        "@media(min-width:768px){.col{flex-basis:50%}.row{display:flex}}"
    );
}

#[pagetop::test]
async fn render_converts_multiple_classes_into_a_compound_selector() {
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Md, "foo bar", "color", "red");
    assert_eq!(
        r.render(&cx).into_string(),
        "@media(min-width:768px){.foo.bar{color:red}}"
    );
}

#[pagetop::test]
async fn render_orders_breakpoints_mobile_first_regardless_of_insertion_order() {
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Lg, "col", "flex-basis", "33%");
    r.add_style(Breakpoint::Xs, "col", "flex-basis", "100%");
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    assert_eq!(
        r.render(&cx).into_string(),
        ".col{flex-basis:100%}\
         @media(min-width:768px){.col{flex-basis:50%}}\
         @media(min-width:992px){.col{flex-basis:33%}}"
    );
}

#[pagetop::test]
async fn render_has_no_line_breaks() {
    let cx = Context::default();
    let mut r = ResponsiveStyles::new();
    r.add_style(Breakpoint::Xs, "col", "flex-basis", "100%");
    r.add_style(Breakpoint::Md, "col", "flex-basis", "50%");
    r.add_style(Breakpoint::Md, "row", "display", "flex");
    assert!(!r.render(&cx).into_string().contains('\n'));
}

// **< Context / AssetsOp integration >*************************************************************

#[pagetop::test]
async fn context_add_responsive_style_feeds_responsives() {
    let cx = Context::default().with_assets(AssetsOp::AddResponsiveStyle(
        Some(Breakpoint::Md),
        "col".into(),
        "flex-basis".into(),
        "50%".into(),
    ));
    assert_eq!(
        cx.responsive_styles().get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%".to_string())
    );
}

#[pagetop::test]
async fn context_add_responsive_style_accumulates_across_calls() {
    let cx = Context::default()
        .with_assets(AssetsOp::AddResponsiveStyle(
            Some(Breakpoint::Md),
            "col".into(),
            "flex-basis".into(),
            "50%".into(),
        ))
        .with_assets(AssetsOp::AddResponsiveStyle(
            Some(Breakpoint::Md),
            "col".into(),
            "margin-inline-start".into(),
            "0".into(),
        ));
    assert_eq!(
        cx.responsive_styles().get_styles(Breakpoint::Md, "col"),
        Some("flex-basis: 50%; margin-inline-start: 0".to_string())
    );
}

#[pagetop::test]
async fn context_default_has_no_responsive_styles() {
    assert!(Context::default().responsive_styles().is_empty());
}

// **< Context::render_assets integration >*********************************************************

#[pagetop::test]
async fn render_assets_includes_style_tag_with_responsive_styles() {
    let mut cx = Context::default().with_assets(AssetsOp::AddResponsiveStyle(
        Some(Breakpoint::Xs),
        "col".into(),
        "flex-basis".into(),
        "100%".into(),
    ));
    assert_eq!(
        cx.render_assets().into_string(),
        "<style>.col{flex-basis:100%}</style>"
    );
}

#[pagetop::test]
async fn render_assets_omits_style_tag_when_no_responsive_styles() {
    let mut cx = Context::default();
    assert_eq!(cx.render_assets().into_string(), "");
}
