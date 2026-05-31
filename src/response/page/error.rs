use crate::core::component::Contextual;
use crate::locale::L10n;
use crate::util;
use crate::web::{HttpRequest, IntoResponse, Response, http};

use super::Page;

use std::fmt;

/// Página de error asociada a un código de estado HTTP.
///
/// Este enumerado agrupa tipos esenciales de error que pueden devolverse como página HTML completa.
/// Cada variante encapsula la solicitud original ([`HttpRequest`]) y se corresponde con un código
/// de estado concreto.
///
/// Para cada error se construye una [`Page`] usando el tema activo, lo que permite personalizar
/// la plantilla y el contenido del mensaje mediante los métodos específicos del tema
/// (por ejemplo, [`Theme::error_403()`](crate::core::theme::Theme::error_403),
/// [`Theme::error_404()`](crate::core::theme::Theme::error_404) o
/// [`Theme::error_fatal()`](crate::core::theme::Theme::error_fatal)).
#[derive(Debug)]
pub enum ErrorPage {
    BadRequest(HttpRequest),
    AccessDenied(HttpRequest),
    NotFound(HttpRequest),
    InternalError(HttpRequest),
    ServiceUnavailable(HttpRequest),
    GatewayTimeout(HttpRequest),
}

impl ErrorPage {
    // Renderiza una página de error genérica usando el tema activo. Deriva las claves de
    // localización del código de estado (`error<code>_title`, `_alert`, `_help`). Si el
    // renderizado falla, escribe el texto plano del código de estado.
    fn display_error_page(&self, f: &mut fmt::Formatter<'_>, request: &HttpRequest) -> fmt::Result {
        let mut page = Page::new(request.clone());
        let code = self.status_code();
        page.theme().error_fatal(
            &mut page,
            code,
            L10n::l(util::join!("error", code.as_str(), "_title")),
            L10n::l(util::join!("error", code.as_str(), "_alert")),
            L10n::l(util::join!("error", code.as_str(), "_help")),
        );
        if let Ok(rendered) = page.render() {
            write!(f, "{}", rendered.into_string())
        } else {
            f.write_str(code.as_str())
        }
    }

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
}

impl fmt::Display for ErrorPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Error 400.
            Self::BadRequest(request) => self.display_error_page(f, request),

            // Error 403.
            Self::AccessDenied(request) => {
                let mut page = Page::new(request.clone());
                page.theme().error_403(&mut page);
                if let Ok(rendered) = page.render() {
                    write!(f, "{}", rendered.into_string())
                } else {
                    f.write_str(self.status_code().as_str())
                }
            }

            // Error 404.
            Self::NotFound(request) => {
                let mut page = Page::new(request.clone());
                page.theme().error_404(&mut page);
                if let Ok(rendered) = page.render() {
                    write!(f, "{}", rendered.into_string())
                } else {
                    f.write_str(self.status_code().as_str())
                }
            }

            // Error 500.
            Self::InternalError(request) => self.display_error_page(f, request),

            // Error 503.
            Self::ServiceUnavailable(request) => self.display_error_page(f, request),

            // Error 504.
            Self::GatewayTimeout(request) => self.display_error_page(f, request),
        }
    }
}

/// Convierte un [`ErrorPage`] en una respuesta HTTP con el código de estado adecuado y el cuerpo
/// HTML generado por el tema activo.
impl IntoResponse for ErrorPage {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = self.to_string();
        (
            status,
            [(http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
            body,
        )
            .into_response()
    }
}
