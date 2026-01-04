use pagetop::prelude::*;

fn assert_classes(c: &Classes, expected: Option<&str>) {
    let got = c.get();
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
    assert_classes(&Classes::new(""), None);
    assert_classes(&Classes::new("   "), None);
    assert_classes(&Classes::new("\t\n\r  "), None);
}

#[pagetop::test]
async fn classes_new_normalizes_and_dedups_and_preserves_first_occurrence_order() {
    let c = Classes::new("Btn btn BTN  btn-primary  BTN-PRIMARY");
    assert_classes(&c, Some("btn btn-primary"));
    assert!(c.contains("BTN"));
    assert!(c.contains("btn-primary"));
}

#[pagetop::test]
async fn classes_get_returns_none_when_empty_some_when_not() {
    assert_classes(&Classes::new(" "), None);
    assert_classes(&Classes::new("a"), Some("a"));
}

// **< Basic operations (add/prepend/set) >*********************************************************

#[pagetop::test]
async fn classes_add_appends_unique_and_normalizes() {
    let c = Classes::new("a b").with_classes(ClassesOp::Add, "C b  D");
    assert_classes(&c, Some("a b c d"));
}

#[pagetop::test]
async fn classes_add_ignores_empty_input() {
    let c = Classes::new("a b").with_classes(ClassesOp::Add, "   \t");
    assert_classes(&c, Some("a b"));
}

#[pagetop::test]
async fn classes_add_same_tokens() {
    let c = Classes::new("a b").with_classes(ClassesOp::Add, "A B a b");
    assert_classes(&c, Some("a b"));
}

#[pagetop::test]
async fn classes_add_rejects_non_ascii_is_noop() {
    let c = Classes::new("a b").with_classes(ClassesOp::Add, "c ñ d");
    assert_classes(&c, Some("a b"));
}

#[pagetop::test]
async fn classes_prepend_inserts_at_front_preserving_new_order() {
    let c = Classes::new("c d").with_classes(ClassesOp::Prepend, "A b");
    assert_classes(&c, Some("a b c d"));
}

#[pagetop::test]
async fn classes_prepend_inserts_new_tokens_skipping_duplicates() {
    let c = Classes::new("b c").with_classes(ClassesOp::Prepend, "a b d");
    assert_classes(&c, Some("a d b c"));
}

#[pagetop::test]
async fn classes_prepend_ignores_empty_input() {
    let c = Classes::new("a b").with_classes(ClassesOp::Prepend, "");
    assert_classes(&c, Some("a b"));
}

#[pagetop::test]
async fn classes_set_replaces_entire_list_and_dedups() {
    let c = Classes::new("a b c").with_classes(ClassesOp::Set, "X  y  y  Z");
    assert_classes(&c, Some("x y z"));
}

#[pagetop::test]
async fn classes_set_with_empty_input_clears() {
    let base = Classes::new("a b");
    let c = base.with_classes(ClassesOp::Set, " \n ");
    assert_classes(&c, None);
}

// **< Mutation operations (remove/toggle/replace) >************************************************

#[pagetop::test]
async fn classes_remove_is_case_insensitive() {
    let c = Classes::new("a b c d").with_classes(ClassesOp::Remove, "B  D");
    assert_classes(&c, Some("a c"));
}

#[pagetop::test]
async fn classes_remove_non_existing_is_noop() {
    let c = Classes::new("a b c").with_classes(ClassesOp::Remove, "x y z");
    assert_classes(&c, Some("a b c"));
}

#[pagetop::test]
async fn classes_remove_with_extra_whitespace() {
    let c = Classes::new("a b c d").with_classes(ClassesOp::Remove, "   b\t\t  \n d  ");
    assert_classes(&c, Some("a c"));
}

#[pagetop::test]
async fn classes_toggle_removes_if_present_case_insensitive() {
    let c = Classes::new("a b c").with_classes(ClassesOp::Toggle, "B");
    assert_classes(&c, Some("a c"));
}

#[pagetop::test]
async fn classes_toggle_adds_if_missing_and_normalizes() {
    let c = Classes::new("a b").with_classes(ClassesOp::Toggle, "C");
    assert_classes(&c, Some("a b c"));
}

