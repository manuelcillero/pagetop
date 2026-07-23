use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

// Forces an effective language different from the default negotiated one (en-US, with no `?lang` in
// the request), so that `Context::route()` decides to propagate `?lang=...` in local routes.
fn cx_with_lang(lang: &str) -> Context {
    Context::new(None).with_langid(&Locale::resolve(lang))
}

fn header<'a>(response: &'a web::Response, name: &str) -> Option<&'a str> {
    response.headers().get(name)?.to_str().ok()
}

// **< HtmxResponse::new() / empty() >**************************************************************

#[pagetop::test]
async fn new_renders_the_given_markup_with_an_html_content_type() {
    let response = HtmxResponse::new(html! { li #item-42 { "New item" } }).into_response();

    assert_eq!(
        header(&response, "content-type"),
        Some("text/html; charset=utf-8")
    );

    let body = web::test::read_body_text(response).await;
    assert_eq!(body, r#"<li id="item-42">New item</li>"#);
}

#[pagetop::test]
async fn empty_has_no_body_but_keeps_the_html_content_type() {
    let response = HtmxResponse::empty().into_response();

    assert_eq!(
        header(&response, "content-type"),
        Some("text/html; charset=utf-8")
    );

    let body = web::test::read_body_text(response).await;
    assert_eq!(body, "");
}

// **< location() / location_json() >***************************************************************

#[pagetop::test]
async fn location_sets_hx_location_from_a_route_path() {
    let response = HtmxResponse::empty().location("/items").into_response();

    assert_eq!(header(&response, "hx-location"), Some("/items"));
}

#[pagetop::test]
async fn location_preserves_lang_when_built_from_context_route() {
    let cx = cx_with_lang("es-ES");

    let response = HtmxResponse::empty()
        .location(cx.route("/items"))
        .into_response();

    assert_eq!(header(&response, "hx-location"), Some("/items?lang=es-ES"));
}

#[pagetop::test]
async fn location_json_sets_hx_location_when_the_json_is_syntactically_valid() {
    let json = r##"{"path": "/items", "target": "#content"}"##;

    let response = HtmxResponse::empty().location_json(json).into_response();

    assert_eq!(header(&response, "hx-location"), Some(json));
}

#[pagetop::test]
async fn location_json_discards_the_header_when_the_json_is_malformed() {
    // Missing closing brace: invalid JSON. The header must be silently dropped rather than sending
    // a broken payload to the client.
    let response = HtmxResponse::empty()
        .location_json(r##"{"path": "/items""##)
        .into_response();

    assert_eq!(header(&response, "hx-location"), None);
}

#[pagetop::test]
async fn location_json_only_validates_syntax_not_the_expected_keys() {
    // A key HTMX does not recognize (`"tagret"` instead of `"target"`) is still valid JSON, so it
    // passes this check; the mistake would only surface client-side. This documents that limit.
    let json = r##"{"path": "/items", "tagret": "#content"}"##;

    let response = HtmxResponse::empty().location_json(json).into_response();

    assert_eq!(header(&response, "hx-location"), Some(json));
}

// **< push_url() / replace_url() / redirect() >****************************************************

#[pagetop::test]
async fn push_url_sets_hx_push_url_from_a_route_path() {
    let cx = cx_with_lang("es-ES");

    let response = HtmxResponse::empty()
        .push_url(cx.route("/items"))
        .into_response();

    assert_eq!(header(&response, "hx-push-url"), Some("/items?lang=es-ES"));
}

#[pagetop::test]
async fn push_url_accepts_the_false_sentinel_to_disable_pushing() {
    let response = HtmxResponse::empty().push_url("false").into_response();

    assert_eq!(header(&response, "hx-push-url"), Some("false"));
}

#[pagetop::test]
async fn replace_url_sets_hx_replace_url_from_a_route_path() {
    let response = HtmxResponse::empty()
        .replace_url("/items/42")
        .into_response();

    assert_eq!(header(&response, "hx-replace-url"), Some("/items/42"));
}

#[pagetop::test]
async fn redirect_sets_hx_redirect_from_a_route_path() {
    let cx = cx_with_lang("es-ES");

    let response = HtmxResponse::empty()
        .redirect(cx.route("/items"))
        .into_response();

    assert_eq!(header(&response, "hx-redirect"), Some("/items?lang=es-ES"));
}

// **< refresh() / retarget() / reswap() / reselect() >*********************************************

#[pagetop::test]
async fn refresh_sets_hx_refresh_to_true() {
    let response = HtmxResponse::empty().refresh().into_response();

    assert_eq!(header(&response, "hx-refresh"), Some("true"));
}

#[pagetop::test]
async fn retarget_reswap_and_reselect_set_the_expected_headers() {
    let response = HtmxResponse::empty()
        .retarget("#message")
        .reswap(hx::swap::BEFORE_END)
        .reselect("#fragment")
        .into_response();

    assert_eq!(header(&response, "hx-retarget"), Some("#message"));
    assert_eq!(header(&response, "hx-reswap"), Some("beforeend"));
    assert_eq!(header(&response, "hx-reselect"), Some("#fragment"));
}

// **< trigger() / trigger_after_settle() / trigger_after_swap() >**********************************

#[pagetop::test]
async fn trigger_accepts_a_single_event_name() {
    let response = HtmxResponse::empty().trigger("itemAdded").into_response();

    assert_eq!(header(&response, "hx-trigger"), Some("itemAdded"));
}

#[pagetop::test]
async fn trigger_accepts_multiple_comma_separated_events() {
    let response = HtmxResponse::empty()
        .trigger("itemAdded, listUpdated")
        .into_response();

    assert_eq!(
        header(&response, "hx-trigger"),
        Some("itemAdded, listUpdated")
    );
}

#[pagetop::test]
async fn trigger_accepts_a_json_payload_with_event_data() {
    let json = r#"{"itemAdded": {"id": 42, "name": "Example"}}"#;

    let response = HtmxResponse::empty().trigger(json).into_response();

    assert_eq!(header(&response, "hx-trigger"), Some(json));
}

#[pagetop::test]
async fn trigger_after_settle_and_trigger_after_swap_use_their_own_headers() {
    let response = HtmxResponse::empty()
        .trigger_after_settle("settled")
        .trigger_after_swap("swapped")
        .into_response();

    assert_eq!(
        header(&response, "hx-trigger-after-settle"),
        Some("settled")
    );
    assert_eq!(header(&response, "hx-trigger-after-swap"), Some("swapped"));
}

// **< Builder chaining behavior >******************************************************************

#[pagetop::test]
async fn chaining_several_methods_sets_all_their_headers_at_once() {
    let response = HtmxResponse::new(html! { ul { li { "Item 1" } li { "Item 2" } } })
        .retarget("#list")
        .reswap(hx::swap::BEFORE_END)
        .push_url("/items")
        .trigger("itemAdded")
        .into_response();

    assert_eq!(header(&response, "hx-retarget"), Some("#list"));
    assert_eq!(header(&response, "hx-reswap"), Some("beforeend"));
    assert_eq!(header(&response, "hx-push-url"), Some("/items"));
    assert_eq!(header(&response, "hx-trigger"), Some("itemAdded"));
}

#[pagetop::test]
async fn calling_the_same_method_twice_the_last_call_wins() {
    let response = HtmxResponse::empty()
        .trigger("first")
        .trigger("second")
        .into_response();

    assert_eq!(header(&response, "hx-trigger"), Some("second"));
}

#[pagetop::test]
async fn a_header_value_with_control_characters_is_silently_discarded() {
    // `\n` is forbidden in an HTTP header value; `set_header()` must drop it rather than panicking
    // or producing a malformed response.
    let response = HtmxResponse::empty().retarget("foo\nbar").into_response();

    assert_eq!(header(&response, "hx-retarget"), None);
}
