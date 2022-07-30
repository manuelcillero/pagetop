use actix_web::ResponseError;
use pagetop::prelude::*;

use static_files::Resource;

use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/mdbook.rs"));

static MDBOOK: LazyStatic<HashMap<&'static str, Resource>> = LazyStatic::new(||
    generate()
);

pub_const_handler!(MODULE_MDBOOK);

pub struct MdBook;

impl ModuleTrait for MdBook {
    fn handler(&self) -> Handler {
        MODULE_MDBOOK
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.service(
            app::web::scope("/doc")
                .route("{tail:.*html$}", app::web::get().to(mdbook_page))
                .route("{tail:.*$}", app::web::get().to(mdbook_resource))
        );
    }
}

async fn mdbook_page(request: app::HttpRequest) -> ResultPage<Markup, FatalError> {
    // Remove initial "/doc/" from path:
    let path = &request.path()[5..];

    if let Some(content) = MDBOOK.get(path) {
        if let Ok(html) = std::str::from_utf8(content.data) {

            let _lang = extract("Lang", html);
            let title = match extract("Title", html) {
                Some(title) => title,
                _ => "Documentación",
            };
            let _print = match extract("Print", html) {
                Some("enabled") => true,
                _ => false,
            };
            let _mathjax = match extract("MathJax", html) {
                Some("supported") => true,
                _ => false,
            };
            let beginning = {
                let separator = "<!-- mdBook -->";
                match html.find(separator) {
                    Some(pos) => pos + separator.len(),
                    _ => 0,
                }
            };

            Page::new()
                .with_title(title)
                .with_context(PageOp::AddMetadata("theme-color", "#ffffff"))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/css/variables.css")
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/css/general.css")
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/css/chrome.css")
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/css/print.css").for_media(TargetMedia::Print)
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/FontAwesome/css/font-awesome.css")
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/fonts/fonts.css")
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/highlight.css")
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/tomorrow-night.css")
                ))
                .with_context(PageOp::AddStyleSheet(
                    StyleSheet::located("/doc/ayu-highlight.css")
                ))
                .add_to(
                    "region-content",
                    Container::new()
                        .with_id("mdbook")
                        .with_component(Html::with(html! { (PreEscaped(&html[beginning..])) }))
                )
                .render()
        } else {
            Err(FatalError::NotFound)
        }
    } else {
        Err(FatalError::NotFound)
    }
}

async fn mdbook_resource(request: app::HttpRequest) -> app::HttpResponse {
    // Remove initial "/doc/" from path:
    let path = &request.path()[5..];

    // From https://github.com/kilork/actix-web-static-files/blob/master/src/resource_files.rs, see
    // functions respond_to(), any_match() and none_match().
    if let Some(file) = &MDBOOK.get(path) {
        let etag = Some(app::http::header::EntityTag::new_strong(format!(
            "{:x}:{:x}",
            file.data.len(),
            file.modified
        )));

        let precondition_failed = !any_match(etag.as_ref(), &request);

        let not_modified = !none_match(etag.as_ref(), &request);

        let mut resp = app::HttpResponse::build(app::http::StatusCode::OK);
        resp.insert_header((app::http::header::CONTENT_TYPE, file.mime_type));

        if let Some(etag) = etag {
            resp.insert_header(app::http::header::ETag(etag));
        }

        if precondition_failed {
            return FatalError::PreconditionFailed.error_response();
        } else if not_modified {
            return FatalError::NotModified.error_response();
        }

        resp.body(file.data)
    } else {
        FatalError::NotFound.error_response()
    }
}

/// Returns true if `request` has no `If-Match` header or one which matches `etag`.
fn any_match(etag: Option<&app::http::header::EntityTag>, request: &app::HttpRequest) -> bool {
    match request.get_header::<app::http::header::IfMatch>() {
        None | Some(app::http::header::IfMatch::Any) => true,
        Some(app::http::header::IfMatch::Items(ref items)) => {
            if let Some(some_etag) = etag {
                for item in items {
                    if item.strong_eq(some_etag) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

/// Returns true if `request` doesn't have an `If-None-Match` header matching `req`.
fn none_match(etag: Option<&app::http::header::EntityTag>, request: &app::HttpRequest) -> bool {
    match request.get_header::<app::http::header::IfNoneMatch>() {
        Some(app::http::header::IfNoneMatch::Any) => false,
        Some(app::http::header::IfNoneMatch::Items(ref items)) => {
            if let Some(some_etag) = etag {
                for item in items {
                    if item.weak_eq(some_etag) {
                        return false;
                    }
                }
            }
            true
        }
        None => true,
    }
}

fn extract(attr: &'static str, from: &'static str) -> Option<&'static str> {
    let search = concat_string!("<!-- ", attr, ":");
    if let Some(ini) = from.find(&search) {
        let ini = ini + search.len() + 1;
        if let Some(end) = from[ini..].find("-->").map(|i| i + ini) {
            let end = end - 1;
            return Some(&from[ini..end]);
        }
    }
    None
}
