use pagetop_htmx::prelude::*;

// **< HTTP Methods >*******************************************************************************

#[pagetop::test]
async fn http_method_constants_match_the_htmx_attribute_names() {
    assert_eq!(hx::GET, "hx-get");
    assert_eq!(hx::POST, "hx-post");
    assert_eq!(hx::PUT, "hx-put");
    assert_eq!(hx::PATCH, "hx-patch");
    assert_eq!(hx::DELETE, "hx-delete");
}

// **< Target and Swap >****************************************************************************

#[pagetop::test]
async fn target_and_swap_constants_match_the_htmx_attribute_names() {
    assert_eq!(hx::TARGET, "hx-target");
    assert_eq!(hx::SWAP, "hx-swap");
    assert_eq!(hx::SWAP_OOB, "hx-swap-oob");
    assert_eq!(hx::SELECT, "hx-select");
    assert_eq!(hx::SELECT_OOB, "hx-select-oob");
}

// **< Trigger >************************************************************************************

#[pagetop::test]
async fn trigger_related_constants_match_the_htmx_attribute_names() {
    assert_eq!(hx::TRIGGER, "hx-trigger");
    assert_eq!(hx::BOOST, "hx-boost");
    assert_eq!(hx::PUSH_URL, "hx-push-url");
    assert_eq!(hx::REPLACE_URL, "hx-replace-url");
    assert_eq!(hx::SYNC, "hx-sync");
}

// **< Request Data >*******************************************************************************

#[pagetop::test]
async fn request_data_constants_match_the_htmx_attribute_names() {
    assert_eq!(hx::INCLUDE, "hx-include");
    assert_eq!(hx::PARAMS, "hx-params");
    assert_eq!(hx::VALS, "hx-vals");
    assert_eq!(hx::HEADERS, "hx-headers");
    assert_eq!(hx::ENCODING, "hx-encoding");
}

// **< Element Behavior >***************************************************************************

#[pagetop::test]
async fn element_behavior_constants_match_the_htmx_attribute_names() {
    assert_eq!(hx::INDICATOR, "hx-indicator");
    assert_eq!(hx::DISABLED_ELT, "hx-disabled-elt");
    assert_eq!(hx::CONFIRM, "hx-confirm");
    assert_eq!(hx::PROMPT, "hx-prompt");
    assert_eq!(hx::VALIDATE, "hx-validate");
    assert_eq!(hx::PRESERVE, "hx-preserve");
}

// **< Config and Extensions >**********************************************************************

#[pagetop::test]
async fn config_and_extension_constants_match_the_htmx_attribute_names() {
    assert_eq!(hx::EXT, "hx-ext");
    assert_eq!(hx::DISINHERIT, "hx-disinherit");
    assert_eq!(hx::INHERIT, "hx-inherit");
    assert_eq!(hx::REQUEST, "hx-request");
    assert_eq!(hx::HISTORY, "hx-history");
    assert_eq!(hx::HISTORY_ELT, "hx-history-elt");
    assert_eq!(hx::DISABLE, "hx-disable");
}

// **< Inline Events (hx-on) >**********************************************************************

#[pagetop::test]
async fn on_builds_the_dom_event_attribute_name() {
    assert_eq!(hx::on("click"), "hx-on:click");
    assert_eq!(hx::on("mouseenter"), "hx-on:mouseenter");
}

#[pagetop::test]
async fn on_htmx_builds_the_htmx_lifecycle_event_attribute_name() {
    assert_eq!(hx::on_htmx("before-request"), "hx-on::before-request");
    assert_eq!(hx::on_htmx("after-swap"), "hx-on::after-swap");
}

#[pagetop::test]
async fn on_and_on_htmx_use_a_different_separator_for_the_same_event_name() {
    // The single/double colon is the only thing that distinguishes a native DOM event from an
    // HTMX lifecycle event with the same name; a typo here would silently listen to the wrong one.
    let event = "after-swap";
    assert_ne!(hx::on(event), hx::on_htmx(event));
    assert_eq!(hx::on(event), "hx-on:after-swap");
    assert_eq!(hx::on_htmx(event), "hx-on::after-swap");
}

// **< HTMX Request Headers (hx::request) >*********************************************************

#[pagetop::test]
async fn request_header_constants_match_the_lowercase_htmx_header_names() {
    assert_eq!(hx::request::REQUEST, "hx-request");
    assert_eq!(hx::request::BOOSTED, "hx-boosted");
    assert_eq!(hx::request::CURRENT_URL, "hx-current-url");
    assert_eq!(
        hx::request::HISTORY_RESTORE_REQUEST,
        "hx-history-restore-request"
    );
    assert_eq!(hx::request::PROMPT, "hx-prompt");
    assert_eq!(hx::request::TARGET, "hx-target");
    assert_eq!(hx::request::TRIGGER, "hx-trigger");
    assert_eq!(hx::request::TRIGGER_NAME, "hx-trigger-name");
}

// **< HTMX Response Headers (hx::response) >*******************************************************

#[pagetop::test]
async fn response_header_constants_match_the_capitalized_htmx_header_names() {
    // Unlike the request headers, HTMX documents the response headers in their canonical
    // capitalized form (`HX-Location`, not `hx-location`); the constants mirror that on purpose.
    assert_eq!(hx::response::LOCATION, "HX-Location");
    assert_eq!(hx::response::PUSH_URL, "HX-Push-Url");
    assert_eq!(hx::response::REDIRECT, "HX-Redirect");
    assert_eq!(hx::response::REFRESH, "HX-Refresh");
    assert_eq!(hx::response::REPLACE_URL, "HX-Replace-Url");
    assert_eq!(hx::response::RESWAP, "HX-Reswap");
    assert_eq!(hx::response::RETARGET, "HX-Retarget");
    assert_eq!(hx::response::RESELECT, "HX-Reselect");
    assert_eq!(hx::response::TRIGGER, "HX-Trigger");
    assert_eq!(
        hx::response::TRIGGER_AFTER_SETTLE,
        "HX-Trigger-After-Settle"
    );
    assert_eq!(hx::response::TRIGGER_AFTER_SWAP, "HX-Trigger-After-Swap");
}

// **< hx-swap Values (hx::swap) >******************************************************************

#[pagetop::test]
async fn swap_value_constants_match_the_htmx_swap_strategies() {
    assert_eq!(hx::swap::INNER_HTML, "innerHTML");
    assert_eq!(hx::swap::OUTER_HTML, "outerHTML");
    assert_eq!(hx::swap::BEFORE_BEGIN, "beforebegin");
    assert_eq!(hx::swap::AFTER_BEGIN, "afterbegin");
    assert_eq!(hx::swap::BEFORE_END, "beforeend");
    assert_eq!(hx::swap::AFTER_END, "afterend");
    assert_eq!(hx::swap::DELETE, "delete");
    assert_eq!(hx::swap::NONE, "none");
}

// **< hx-trigger Values (hx::trigger) >************************************************************

#[pagetop::test]
async fn trigger_value_constants_match_the_htmx_event_names() {
    assert_eq!(hx::trigger::CLICK, "click");
    assert_eq!(hx::trigger::CHANGE, "change");
    assert_eq!(hx::trigger::SUBMIT, "submit");
    assert_eq!(hx::trigger::KEYUP, "keyup");
    assert_eq!(hx::trigger::LOAD, "load");
    assert_eq!(hx::trigger::REVEALED, "revealed");
    assert_eq!(hx::trigger::INTERSECT, "intersect");
}
