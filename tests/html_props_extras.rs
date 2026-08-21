use pagetop::prelude::*;

// **< PropsOp::set_extra / remove_extra >**********************************************************

#[pagetop::test]
async fn set_and_read_extra() {
    let props = Props::default().with_prop(PropsOp::set_extra("ext.flag", true));
    assert!(*props.extra::<bool>("ext.flag").unwrap());
}

#[pagetop::test]
async fn overwrite_extra() {
    let props = Props::default()
        .with_prop(PropsOp::set_extra("ext.count", 1_u32))
        .with_prop(PropsOp::set_extra("ext.count", 2_u32));
    assert_eq!(*props.extra::<u32>("ext.count").unwrap(), 2);
}

#[pagetop::test]
async fn remove_extra() {
    let props = Props::default()
        .with_prop(PropsOp::set_extra("ext.flag", true))
        .with_prop(PropsOp::remove_extra("ext.flag"));
    assert_eq!(
        props.extra::<bool>("ext.flag"),
        Err(PropsError::ExtraNotFound { key: "ext.flag" })
    );
}

// **< Props::extra >*******************************************************************************

#[pagetop::test]
async fn extra_not_found() {
    let props = Props::default();
    assert_eq!(
        props.extra::<bool>("ext.missing"),
        Err(PropsError::ExtraNotFound { key: "ext.missing" })
    );
}

#[pagetop::test]
async fn extra_type_mismatch() {
    let props = Props::default().with_prop(PropsOp::set_extra("ext.flag", true));
    assert!(matches!(
        props.extra::<u32>("ext.flag"),
        Err(PropsError::ExtraTypeMismatch { .. })
    ));
}

// **< Props::extra_or >****************************************************************************

#[pagetop::test]
async fn extra_or_returns_value_when_found() {
    let props = Props::default().with_prop(PropsOp::set_extra("ext.flag", true));
    assert!(props.extra_or("ext.flag", false));
}

#[pagetop::test]
async fn extra_or_returns_default_on_missing() {
    let props = Props::default();
    assert!(!props.extra_or("ext.flag", false));
}

#[pagetop::test]
async fn extra_or_returns_default_on_type_mismatch() {
    let props = Props::default().with_prop(PropsOp::set_extra("ext.flag", true));
    assert_eq!(props.extra_or("ext.flag", 0_u32), 0_u32);
}

// **< Props::extra_or_default >********************************************************************

#[pagetop::test]
async fn extra_or_default_returns_type_default_on_missing() {
    let props = Props::default();
    assert!(!props.extra_or_default::<bool>("ext.flag"));
    assert_eq!(props.extra_or_default::<i32>("ext.count"), 0);
}

// **< Props::extra_or_else >***********************************************************************

#[pagetop::test]
async fn extra_or_else_calls_closure_on_missing() {
    let props = Props::default();
    assert_eq!(
        props.extra_or_else("ext.label", || "fallback".to_string()),
        "fallback"
    );
}

// **< Clone and mixed types >**********************************************************************

#[pagetop::test]
async fn clone_preserves_extras() {
    let props = Props::default().with_prop(PropsOp::set_extra("ext.flag", true));
    let cloned = props.clone();
    assert!(*cloned.extra::<bool>("ext.flag").unwrap());
}

#[pagetop::test]
async fn multiple_extras_with_different_types() {
    let props = Props::default()
        .with_prop(PropsOp::set_extra("ext.flag", true))
        .with_prop(PropsOp::set_extra("ext.count", 42_u32))
        .with_prop(PropsOp::set_extra("ext.label", "hello".to_string()));

    assert!(*props.extra::<bool>("ext.flag").unwrap());
    assert_eq!(*props.extra::<u32>("ext.count").unwrap(), 42);
    assert_eq!(props.extra::<String>("ext.label").unwrap(), "hello");
}

#[pagetop::test]
async fn extras_not_emitted_in_html() {
    let props = Props::default()
        .with_prop(PropsOp::set_extra("ext.flag", true))
        .with_prop(PropsOp::add_classes("btn"));
    assert_eq!(
        html! { button (props) { "OK" } }.into_string(),
        r#"<button class="btn">OK</button>"#
    );
}
