use axum::extract::Request;
use axum::middleware::Next;

use crate::core::component::Contextual;
use crate::html::Markup;
use crate::locale::Lc;
use crate::web::{HttpRequest, IntoResponse, Response, http};
use crate::{trace, util};

use super::Page;

use std::any::Any;

// **< Errores controlados >************************************************************************

/// Página de error asociada a un código de estado HTTP.
///
/// Este enumerado agrupa tipos esenciales de error que pueden devolverse como página HTML completa.
/// Cada variante encapsula la petición original si está disponible ([`HttpRequest`]), y se asocia a
/// un código de estado concreto.
///
/// Para cada error se construye una [`Page`] usando el tema activo, lo que permite personalizar la
/// plantilla y el contenido del mensaje con los métodos específicos del tema, como
/// [`Theme::error_403()`], [`Theme::error_404()`] o [`Theme::error_fatal()`].
///
/// Sin `request` (`None`), la página se renderiza igualmente, pero sin el idioma negociado ni el
/// usuario actual, que dependen de la petición original.
///
/// [`Theme::error_403()`]: crate::core::theme::Theme::error_403
/// [`Theme::error_404()`]: crate::core::theme::Theme::error_404
/// [`Theme::error_fatal()`]: crate::core::theme::Theme::error_fatal
#[derive(Clone, Debug)]
pub enum ErrorPage {
    /// Petición incorrecta (400). El servidor no puede procesar la petición tal y como está
    /// formulada (datos malformados, parámetros inválidos, etc.).
    BadRequest(Option<HttpRequest>),
    /// Acceso denegado (403). El usuario actual no tiene permiso para acceder al recurso.
    ///
    /// Se renderiza con [`Theme::error_403()`](crate::core::theme::Theme::error_403).
    AccessDenied(Option<HttpRequest>),
    /// Recurso no encontrado (404). La ruta solicitada no existe o no coincide con ningún handler.
    ///
    /// Se renderiza con [`Theme::error_404()`](crate::core::theme::Theme::error_404).
    NotFound(Option<HttpRequest>),
    /// Error interno del servidor (500). Un fallo controlado (no un `panic!`) impide completar la
    /// petición.
    InternalError(Option<HttpRequest>),
    /// Servicio no disponible (503). El servidor no puede atender la petición temporalmente
    /// (mantenimiento, sobrecarga, etc.).
    ServiceUnavailable(Option<HttpRequest>),
    /// Tiempo de espera agotado (504). Una dependencia externa (proxy, servicio remoto) no ha
    /// respondido a tiempo.
    GatewayTimeout(Option<HttpRequest>),
}

impl ErrorPage {
    /// Devuelve el código de estado HTTP asociado a la variante de error.
    pub fn status_code(&self) -> http::StatusCode {
        match self {
            ErrorPage::BadRequest(_) => http::StatusCode::BAD_REQUEST,
            ErrorPage::AccessDenied(_) => http::StatusCode::FORBIDDEN,
            ErrorPage::NotFound(_) => http::StatusCode::NOT_FOUND,
            ErrorPage::InternalError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorPage::ServiceUnavailable(_) => http::StatusCode::SERVICE_UNAVAILABLE,
            ErrorPage::GatewayTimeout(_) => http::StatusCode::GATEWAY_TIMEOUT,
        }
    }

    // Renderiza la página de error y construye la respuesta HTTP completa con el HTML generado. Si
    // el renderizado falla, devuelve sólo el código de estado sin cuerpo.
    async fn render_html(self) -> Response {
        let status = self.status_code();
        let mut page = match self {
            Self::AccessDenied(request) => {
                let mut page = Page::default().with_request(request);
                page.theme().error_403(&mut page);
                page
            }
            Self::NotFound(request) => {
                let mut page = Page::default().with_request(request);
                page.theme().error_404(&mut page);
                page
            }
            Self::BadRequest(request)
            | Self::InternalError(request)
            | Self::ServiceUnavailable(request)
            | Self::GatewayTimeout(request) => {
                let mut page = Page::default().with_request(request);
                page.theme().error_fatal(
                    &mut page,
                    status,
                    Lc::l(util::join!("error", status.as_str(), "_title")),
                    Lc::l(util::join!("error", status.as_str(), "_alert")),
                    Lc::l(util::join!("error", status.as_str(), "_help")),
                );
                page
            }
        };
        match page.render().await {
            Ok(rendered) => (
                status,
                [(http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
                rendered.into_string(),
            )
                .into_response(),
            // Si renderizar la propia página de error falla, se descarta el `ErrorPage` resultante
            // en vez de intentar renderizarlo de nuevo, para no arriesgarse a una recursión si el
            // fallo persiste.
            Err(_) => status.into_response(),
        }
    }
}

/// Convierte un [`ErrorPage`] en una respuesta HTTP que luego se convertirá en una página HTML
/// completa usando el tema activo.
impl IntoResponse for ErrorPage {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let mut response = status.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

// Gestión de las rutas sin coincidencia.
//
// Se registra como `.fallback()` del router principal desde [`Application`](crate::Application).
pub(crate) async fn route_not_found(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Err(ErrorPage::NotFound(Some(request)))
}

// Intercepta respuestas con un [`ErrorPage`] pendiente y las convierte en páginas HTML.
//
// Se registra globalmente sobre el router principal desde [`Application`](crate::Application).
pub(crate) async fn render_error_pages(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    if let Some(error_page) = response.extensions_mut().remove::<ErrorPage>() {
        return error_page.render_html().await;
    }
    response
}

// **< Fallo catastrófico >*************************************************************************

// HTML mínimo para un fallo catastrófico (`panic!`). No usa el tema, ni componentes, ni acceso a
// datos, porque el fallo podría estar precisamente ahí. Tampoco se traduce porque ni siquiera el
// sistema de localización es seguro; tampoco hay forma de elegir idioma vía `Accept-Language`.
const FATAL_ERROR_HTML: &str = concat!(
    "<!DOCTYPE html>",
    "<html lang=\"en\">",
    "<head>",
    "<meta charset=\"utf-8\">",
    "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">",
    "<title>Unexpected error</title>",
    "</head><body>",
    "<h1>An unexpected error has occurred</h1>",
    "<p>Sorry for the inconvenience. Please try again or contact your system administrator.</p>",
    "</body>",
    "</html>",
);

// Captura el fallo catastrófico (`panic!`) con el motivo para diagnóstico y responde un HTTP 500.
//
// Se registra sobre el router principal desde [`Application`](crate::Application).
pub(crate) fn response_for_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let reason = err
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| err.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "panic with no message".to_string());
    trace::error!(panic = %reason, "Unhandled panic caught by CatchPanicLayer");

    (
        http::StatusCode::INTERNAL_SERVER_ERROR,
        [(http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        FATAL_ERROR_HTML,
    )
        .into_response()
}
