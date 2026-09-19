use pagetop::prelude::*;

#[pagetop::test]
async fn is_not_rendered_when_empty() {
    let mut container = Container::new();
    let html = container.render(&mut Context::default()).await;

    assert!(html.is_empty());
}

#[pagetop::test]
async fn default_kind_renders_a_div_element() {
    let mut container = Container::new().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.starts_with("<div"));
    assert!(html.ends_with("</div>"));
}

#[pagetop::test]
async fn main_kind_renders_a_main_element() {
    let mut container = Container::main().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.starts_with("<main"));
    assert!(html.ends_with("</main>"));
}

// **< Container::with_width >**********************************************************************

async fn container_html(width: impl Into<Option<container::Width>>) -> String {
    let mut container = Container::new().with_width(width).with_child(Lc::n("x"));
    container
        .render(&mut Context::default())
        .await
        .into_string()
}

#[pagetop::test]
async fn width_is_none_by_default_and_adds_no_class() {
    assert_eq!(Container::new().width(), None);
    // A neutral box: without a width, the component adds no class at all.
    assert!(!container_html(None).await.contains("container"));
}

#[pagetop::test]
async fn width_can_be_set_and_reset() {
    let width = container::Width::From(Breakpoint::Lg);
    assert_eq!(Container::new().with_width(width).width(), Some(width));
    assert_eq!(
        Container::new().with_width(width).with_width(None).width(),
        None
    );
}

#[pagetop::test]
async fn width_responsive_adds_the_container_class() {
    assert!(
        container_html(container::Width::Responsive)
            .await
            .contains(r#"class="container""#)
    );
}

#[pagetop::test]
async fn width_from_uses_the_breakpoint_name() {
    assert!(
        container_html(container::Width::From(Breakpoint::Lg))
            .await
            .contains(r#"class="container-lg""#)
    );
}

#[pagetop::test]
async fn width_from_a_breakpoint_without_min_width_has_no_name() {
    // `Xs` has no minimum width, so the class carries no breakpoint name.
    assert!(
        container_html(container::Width::From(Breakpoint::Xs))
            .await
            .contains(r#"class="container""#)
    );
}

#[pagetop::test]
async fn width_fluid_adds_the_container_fluid_class() {
    assert!(
        container_html(container::Width::Fluid)
            .await
            .contains(r#"class="container-fluid""#)
    );
}

#[pagetop::test]
async fn width_fluid_max_adds_the_class_and_a_max_width_style() {
    let html = container_html(container::Width::FluidMax(UnitValue::RelRem(75.0))).await;

    assert!(html.contains("container-fluid"));
    assert!(html.contains("max-width: 75rem"));
}

#[pagetop::test]
async fn width_fluid_max_ignores_a_non_measurable_value() {
    let html = container_html(container::Width::FluidMax(UnitValue::Auto)).await;

    assert!(html.contains("container-fluid"));
    assert!(!html.contains("max-width"));
}
