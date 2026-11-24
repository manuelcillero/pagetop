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
const DIALOG_TARGET: &str = r##"data-dialog-target="#hx-confirm-dialog""##;
const QUESTION_ID: &str = "hx-confirm-dialog-question";

#[pagetop::test]
async fn confirm_opens_the_shared_dialog_with_the_question() {
    let mut button = hx_confirm::confirm(
        Button::delete()
            .with_hidden_label(Lc::n("A1"))
            .with_prop(PropsOp::set(hx::POST, "/plots/1/delete")),
        "Delete the plot «A1»?",
    );
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"hx-post="/plots/1/delete""#), "{html}");
    assert!(
        html.contains(r#"hx-confirm="Delete the plot «A1»?""#),
        "{html}"
    );
    assert!(html.contains(r#"data-hx-confirm="true""#), "{html}");
    assert!(html.contains(r#"data-dialog-toggle="modal""#), "{html}");
    assert!(html.contains(DIALOG_TARGET), "{html}");
}

#[pagetop::test]
async fn confirm_escapes_the_question() {
    let mut button = hx_confirm::confirm(Button::delete(), r#"Delete "<b>"?"#);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(
        html.contains(r#"hx-confirm="Delete &quot;&lt;b&gt;&quot;?""#),
        "{html}"
    );
}

#[pagetop::test]
async fn the_dialog_is_named_described_and_starts_on_cancel() {
    for (language, title, cancel, delete) in [
        ("en-US", "Confirm deletion", "Cancel", "Delete"),
        ("es-ES", "Confirmar la eliminación", "Cancelar", "Eliminar"),
    ] {
        let mut cx = cx_with_lang(language);
        let html = hx_confirm::dialog().render(&mut cx).await.into_string();

        assert!(html.contains(r#"id="hx-confirm-dialog""#), "{html}");
        assert!(
            html.contains(r#"aria-labelledby="hx-confirm-dialog-label""#),
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
