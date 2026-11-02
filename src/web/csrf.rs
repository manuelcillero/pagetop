// Protección frente a CSRF (*Cross-Site Request Forgery*) comprobando el origen de las peticiones,
// sin *tokens*, con el mismo criterio que `http.CrossOriginProtection` de Go:
//
// 1. Los métodos seguros (`GET`, `HEAD`, `OPTIONS`) se admiten siempre: no deben cambiar estado.
// 2. Con `Sec-Fetch-Site`, que envían todos los navegadores actuales, sólo se admiten `same-origin`
//    y `none` (dirección escrita a mano o marcador). `same-site` se rechaza también: una página en
//    otro subdominio no es la aplicación, y la cookie `SameSite=Lax` no la frena.
// 3. Sin `Sec-Fetch-Site` (navegadores antiguos, o HTTP sin cifrar fuera de `localhost`) decide
//    `Origin`, que debe coincidir con el servidor de la petición.
// 4. Sin ninguna de las dos no es un navegador (`curl`, otro servidor, las pruebas), o es uno de
//    hace años, anterior a enviar `Origin` en los `POST`. Un ataque CSRF necesita un navegador: se
//    admite.
//
// En los pasos 2 y 3 se admiten además los orígenes de `server.trusted_origins`.

use crate::global;
use crate::response::ErrorPage;
use crate::trace;
use crate::web::http::uri::Authority;
use crate::web::http::{HeaderMap, HeaderName, Method, header};
use crate::web::{HttpRequest, Request, Response};

use axum::extract::FromRequestParts;
use axum::middleware::Next;
use axum::response::IntoResponse;

const SEC_FETCH_SITE: HeaderName = HeaderName::from_static("sec-fetch-site");

// Rechaza con `403` las peticiones de otro origen que puedan cambiar estado. Va por fuera del
// *middleware* de las extensiones, así que una petición rechazada no llega a cargar la sesión.
pub(crate) async fn reject_cross_origin(request: Request, next: Next) -> Response {
    if is_allowed(
        request.method(),
        request.headers(),
        request.uri().authority(),
    ) {
        return next.run(request).await;
    }
    trace::warn!(
        "Cross-origin request rejected: {} {} (Sec-Fetch-Site: {:?}, Origin: {:?})",
        request.method(),
        request.uri().path(),
        request.headers().get(SEC_FETCH_SITE),
        request.headers().get(header::ORIGIN),
    );
    let (mut parts, _) = request.into_parts();
    let Ok(request) = HttpRequest::from_request_parts(&mut parts, &()).await;
    ErrorPage::AccessDenied(Some(request)).into_response()
}

// Con HTTP/2 el servidor llega en la URI (`:authority`) y no siempre en `Host`.
fn is_allowed(method: &Method, headers: &HeaderMap, authority: Option<&Authority>) -> bool {
    if matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS) {
        return true;
    }
    // Una cabecera `Origin` ilegible no coincide con nada, pero sigue indicando un navegador.
    let origin = headers
        .get(header::ORIGIN)
        .map(|v| v.to_str().unwrap_or_default());
    if let Some(site) = headers.get(SEC_FETCH_SITE) {
        let site = site.as_bytes();
        return site == b"same-origin" || site == b"none" || origin.is_some_and(is_trusted);
    }
    let Some(origin) = origin else {
        return true;
    };
    let host = authority
        .map(|a| a.as_str())
        .or_else(|| headers.get(header::HOST).and_then(|v| v.to_str().ok()));
    let same_host = origin
        .split_once("://")
        .zip(host)
        .is_some_and(|((_, origin_host), host)| origin_host.eq_ignore_ascii_case(host));
    same_host || is_trusted(origin)
}

// Los orígenes de confianza se normalizan en minúsculas al cargar la configuración.
fn is_trusted(origin: &str) -> bool {
    global::SETTINGS
        .server
        .trusted_origins
        .iter()
        .any(|trusted| trusted.eq_ignore_ascii_case(origin))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::web::http::HeaderValue;

    fn allowed(method: Method, headers: &[(&str, &str)], authority: Option<&str>) -> bool {
        let mut map = HeaderMap::new();
        for (name, value) in headers {
            map.insert(
                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                value.parse().unwrap(),
            );
        }
        let authority = authority.map(|a| a.parse().unwrap());
        is_allowed(&method, &map, authority.as_ref())
    }

    #[test]
    fn safe_methods_are_always_allowed() {
        let cross = [("sec-fetch-site", "cross-site")];
        assert!(allowed(Method::GET, &cross, None));
        assert!(allowed(Method::HEAD, &cross, None));
        assert!(allowed(Method::OPTIONS, &cross, None));
    }

    #[test]
    fn sec_fetch_site_decides_when_present() {
        assert!(allowed(
            Method::POST,
            &[("sec-fetch-site", "same-origin")],
            None
        ));
        assert!(allowed(Method::POST, &[("sec-fetch-site", "none")], None));
        // A sibling subdomain is "same-site" but not the application.
        assert!(!allowed(
            Method::POST,
            &[("sec-fetch-site", "same-site")],
            None
        ));
        assert!(!allowed(
            Method::PUT,
            &[("sec-fetch-site", "cross-site")],
            None
        ));
        // A matching Origin does not override a cross-site Sec-Fetch-Site.
        let headers = [
            ("sec-fetch-site", "cross-site"),
            ("origin", "http://example.com"),
        ];
        assert!(!allowed(Method::DELETE, &headers, Some("example.com")));
    }

    #[test]
    fn origin_must_match_the_host_without_sec_fetch_site() {
        let origin = [("origin", "https://example.com"), ("host", "example.com")];
        assert!(allowed(Method::POST, &origin, None));
        let origin = [("origin", "https://EXAMPLE.com:8443")];
        assert!(allowed(Method::POST, &origin, Some("example.com:8443")));
        let origin = [("origin", "https://evil.example"), ("host", "example.com")];
        assert!(!allowed(Method::POST, &origin, None));
        // Opaque origins (sandboxed iframes, some redirects).
        assert!(!allowed(
            Method::POST,
            &[("origin", "null"), ("host", "example.com")],
            None
        ));
        // An unreadable Origin is still a browser's, and matches nothing.
        let mut headers = HeaderMap::new();
        headers.insert(header::HOST, "example.com".parse().unwrap());
        headers.insert(
            header::ORIGIN,
            HeaderValue::from_bytes(b"https://ex\xe1mple.com").unwrap(),
        );
        assert!(!is_allowed(&Method::POST, &headers, None));
        // Without a known host nothing can match.
        assert!(!allowed(
            Method::PATCH,
            &[("origin", "https://example.com")],
            None
        ));
    }

    #[test]
    fn requests_without_browser_headers_are_allowed() {
        assert!(allowed(Method::POST, &[], None));
        assert!(allowed(Method::POST, &[("host", "example.com")], None));
    }
}