#[pagetop::test]
async fn classes_toggle_multiple_tokens_is_sequential_and_order_dependent() {
    let c = Classes::new("a b").with_classes(ClassesOp::Toggle, "C  B   A");
    assert_classes(&c, Some("c"));
}

#[pagetop::test]
async fn classes_toggle_duplicate_tokens_are_applied_sequentially() {
    let c = Classes::new("b").with_classes(ClassesOp::Toggle, "a a");
    assert_classes(&c, Some("b"));

    let c = Classes::new("a b").with_classes(ClassesOp::Toggle, "a a");
    assert_classes(&c, Some("b a"));
}

#[pagetop::test]
async fn classes_replace_removes_targets_and_inserts_new_at_min_position() {
    let c = Classes::new("a b c d").with_classes(ClassesOp::Replace("c a".into()), "x y");
    assert_classes(&c, Some("x y b d"));
}

#[pagetop::test]
async fn classes_replace_when_none_found_does_nothing() {
    let c = Classes::new("a b").with_classes(ClassesOp::Replace("x y".into()), "c d");
    assert_classes(&c, Some("a b"));
}

#[pagetop::test]
async fn classes_replace_is_case_insensitive_on_targets_and_new_values_are_normalized() {
    let c = Classes::new("btn btn-primary active")
        .with_classes(ClassesOp::Replace("BTN-PRIMARY".into()), "Btn-Secondary");
    assert_classes(&c, Some("btn btn-secondary active"));
}

#[pagetop::test]
async fn classes_replace_with_empty_new_removes_only() {
    let c = Classes::new("a b c").with_classes(ClassesOp::Replace("b".into()), "   ");
    assert_classes(&c, Some("a c"));
}

#[pagetop::test]
async fn classes_replace_dedups_against_existing_items() {
    let c = Classes::new("a b c").with_classes(ClassesOp::Replace("b".into()), "c d");
    assert_classes(&c, Some("a d c"));
}

#[pagetop::test]
async fn classes_replace_ignores_target_whitespace_and_repetition() {
    let c = Classes::new("a b c").with_classes(ClassesOp::Replace("  b  b ".into()), "x y");
    assert_classes(&c, Some("a x y c"));
}

#[pagetop::test]
async fn classes_replace_rejects_non_ascii_targets_is_noop() {
    let c = Classes::new("a b c").with_classes(ClassesOp::Replace("b ñ".into()), "x");
    assert_classes(&c, Some("a b c"));
}

// **< Queries (contains) >*************************************************************************

#[pagetop::test]
async fn classes_contains_single() {
    let c = Classes::new("btn btn-primary");
    assert!(c.contains("btn"));
    assert!(c.contains("BTN"));
    assert!(!c.contains("missing"));
}

#[pagetop::test]
async fn classes_contains_all_and_any() {
    let c = Classes::new("btn btn-primary active");

    assert!(c.contains("btn active"));
    assert!(c.contains("BTN BTN-PRIMARY"));
    assert!(!c.contains("btn missing"));

    assert!(c.contains_any("missing active"));
    assert!(c.contains_any("BTN-PRIMARY missing"));
    assert!(!c.contains_any("missing other"));
}

#[pagetop::test]
async fn classes_contains_empty_and_whitespace_is_false() {
    let c = Classes::new("a b");
    assert!(!c.contains(""));
    assert!(!c.contains("   \t"));
    assert!(!c.contains_any(""));
    assert!(!c.contains_any(" \n "));
}

#[pagetop::test]
async fn classes_contains_non_ascii_is_false() {
    let c = Classes::new("a b");
    assert!(!c.contains("ñ"));
    assert!(!c.contains_any("a ñ"));
}

// **< Properties / regression (combined sequences, ordering) >*************************************

#[pagetop::test]
async fn classes_order_is_stable_for_existing_items() {
    let c = Classes::new("a b c")
        .with_classes(ClassesOp::Add, "d") // a b c d
        .with_classes(ClassesOp::Prepend, "x") // x a b c d
        .with_classes(ClassesOp::Remove, "b") // x a c d
        .with_classes(ClassesOp::Add, "b"); // x a c d b
    assert_classes(&c, Some("x a c d b"));
}
