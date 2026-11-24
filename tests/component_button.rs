use pagetop::prelude::*;

// **< Button >*************************************************************************************

#[pagetop::test]
async fn label_is_rendered_when_set() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("Save"));
}

#[pagetop::test]
async fn label_can_be_cleared_with_lc_none() {
    let mut button = Button::submit(Lc::n("Save")).with_label(Lc::none());
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("Save"));
    // The button itself must still render.
    assert!(html.contains("<button"));
}

#[pagetop::test]
async fn title_attribute_is_absent_by_default() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("title="));
}

#[pagetop::test]
async fn title_attribute_is_present_when_set() {
    let mut button = Button::submit(Lc::n("Save")).with_title(Lc::n("Save changes"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"title="Save changes""#));
}

#[pagetop::test]
async fn title_attribute_can_be_cleared_with_lc_none() {
    let mut button = Button::submit(Lc::n("Save"))
        .with_title(Lc::n("Save changes"))
        .with_title(Lc::none());
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("title="));
}

// The hidden label completes the visible one for screen readers, right after it and escaped.
#[pagetop::test]
async fn hidden_label_follows_the_visible_label() {
    let mut button = Button::delete().with_hidden_label(Lc::n("<Admins>"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(
        html.contains(r#"Delete<span class="visually-hidden"> &lt;Admins&gt;</span></button>"#),
        "{html}"
    );
}

#[pagetop::test]
async fn hidden_label_is_absent_by_default_and_when_empty() {
    for mut button in [
        Button::edit("/items/1/edit"),
        Button::edit("/items/1/edit").with_hidden_label(Lc::n("")),
        Button::edit("/items/1/edit")
            .with_hidden_label(Lc::n("Item 1"))
            .with_hidden_label(Lc::none()),
    ] {
        let html = button.render(&mut Context::default()).await.into_string();
        assert!(!html.contains("visually-hidden"), "{html}");
    }
}

// With icons, the hidden label stays attached to the label, before the end icon.
#[pagetop::test]
async fn hidden_label_goes_before_the_end_icon() {
    let mut button = Button::plain(Lc::n("Next"))
        .with_hidden_label(Lc::n("page"))
        .with_icon_end(Icon::with(icon::Core::ChevronRight));
    let html = button.render(&mut Context::default()).await.into_string();

    let hidden = html.find("visually-hidden").expect(&html);
    let icon = html.rfind("<svg").expect(&html);
    assert!(hidden < icon, "{html}");
}

#[pagetop::test]
async fn style_class_reflects_intent_and_style() {
    let mut button = Button::submit(Lc::n("Save")).with_style(button::Style::Solid(Intent::Severe));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-severe"));
}

#[pagetop::test]
async fn outline_style_generates_outline_class() {
    let mut button =
        Button::submit(Lc::n("Save")).with_style(button::Style::Outline(Intent::Primary));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-outline-primary"));
}

#[pagetop::test]
async fn link_style_generates_link_class_without_intent() {
    let mut button = Button::plain(Lc::n("Cancel")).with_style(button::Style::Link);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-link"));
}

#[pagetop::test]
async fn size_small_generates_button_sm_class() {
    let mut button = Button::submit(Lc::n("Save")).with_size(button::Size::Small);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-sm"));
}

#[pagetop::test]
async fn size_large_generates_button_lg_class() {
    let mut button = Button::submit(Lc::n("Save")).with_size(button::Size::Large);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-lg"));
}

#[pagetop::test]
async fn size_none_omits_size_class_by_default() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("button-sm"));
    assert!(!html.contains("button-lg"));
}

#[pagetop::test]
async fn anchor_renders_as_a_tag_with_href() {
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit");
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<a "));
    assert!(html.contains(r#"href="/items/1/edit""#));
    assert!(html.contains("Edit"));
}

#[pagetop::test]
async fn button_without_href_renders_as_button_tag() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<button"));
}

#[pagetop::test]
async fn anchor_falls_back_to_button_when_href_is_reset() {
    // `Route::default()` resolves to an empty path; `prepare()` must ignore `href` and render a
    // `<button>` again, as documented in `Button::with_href()`.
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit").with_href(Route::default());
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<button"));
}

#[pagetop::test]
async fn enabled_anchor_has_no_aria_disabled_or_tabindex() {
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit");
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("aria-disabled"));
    assert!(!html.contains("tabindex"));
}

#[pagetop::test]
async fn disabled_anchor_omits_href_and_sets_aria_disabled() {
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit").with_disabled(true);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("href="));
    assert!(html.contains(r#"aria-disabled="true""#));
    assert!(html.contains(r#"tabindex="-1""#));
}

#[pagetop::test]
async fn anchor_blank_opens_in_a_new_window() {
    let mut button = Button::anchor(Lc::n("Docs"), "https://docs.rs");
    let html = button.render(&mut Context::default()).await.into_string();
    assert!(!html.contains("target="), "{html}");

    let mut button = Button::anchor_blank(Lc::n("Docs"), "https://docs.rs");
    let html = button.render(&mut Context::default()).await.into_string();
    assert!(html.starts_with("<a "), "{html}");
    assert!(html.contains(r#"target="_blank""#));
    assert!(html.contains(r#"rel="noopener noreferrer""#));

    // Disabled, it has no link to open.
    let mut button = Button::anchor_blank(Lc::n("Docs"), "https://docs.rs").with_disabled(true);
    let html = button.render(&mut Context::default()).await.into_string();
    assert!(!html.contains("target="), "{html}");
    assert!(!html.contains("rel="));
}

// The save button of a form: a translated submit button with the primary style.
#[pagetop::test]
async fn save_is_a_translated_primary_submit_button() {
    for (language, text) in [("en-US", "Save"), ("es-ES", "Guardar")] {
        let mut cx = Context::default().with_langid(&Locale::resolve(language));
        let html = Button::save().render(&mut cx).await.into_string();
        assert!(html.starts_with("<button"), "{html}");
        assert!(html.contains(r#"type="submit""#), "{html}");
        assert!(html.contains("button-primary"), "{html}");
        assert!(html.contains(text), "{language}: {html}");
    }
}

// Edit is a translated primary link; delete a translated severe plain button, without a link.
#[pagetop::test]
async fn edit_is_a_link_and_delete_a_plain_button() {
    for (language, edit, delete) in [("en-US", "Edit", "Delete"), ("es-ES", "Editar", "Eliminar")] {
        let mut cx = Context::default().with_langid(&Locale::resolve(language));
        let html = Button::edit("/items/1/edit")
            .render(&mut cx)
            .await
            .into_string();
        assert!(html.starts_with("<a "), "{html}");
        assert!(html.contains(r#"href="/items/1/edit"#), "{html}");
        assert!(html.contains("button-primary"), "{html}");
        assert!(html.contains(edit), "{language}: {html}");

        let html = Button::delete().render(&mut cx).await.into_string();
        assert!(html.starts_with("<button"), "{html}");
        assert!(html.contains(r#"type="button""#), "{html}");
        assert!(!html.contains("href"), "{html}");
        assert!(html.contains("button-severe"), "{html}");
        assert!(html.contains(delete), "{language}: {html}");
    }
}

// The cancel button of a dialog: a translated plain button that closes it, not a link.
#[pagetop::test]
async fn dismiss_is_a_plain_button_that_closes_the_dialog() {
    for (language, text) in [("en-US", "Cancel"), ("es-ES", "Cancelar")] {
        let mut cx = Context::default().with_langid(&Locale::resolve(language));
        let html = Button::dismiss().render(&mut cx).await.into_string();
        assert!(html.starts_with("<button"), "{html}");
        assert!(html.contains(r#"type="button""#), "{html}");
        assert!(html.contains(r#"data-dialog-dismiss="modal""#), "{html}");
        assert!(!html.contains("href"), "{html}");
        assert!(html.contains("button-outline-neutral"), "{html}");
        assert!(html.contains(text), "{language}: {html}");
    }
}

// The links to leave a form or a detail page: a translated text and the link style.
#[pagetop::test]
async fn cancel_and_back_are_translated_links() {
    for (language, cancel, back) in [("en-US", "Cancel", "Back"), ("es-ES", "Cancelar", "Volver")] {
        let mut cx = Context::default().with_langid(&Locale::resolve(language));
        for (mut button, text) in [
            (Button::cancel("/items"), cancel),
            (Button::back("/items"), back),
        ] {
            let html = button.render(&mut cx).await.into_string();
            assert!(html.starts_with("<a "), "{html}");
            assert!(html.contains(r#"href="/items"#), "{html}");
            assert!(html.contains("button-link"), "{html}");
            assert!(html.contains(text), "{language}: {html}");
        }
    }
}

// A disabled `<a>` has no native equivalent to `:disabled` that CSS can hook into, unlike a
// `<button disabled>`; the `disabled` class is what both Basic (`.button.disabled`) and Bootsier
// (Bootstrap's own `.btn.disabled`) actually style.
#[pagetop::test]
async fn disabled_anchor_gets_the_disabled_class() {
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit").with_disabled(true);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="button disabled""#));
}

#[pagetop::test]
async fn disabled_button_also_gets_the_disabled_class() {
    let mut button = Button::submit(Lc::n("Save")).with_disabled(true);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="button disabled""#));
}

// `ListFilter` relies on this for its reset link (`target="_top"`).
#[pagetop::test]
async fn anchor_keeps_a_target_set_with_props() {
    let mut button =
        Button::anchor(Lc::n("Clear"), "/items").with_prop(PropsOp::set("target", "_top"));
    let html = button.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"target="_top""#), "{html}");
}
