use pagetop::prelude::*;

// The help text of a field gets an id and the control points to it with `aria-describedby`, so
// screen readers announce it when the control gets the focus.

async fn render(component: &mut impl Component) -> String {
    component
        .render(&mut Context::default())
        .await
        .into_string()
}

fn help() -> Lc {
    Lc::n("Help")
}

// **< Form fields >********************************************************************************

#[pagetop::test]
async fn single_controls_point_to_their_help_text() {
    let cases: [(String, &str); 5] = [
        (
            render(
                &mut form::input::Field::text()
                    .with_name("a")
                    .with_help_text(help()),
            )
            .await,
            "edit-a-input-help",
        ),
        (
            render(&mut form::Number::new().with_name("b").with_help_text(help())).await,
            "edit-b-input-help",
        ),
        (
            render(&mut form::Range::new().with_name("c").with_help_text(help())).await,
            "edit-c-range-help",
        ),
        (
            render(&mut form::Textarea::new().with_name("d").with_help_text(help())).await,
            "edit-d-textarea-help",
        ),
        (
            render(
                &mut form::select::Field::new()
                    .with_name("e")
                    .with_help_text(help()),
            )
            .await,
            "edit-e-select-help",
        ),
    ];
    for (html, id) in cases {
        assert!(
            html.contains(&format!(r#"aria-describedby="{id}""#)),
            "{html}"
        );
        assert!(
            html.contains(&format!(r#"<div id="{id}" class="form-text">Help</div>"#)),
            "{html}"
        );
    }
}

#[pagetop::test]
async fn without_help_text_there_is_no_aria_describedby() {
    let html = render(&mut form::input::Field::text().with_name("a")).await;
    assert!(!html.contains("aria-describedby"), "{html}");
}

#[pagetop::test]
async fn date_fields_point_to_their_format_and_their_help_text() {
    let html = render(
        &mut form::date::Field::date()
            .with_name("f")
            .with_help_text(help()),
    )
    .await;
    assert!(html.contains(r#"aria-describedby="edit-f-input-format edit-f-input-help""#));
    assert!(html.contains(r#"<div id="edit-f-input-help" class="form-text">Help</div>"#));
}

#[pagetop::test]
async fn checks_and_radios_are_labelled_groups_with_help_text() {
    let check = render(
        &mut form::check::Field::new()
            .with_name("g")
            .with_label(Lc::n("Options"))
            .with_help_text(help())
            .with_item(form::check::Item::new("1", Lc::n("One"))),
    )
    .await;
    let radio = render(
        &mut form::radio::Field::new()
            .with_name("h")
            .with_label(Lc::n("Choice"))
            .with_help_text(help())
            .with_item(form::radio::Item::new("1", Lc::n("One"))),
    )
    .await;
    for (html, id) in [(check, "edit-g"), (radio, "edit-h")] {
        assert!(html.contains(r#"role="group""#), "{html}");
        assert!(
            html.contains(&format!(r#"aria-labelledby="{id}-label""#)),
            "{html}"
        );
        assert!(
            html.contains(&format!(r#"aria-describedby="{id}-help""#)),
            "{html}"
        );
        assert!(
            html.contains(&format!(r#"<label id="{id}-label" class="form-label">"#)),
            "{html}"
        );
        assert!(
            html.contains(&format!(r#"<div id="{id}-help" class="form-text">"#)),
            "{html}"
        );
    }
}

// **< form::parts::Help >**************************************************************************

#[pagetop::test]
async fn help_links_only_when_there_is_text_and_an_anchor() {
    let cx = Context::default();
    let text = help();
    let none = Lc::none();

    let field = form::parts::Help::new(&text, Some("edit-x-input"));
    assert_eq!(field.id(), Some("edit-x-input-help"));
    assert_eq!(
        field.render(&cx).into_string(),
        r#"<div id="edit-x-input-help" class="form-text">Help</div>"#
    );

    // Without an anchor the text is still shown, but there is no id to point to.
    let field = form::parts::Help::new(&text, None);
    assert_eq!(field.id(), None);
    assert_eq!(
        field.render(&cx).into_string(),
        r#"<div class="form-text">Help</div>"#
    );

    // Without text there is nothing to show nor to link.
    let field = form::parts::Help::new(&none, Some("edit-x-input"));
    assert_eq!(field.id(), None);
    assert_eq!(field.render(&cx).into_string(), "");

    let format = form::parts::Help::suffixed(&text, Some("edit-x-input"), "-format");
    assert_eq!(format.id(), Some("edit-x-input-format"));
}

#[pagetop::test]
async fn help_class_can_be_replaced() {
    let cx = Context::default();
    let text = help();
    let field = form::parts::Help::new(&text, Some("edit-x-input")).with_class("hint small");
    assert_eq!(
        field.render(&cx).into_string(),
        r#"<div id="edit-x-input-help" class="hint small">Help</div>"#
    );
}

#[pagetop::test]
async fn help_without_translation_keeps_the_linked_element() {
    let cx = Context::default();
    // A missing translation still renders the element, so `aria-describedby` never dangles.
    let missing = Lc::l("no_such_key_for_help");
    let field = form::parts::Help::new(&missing, Some("edit-x-input"));
    assert_eq!(field.id(), Some("edit-x-input-help"));
    assert_eq!(
        field.render(&cx).into_string(),
        r#"<div id="edit-x-input-help" class="form-text"></div>"#
    );
}
