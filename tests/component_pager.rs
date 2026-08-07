use pagetop::prelude::*;

#[pagetop::test]
async fn single_page_renders_nothing() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_items_per_page(20)
        .with_total_items(5);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert_eq!(html, "");
}

#[pagetop::test]
async fn default_aria_label_is_page_navigation() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(10)
        .with_total_items(50);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"aria-label="Page navigation""#));
}

#[pagetop::test]
async fn with_aria_label_overrides_the_default() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(10)
        .with_total_items(50)
        .with_aria_label(L10n::n("Users pagination"));

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"aria-label="Users pagination""#));
    assert!(!html.contains(r#"aria-label="Page navigation""#));
}

#[pagetop::test]
async fn summary_is_hidden_by_default() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(20)
        .with_total_items(97);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("pager-summary"));
}

#[pagetop::test]
async fn summary_shows_the_range_of_the_current_page() {
    let mut first_page = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(20)
        .with_total_items(97)
        .with_summary(true);
    let html = first_page
        .render(&mut Context::default())
        .await
        .into_string();
    assert!(html.contains(r#"<span class="pager-summary">Showing 1-20 of 97</span>"#));

    // Last page: fewer items than `items_per_page`, so `last` stops at `total_items`.
    let mut last_page = Pager::new()
        .with_base_path("/list")
        .with_current_page(5)
        .with_items_per_page(20)
        .with_total_items(97)
        .with_summary(true);
    let html = last_page
        .render(&mut Context::default())
        .await
        .into_string();
    assert!(html.contains(r#"<span class="pager-summary">Showing 81-97 of 97</span>"#));
}

#[pagetop::test]
async fn renders_page_links_and_current_page() {
    let mut pager = Pager::new()
        .with_base_path("/admin/users")
        .with_current_page(2)
        .with_items_per_page(20)
        .with_total_items(45);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"href="/admin/users?page=1""#));
    assert!(html.contains(r#"href="/admin/users?page=2" aria-current="page""#));
    assert!(html.contains(r#"href="/admin/users?page=3""#));
    assert!(html.contains(r#"<li class="page-item active">"#));
    // Not truncated (3 pages), so there are no previous/next buttons: just 3 page numbers.
    assert_eq!(html.matches(r#"class="page-link""#).count(), 3);
}

#[pagetop::test]
async fn prev_next_are_hidden_by_default_when_not_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(10)
        .with_total_items(50);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("page-previous"));
    assert!(!html.contains("page-next"));
}

#[pagetop::test]
async fn prev_next_are_shown_by_default_when_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(10)
        .with_items_per_page(1)
        .with_total_items(20);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="page-item page-previous"><a"#));
    assert!(html.contains(r#"class="page-item page-next"><a"#));
}

#[pagetop::test]
async fn previous_is_disabled_at_the_first_page_when_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(1)
        .with_total_items(20);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<li class="page-item page-previous disabled">"#));
    assert!(html.contains(r#"aria-disabled="true" aria-label="Previous page">"#));
    assert!(html.contains(r#"class="page-item page-next"><a"#));
}

#[pagetop::test]
async fn next_is_disabled_at_the_last_page_when_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(20)
        .with_items_per_page(1)
        .with_total_items(20);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<li class="page-item page-next disabled">"#));
    assert!(html.contains(r#"aria-disabled="true" aria-label="Next page">"#));
    assert!(html.contains(r#"class="page-item page-previous"><a"#));
}

#[pagetop::test]
async fn prev_next_can_be_forced_to_always_show_even_when_not_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(3)
        .with_items_per_page(10)
        .with_total_items(50)
        .with_prev_next(PagerVisibility::Always);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="page-item page-previous"><a"#));
    assert!(html.contains(r#"class="page-item page-next"><a"#));
}

#[pagetop::test]
async fn prev_next_never_hides_them_even_when_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(10)
        .with_items_per_page(1)
        .with_total_items(20)
        .with_prev_next(PagerVisibility::Never);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("page-previous"));
    assert!(!html.contains("page-next"));
}

#[pagetop::test]
async fn first_and_last_icon_buttons_never_render() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(10)
        .with_items_per_page(1)
        .with_total_items(20)
        .with_prev_next(PagerVisibility::Always)
        .with_window(3);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("page-first"));
    assert!(!html.contains("page-last"));
}

#[pagetop::test]
async fn jump_form_is_hidden_by_default_when_not_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(2)
        .with_items_per_page(1)
        .with_total_items(5);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("pager-jump"));
}

#[pagetop::test]
async fn jump_form_is_shown_by_default_when_truncated() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(10)
        .with_items_per_page(1)
        .with_total_items(20);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(
        r#"<form action="/list" accept-charset="UTF-8" id="pager-1-jump" class="form pager-jump">"#
    ));
    assert!(html.contains(r#"name="page""#));
    assert!(html.contains(r#"min="1""#));
    assert!(html.contains(r#"max="20""#));
}

#[pagetop::test]
async fn jump_form_can_be_forced_always_or_never() {
    let mut always_shown = Pager::new()
        .with_base_path("/list")
        .with_current_page(2)
        .with_items_per_page(1)
        .with_total_items(5)
        .with_jump(PagerVisibility::Always);
    let html = always_shown
        .render(&mut Context::default())
        .await
        .into_string();
    assert!(html.contains("pager-jump"));

    let mut never_shown = Pager::new()
        .with_base_path("/list")
        .with_current_page(10)
        .with_items_per_page(1)
        .with_total_items(20)
        .with_jump(PagerVisibility::Never);
    let html = never_shown
        .render(&mut Context::default())
        .await
        .into_string();
    assert!(!html.contains("pager-jump"));
}

#[pagetop::test]
async fn extra_query_travels_as_hidden_fields_in_the_jump_form() {
    let mut pager = Pager::new()
        .with_base_path("/admin/users")
        .with_extra_query("q", "ana")
        .with_extra_query("sort", "username")
        .with_current_page(1)
        .with_items_per_page(20)
        .with_total_items(97)
        .with_jump(PagerVisibility::Always);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<input type="hidden" name="q" value="ana">"#));
    assert!(html.contains(r#"<input type="hidden" name="sort" value="username">"#));
}

#[pagetop::test]
async fn without_a_window_every_page_number_is_shown() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(10)
        .with_items_per_page(1)
        .with_total_items(20)
        .with_window(0);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("page-ellipsis"));
    assert!(html.contains(r#"href="/list?page=1""#));
    assert!(html.contains(r#"href="/list?page=20""#));
}

#[pagetop::test]
async fn window_truncates_far_pages_behind_an_ellipsis() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(10)
        .with_items_per_page(1)
        .with_total_items(20)
        .with_window(3);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert_eq!(html.matches("page-ellipsis").count(), 2);
    for page in [7, 8, 9, 10, 11, 12, 13] {
        assert!(
            html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be visible"
        );
    }
    for page in [2, 3, 4, 5, 6, 14, 15, 16, 17, 18, 19] {
        assert!(
            !html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be hidden behind an ellipsis"
        );
    }
    assert!(html.contains(r#"href="/list?page=1""#));
    assert!(html.contains(r#"href="/list?page=20""#));
}

#[pagetop::test]
async fn a_single_hidden_page_is_shown_instead_of_an_ellipsis() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(6)
        .with_items_per_page(1)
        .with_total_items(20)
        .with_window(3);

    let html = pager.render(&mut Context::default()).await.into_string();

    // The window (3..=9) leaves a single gap on the low side (page 2):
    // it is shown instead of being truncated with an ellipsis.
    assert_eq!(html.matches("page-ellipsis").count(), 1);
    assert!(html.contains(r#"href="/list?page=2""#));
}

#[pagetop::test]
async fn current_page_near_an_edge_does_not_panic_and_keeps_first_and_last() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(1)
        .with_total_items(20)
        .with_window(3);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"href="/list?page=1" aria-current="page""#));
    assert!(html.contains(r#"href="/list?page=20""#));
}

#[pagetop::test]
async fn window_max_cell_count_follows_2_window_plus_5() {
    // Max cells = first + last + one filler slot each side (number or ellipsis) + the numbers in
    // the window itself (2 * window + 1) = 2 * window + 5. Must hold both for a centered page
    // (baseline case) and near an edge (padded case), for any window size.
    for window in [1_u64, 2, 3, 4, 5] {
        let max_cells = 2 * window + 5;

        let mut centered = Pager::new()
            .with_base_path("/list")
            .with_current_page(500)
            .with_items_per_page(1)
            .with_total_items(1000)
            .with_window(window)
            .with_prev_next(PagerVisibility::Never);
        let html = centered.render(&mut Context::default()).await.into_string();
        assert_eq!(
            html.matches("page-item").count() as u64,
            max_cells,
            "window={window}, centered page"
        );

        let mut near_edge = Pager::new()
            .with_base_path("/list")
            .with_current_page(1)
            .with_items_per_page(1)
            .with_total_items(1000)
            .with_window(window)
            .with_prev_next(PagerVisibility::Never);
        let html = near_edge
            .render(&mut Context::default())
            .await
            .into_string();
        assert_eq!(
            html.matches("page-item").count() as u64,
            max_cells,
            "window={window}, page 1"
        );
    }
}

#[pagetop::test]
async fn window_pads_out_as_documented_in_the_module_example() {
    // Matches the exact example in the doc comment of `Pager`.
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(1)
        .with_total_items(200)
        .with_window(3);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert_eq!(html.matches("page-ellipsis").count(), 1);
    for page in [1, 2, 3, 4, 5, 6, 7, 8, 9, 200] {
        assert!(
            html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be visible"
        );
    }
    for page in [10, 11, 199] {
        assert!(
            !html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be hidden behind an ellipsis"
        );
    }
}

#[pagetop::test]
async fn window_pads_out_to_a_constant_cell_count_near_the_first_page() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(1)
        .with_items_per_page(1)
        .with_total_items(188)
        .with_window(2);

    let html = pager.render(&mut Context::default()).await.into_string();

    // Near page 1 the low side of the window needs neither ellipsis nor filler number, so that gap
    // is reinvested on the high side instead of just shrinking the window.
    assert_eq!(html.matches("page-ellipsis").count(), 1);
    for page in [1, 2, 3, 4, 5, 6, 7, 188] {
        assert!(
            html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be visible"
        );
    }
    for page in [8, 9, 100, 187] {
        assert!(
            !html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be hidden behind an ellipsis"
        );
    }
}

#[pagetop::test]
async fn window_pads_out_to_a_constant_cell_count_near_the_last_page() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(188)
        .with_items_per_page(1)
        .with_total_items(188)
        .with_window(2);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert_eq!(html.matches("page-ellipsis").count(), 1);
    for page in [1, 182, 183, 184, 185, 186, 187, 188] {
        assert!(
            html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be visible"
        );
    }
    for page in [2, 89, 181] {
        assert!(
            !html.contains(&format!(r#"href="/list?page={page}""#)),
            "expected page {page} to be hidden behind an ellipsis"
        );
    }
}

#[pagetop::test]
async fn small_total_is_never_truncated_even_with_a_window() {
    let mut pager = Pager::new()
        .with_base_path("/list")
        .with_current_page(5)
        .with_items_per_page(1)
        .with_total_items(9) // total_pages == 2 * window + 3
        .with_window(3);

    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("page-ellipsis"));
    for page in 1..=9 {
        assert!(html.contains(&format!(r#"href="/list?page={page}""#)));
    }
}
