use pagetop::prelude::*;

fn assert_classes(p: &Props, expected: Option<&str>) {
    let got = p.get_classes();
    assert_eq!(
        got.as_deref(),
        expected,
        "Expected {:?}, got {:?}",
        expected,
        got
    );
}

// **< Construction & invariants (new/get) >********************************************************

#[pagetop::test]
async fn classes_new_empty_and_whitespace_is_empty() {
    assert_classes(&Props::classes(""), None);
    assert_classes(&Props::classes("   "), None);
    assert_classes(&Props::classes("\t\n\r  "), None);
}

#[pagetop::test]
async fn classes_new_normalizes_and_dedups_and_preserves_first_occurrence_order() {
    let p = Props::classes("Btn btn BTN  btn-primary  BTN-PRIMARY");
    assert_classes(&p, Some("btn btn-primary"));
    assert!(p.has_class("BTN"));
    assert!(p.has_class("btn-primary"));
}

#[pagetop::test]
async fn classes_get_returns_none_when_empty_some_when_not() {
    assert_classes(&Props::classes(" "), None);
    assert_classes(&Props::classes("a"), Some("a"));
}

// **< Basic operations (add/prepend/set) >*********************************************************

#[pagetop::test]
async fn classes_add_appends_unique_and_normalizes() {
    let p = Props::classes("a b").with_prop(PropsOp::add_classes("C b  D"));
    assert_classes(&p, Some("a b c d"));
}

#[pagetop::test]
async fn classes_add_ignores_empty_input() {
    let p = Props::classes("a b").with_prop(PropsOp::add_classes("   \t"));
    assert_classes(&p, Some("a b"));
}

#[pagetop::test]
async fn classes_add_same_tokens() {
    let p = Props::classes("a b").with_prop(PropsOp::add_classes("A B a b"));
    assert_classes(&p, Some("a b"));
}

#[pagetop::test]
async fn classes_add_rejects_non_ascii_is_noop() {
    let p = Props::classes("a b").with_prop(PropsOp::add_classes("c ñ d"));
    assert_classes(&p, Some("a b"));
}

#[pagetop::test]
async fn classes_prepend_inserts_at_front_preserving_new_order() {
    let p = Props::classes("c d").with_prop(PropsOp::prepend_classes("A b"));
    assert_classes(&p, Some("a b c d"));
}

#[pagetop::test]
async fn classes_prepend_inserts_new_tokens_skipping_duplicates() {
    let p = Props::classes("b c").with_prop(PropsOp::prepend_classes("a b d"));
    assert_classes(&p, Some("a d b c"));
}

#[pagetop::test]
async fn classes_prepend_ignores_empty_input() {
    let p = Props::classes("a b").with_prop(PropsOp::prepend_classes(""));
    assert_classes(&p, Some("a b"));
}

#[pagetop::test]
async fn classes_reset_replaces_entire_list_and_dedups() {
    let p = Props::classes("a b c").with_prop(PropsOp::set("class", "X  y  y  Z"));
    assert_classes(&p, Some("x y z"));
}

#[pagetop::test]
async fn classes_reset_with_empty_input_clears() {
    let p = Props::classes("a b").with_prop(PropsOp::set("class", " \n "));
    assert_classes(&p, None);
}

#[pagetop::test]
async fn classes_reset_with_non_ascii_is_noop() {
    let p = Props::classes("a b").with_prop(PropsOp::set("class", "ñ"));
    assert_classes(&p, Some("a b"));
}

// **< Mutation operations (remove) >***************************************************************

#[pagetop::test]
async fn classes_remove_is_case_insensitive() {
    let p = Props::classes("a b c d").with_prop(PropsOp::remove_classes("B  D"));
    assert_classes(&p, Some("a c"));
}

#[pagetop::test]
async fn classes_remove_non_existing_is_noop() {
    let p = Props::classes("a b c").with_prop(PropsOp::remove_classes("x y z"));
    assert_classes(&p, Some("a b c"));
}

#[pagetop::test]
async fn classes_remove_with_extra_whitespace() {
    let p = Props::classes("a b c d").with_prop(PropsOp::remove_classes("   b\t\t  \n d  "));
    assert_classes(&p, Some("a c"));
}

// **< Queries (contains) >*************************************************************************

#[pagetop::test]
async fn classes_contains_single() {
    let p = Props::classes("btn btn-primary");
    assert!(p.has_class("btn"));
    assert!(p.has_class("BTN"));
    assert!(!p.has_class("missing"));
}

#[pagetop::test]
async fn classes_contains_all_and_any() {
    let p = Props::classes("btn btn-primary active");

    assert!(p.has_class("btn active"));
    assert!(p.has_class("BTN BTN-PRIMARY"));
    assert!(!p.has_class("btn missing"));

    assert!(p.has_any_class("missing active"));
    assert!(p.has_any_class("BTN-PRIMARY missing"));
    assert!(!p.has_any_class("missing other"));
}

#[pagetop::test]
async fn classes_contains_empty_and_whitespace_is_false() {
    let p = Props::classes("a b");
    assert!(!p.has_class(""));
    assert!(!p.has_class("   \t"));
    assert!(!p.has_any_class(""));
    assert!(!p.has_any_class(" \n "));
}

#[pagetop::test]
async fn classes_contains_non_ascii_is_false() {
    let p = Props::classes("a b");
    assert!(!p.has_class("ñ"));
    assert!(!p.has_any_class("a ñ"));
}

// **< Properties / regression (combined sequences, ordering) >*************************************

#[pagetop::test]
async fn classes_order_is_stable_for_existing_items() {
    let p = Props::classes("a b c")
        .with_prop(PropsOp::add_classes("d")) // a b c d
        .with_prop(PropsOp::prepend_classes("x")) // x a b c d
        .with_prop(PropsOp::remove_classes("b")) // x a c d
        .with_prop(PropsOp::add_classes("b")); // x a c d b
    assert_classes(&p, Some("x a c d b"));
}
