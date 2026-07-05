use axum::extract::Request;
use axum::middleware::Next;

use crate::core::component::Contextual;
use crate::locale::L10n;
use crate::util;
use crate::web::{HttpRequest, IntoResponse, Response, http};

use super::Page;

/// Página de error asociada a un código de estado HTTP.
///
/// Este enumerado agrupa tipos esenciales de error que pueden devolverse como página HTML completa.
/// Cada variante encapsula la solicitud original ([`HttpRequest`]) y se corresponde con un código
/// de estado concreto.
///
/// Para cada error se construye una [`Page`] usando el tema activo, lo que permite personalizar la
/// plantilla y el contenido del mensaje mediante los métodos específicos del tema (por ejemplo,
/// [`Theme::error_403()`](crate::core::theme::Theme::error_403),
/// [`Theme::error_404()`](crate::core::theme::Theme::error_404) o
/// [`Theme::error_fatal()`](crate::core::theme::Theme::error_fatal)).
#[derive(Clone, Debug)]
pub enum ErrorPage {
    BadRequest(HttpRequest),
    AccessDenied(HttpRequest),
    NotFound(HttpRequest),
    InternalError(HttpRequest),
    ServiceUnavailable(HttpRequest),
    GatewayTimeout(HttpRequest),
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
                let mut page = Page::new(request);
                page.theme().error_403(&mut page);
                page
            }
            Self::NotFound(request) => {
                let mut page = Page::new(request);
                page.theme().error_404(&mut page);
                page
            }
            Self::BadRequest(request)
            | Self::InternalError(request)
            | Self::ServiceUnavailable(request)
            | Self::GatewayTimeout(request) => {
                let mut page = Page::new(request);
                page.theme().error_fatal(
                    &mut page,
                    status,
                    L10n::l(util::join!("error", status.as_str(), "_title")),
                    L10n::l(util::join!("error", status.as_str(), "_alert")),
                    L10n::l(util::join!("error", status.as_str(), "_help")),
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

// Intercepta respuestas con un [`ErrorPage`] pendiente y las convierte en páginas HTML completas
// usando el tema activo.
//
// Se registra globalmente sobre el router principal desde [`crate::app`].
pub(crate) async fn render_error_pages(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    if let Some(error_page) = response.extensions_mut().remove::<ErrorPage>() {
        return error_page.render_html().await;
    }
    response
}
