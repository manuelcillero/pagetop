use pagetop::prelude::*;

// Forces an effective language different from the default negotiated one (en-US, with no `?lang` in
// the request), so that `Context::route()` decides to propagate `?lang=...` in local routes.
fn cx_with_lang(lang: &str) -> Context {
    Context::new(None).with_langid(&Locale::resolve(lang))
}

// **< Route - automatic detection of external URLs >***********************************************

#[pagetop::test]
async fn route_from_str_detects_external_urls_by_prefix() {
    let cx = cx_with_lang("es-ES");

    for url in [
        "http://example.com",
        "https://example.com",
        "//example.com",
        "mailto:user@example.com",
        "tel:+123456789",
    ] {
        let route: Route = url.into();
        assert_eq!(
            route.resolve(&cx).to_string(),
            url,
            "expected {url:?} to be detected as external and left unmodified"
        );
    }
}

#[pagetop::test]
async fn route_from_string_behaves_like_from_str() {
    let cx = cx_with_lang("es-ES");

    let external: Route = String::from("https://example.com").into();
    assert_eq!(external.resolve(&cx).to_string(), "https://example.com");

    let local: Route = String::from("/local/path").into();
    assert_eq!(local.resolve(&cx).to_string(), "/local/path?lang=es-ES");
}

#[pagetop::test]
async fn route_local_path_goes_through_context_route_and_preserves_lang() {
    let cx = cx_with_lang("es-ES");

    let route: Route = "/local/path".into();
    assert_eq!(route.resolve(&cx).to_string(), "/local/path?lang=es-ES");
}

#[pagetop::test]
async fn route_unrecognized_scheme_is_not_detected_as_external() {
    // `ftp://` is not among the prefixes recognized by the heuristic, so the implicit conversion
    // treats it as a local path and goes through `Context::route()`; for a URL like this,
    // `Route::external()` is still necessary (see the test below).
    let cx = cx_with_lang("es-ES");

    let route: Route = "ftp://files.example.com".into();
    assert_eq!(
        route.resolve(&cx).to_string(),
        "ftp://files.example.com?lang=es-ES"
    );
}

// **< Route::external() / From<RoutePath> (share the same fixed resolution) >**********************

#[pagetop::test]
async fn route_external_ignores_the_rendering_context() {
    let route = Route::external("/fixed/path");

    let cx_es = cx_with_lang("es-ES");
    let cx_en = cx_with_lang("en-US");

    assert_eq!(route.resolve(&cx_es).to_string(), "/fixed/path");
    assert_eq!(route.resolve(&cx_en).to_string(), "/fixed/path");
}

#[pagetop::test]
async fn route_from_route_path_ignores_the_rendering_context() {
    let route: Route = RoutePath::new("/fixed/path").with_param("q", "rust").into();

    let cx_es = cx_with_lang("es-ES");
    let cx_en = cx_with_lang("en-US");

    assert_eq!(route.resolve(&cx_es).to_string(), "/fixed/path?q=rust");
    assert_eq!(route.resolve(&cx_en).to_string(), "/fixed/path?q=rust");
}

#[pagetop::test]
async fn route_external_never_adds_lang_even_for_unrecognized_schemes() {
    let cx = cx_with_lang("es-ES");

    let route = Route::external("ftp://files.example.com");
    assert_eq!(route.resolve(&cx).to_string(), "ftp://files.example.com");
}

// **< Route::with() >******************************************************************************

#[pagetop::test]
async fn route_with_captures_dynamic_values_from_the_environment() {
    let cx = cx_with_lang("es-ES");

    let user_id = 42;
    let route = Route::with(move |cx| cx.route(format!("/users/{user_id}")));

    assert_eq!(route.resolve(&cx).to_string(), "/users/42?lang=es-ES");
}

#[pagetop::test]
async fn route_default_resolves_to_an_empty_path() {
    let cx = Context::new(None);
    assert_eq!(Route::default().resolve(&cx).to_string(), "");
}

// **< Context::route() - direct protection against external URLs >*********************************

#[pagetop::test]
async fn context_route_never_adds_lang_to_an_external_looking_url() {
    let cx = cx_with_lang("es-ES");

    assert_eq!(
        cx.route("https://example.com/help").to_string(),
        "https://example.com/help"
    );
}
