use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

// Forces an effective language different from the default negotiated one (en-US, with no `?lang` in
// the request), so that `Context::route()` decides to propagate `?lang=...` in local routes.
fn cx_with_lang(lang: &str) -> Context {
    Context::default().with_langid(&Locale::resolve(lang))
}

// **< hx_pager::boost() >**************************************************************************

#[pagetop::test]
async fn boost_targets_the_list_from_the_pager_nav() {
    let mut pager = hx_pager::boost(
        Pager::new()
            .with_base_path("/items")
            .with_current_page(1)
            .with_items_per_page(10)
            .with_total_items(50),
        "#item-list",
    );
    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"hx-boost="true""#), "{html}");
    assert!(html.contains(r##"hx-target="#item-list""##), "{html}");
    assert!(html.contains(r#"hx-swap="outerHTML scroll:top""#), "{html}");
    assert!(html.contains(r#"hx-push-url="true""#), "{html}");
}

// **< hx_filter::boost() >*************************************************************************

#[pagetop::test]
async fn boost_sends_the_filter_form_to_the_list() {
    let mut filter = hx_filter::boost(
        ListFilter::new()
            .with_filter_action("/items")
            .with_field(form::input::Field::search().with_name("q")),
        "#item-list",
    );
    let html = filter.render(&mut Context::default()).await.into_string();

    // `hx-boost` on the form sends its fields to its own `action`, so no `hx-get` is needed.
    assert!(html.contains(r#"hx-boost="true""#), "{html}");
    assert!(!html.contains("hx-get"), "{html}");
    // When it applies is up to the filter (button or auto submit), not to HTMX.
    assert!(!html.contains("hx-trigger"), "{html}");
    assert!(html.contains(r##"hx-target="#item-list""##), "{html}");
    assert!(html.contains(r#"hx-push-url="true""#), "{html}");
}

#[pagetop::test]
async fn boosted_filter_keeps_the_language_in_a_hidden_field() {
    // A boosted GET form drops the query of its `action`, the same as the browser does.
    let mut filter = hx_filter::boost(
        ListFilter::new()
            .with_filter_action("/items")
            .with_field(form::input::Field::search().with_name("q")),
        "#item-list",
    );
    let html = filter
        .render(&mut cx_with_lang("es-ES"))
        .await
        .into_string();

    assert!(html.contains(r#"name="lang""#), "{html}");
    assert!(html.contains(r#"value="es-ES""#), "{html}");
}

// **< Overlapping requests and loading indicator >*************************************************

#[pagetop::test]
async fn list_helpers_cancel_overlapping_requests_and_mark_the_list_while_loading() {
    let mut pager = hx_pager::boost(
        Pager::new()
            .with_base_path("/items")
            .with_current_page(1)
            .with_items_per_page(10)
            .with_total_items(50),
        "#item-list",
    );
    let mut filter = hx_filter::boost(ListFilter::new().with_filter_action("/items"), "#item-list");
    let mut table = Table::new().with_column(
        table::Column::new(Lc::n("Name")).with_sort(hx_table::sort_link(
            "/items",
            "#item-list",
            None,
        )),
    );

    let mut cx = Context::default();
    for html in [
        pager.render(&mut cx).await.into_string(),
        filter.render(&mut cx).await.into_string(),
        table.render(&mut cx).await.into_string(),
    ] {
        // Every request against the list cancels the one in progress, wherever it comes from.
        assert!(html.contains(r##"hx-sync="#item-list:replace""##), "{html}");
        assert!(html.contains(r##"hx-indicator="#item-list""##), "{html}");
    }
}

// **< hx_confirm >*********************************************************************************

// The ids the script (`hx-confirm.js`) relies on.
const DIALOG_TARGET: &str = r##"data-dialog-target="#hx-confirm-delete""##;
const QUESTION_ID: &str = "hx-confirm-delete-question";

#[pagetop::test]
async fn confirm_opens_the_shared_dialog_with_the_question() {
    let mut button = hx_confirm::confirm_delete(
        Button::delete()
            .with_hidden_label(Lc::n("A1"))
            .with_prop(PropsOp::set(hx::POST, "/plots/1/delete")),
        Lc::n("Delete the plot «A1»?"),
    );
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"hx-post="/plots/1/delete""#), "{html}");
    assert!(
        html.contains(r#"hx-confirm="Delete the plot «A1»?""#),
        "{html}"
    );
    assert!(html.contains(r#"data-hx-confirm="delete""#), "{html}");
    assert!(html.contains(r#"data-dialog-toggle="modal""#), "{html}");
    assert!(html.contains(DIALOG_TARGET), "{html}");
}

#[pagetop::test]
async fn confirm_escapes_the_question() {
    let mut button = hx_confirm::confirm_delete(Button::delete(), Lc::n(r#"Delete "<b>"?"#));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(
        html.contains(r#"hx-confirm="Delete &quot;&lt;b&gt;&quot;?""#),
        "{html}"
    );
}

#[pagetop::test]
async fn confirm_translates_the_question_into_the_language_of_the_request() {
    for (language, question) in [("en-US", "Delete"), ("es-ES", "Eliminar")] {
        let mut button =
            hx_confirm::confirm_delete(Button::plain(Lc::n("x")), Lc::l("button_delete"));
        let html = button
            .render(&mut cx_with_lang(language))
            .await
            .into_string();
        assert!(
            html.contains(&format!(r#" hx-confirm="{question}""#)),
            "{language}: {html}"
        );
    }
}

// Without a translation the button has no question, but it still opens the dialog, which holds the
// request as with any other question.
#[pagetop::test]
async fn a_question_without_translation_leaves_out_hx_confirm() {
    let mut button = hx_confirm::confirm_delete(Button::plain(Lc::n("x")), Lc::l("no_such_key"));
    let html = button.render(&mut Context::default()).await.into_string();
    assert!(!html.contains(" hx-confirm="), "{html}");
    assert!(html.contains(r#"data-hx-confirm="delete""#), "{html}");
    assert!(html.contains(DIALOG_TARGET), "{html}");
}

#[pagetop::test]
async fn the_dialog_is_named_described_and_starts_on_cancel() {
    for (language, title, cancel, delete) in [
        ("en-US", "Confirm deletion", "Cancel", "Delete"),
        ("es-ES", "Confirmar la eliminación", "Cancelar", "Eliminar"),
    ] {
        let mut cx = cx_with_lang(language);
        let html = hx_confirm::dialog_delete()
            .render(&mut cx)
            .await
            .into_string();

        assert!(html.contains(r#"id="hx-confirm-delete""#), "{html}");
        assert!(
            html.contains(r#"aria-labelledby="hx-confirm-delete-label""#),
            "{html}"
        );
        assert!(html.contains(title), "{language}: {html}");
        assert!(
            html.contains(&format!(r#"aria-describedby="{QUESTION_ID}""#)),
            "{html}"
        );
        assert!(
            html.contains(&format!(r#"<p id="{QUESTION_ID}"></p>"#)),
            "{html}"
        );

        // Cancel comes first in the footer and takes the initial focus; accept closes the dialog.
        let cancel_at = html.find(cancel).expect(&html);
        let delete_at = html.rfind(delete).expect(&html);
        assert!(cancel_at < delete_at, "{html}");
        let footer = &html[html.find("dialog-footer").expect(&html)..];
        let first_button = &footer[..footer.find("</button>").expect(footer)];
        assert!(first_button.contains("autofocus"), "{first_button}");
        assert!(
            footer.contains(r#"data-hx-confirm-accept="true""#),
            "{footer}"
        );
        assert_eq!(
            footer.matches(r#"data-dialog-dismiss="modal""#).count(),
            2,
            "{footer}"
        );
    }
}

// A named dialog has its own title and accept button, which only resumes the held request (it must
// not submit a form) and closes the dialog; the button names the dialog it opens.
#[pagetop::test]
async fn a_named_dialog_confirms_its_own_action() {
    let mut dialog = hx_confirm::dialog(
        "block",
        Lc::n("Block account"),
        Button::submit(Lc::n("Block")).with_style(button::Style::Solid(Intent::Severe)),
    );
    let html = dialog.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"id="hx-confirm-block""#), "{html}");
    assert!(html.contains("Block account"), "{html}");
    assert!(
        html.contains(r#"aria-describedby="hx-confirm-block-question""#),
        "{html}"
    );
    assert!(
        html.contains(r#"<p id="hx-confirm-block-question"></p>"#),
        "{html}"
    );
    let footer = &html[html.find("dialog-footer").expect(&html)..];
    let accept = &footer[footer.rfind("<button").expect(footer)..];
    assert!(accept.contains(r#"type="button""#), "{accept}");
    assert!(
        accept.contains(r#"data-hx-confirm-accept="true""#),
        "{accept}"
    );
    assert!(accept.contains("Block"), "{accept}");

    let mut button = hx_confirm::confirm(
        "block",
        Button::submit(Lc::n("Block")).with_prop(PropsOp::set(hx::POST, "/users/7/block")),
        Lc::n("Block the account «ana»?"),
    );
    let html = button.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"type="submit""#), "{html}");
    assert!(
        html.contains(r#"hx-confirm="Block the account «ana»?""#),
        "{html}"
    );
    assert!(html.contains(r#"data-hx-confirm="block""#), "{html}");
    assert!(
        html.contains(r##"data-dialog-target="#hx-confirm-block""##),
        "{html}"
    );
}

#[pagetop::test]
#[should_panic(expected = "must only have ASCII letters, digits")]
async fn a_dialog_name_that_cannot_be_a_selector_is_an_error() {
    let _ = hx_confirm::dialog(
        "block.account",
        Lc::n("Block"),
        Button::plain(Lc::n("Block")),
    );
}

#[pagetop::test]
#[should_panic(expected = "must only have ASCII letters, digits")]
async fn an_empty_dialog_name_is_an_error() {
    let _ = hx_confirm::confirm("", Button::submit(Lc::n("Block")), Lc::n("Block?"));
}
