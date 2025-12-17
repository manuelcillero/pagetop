use crate::core::component::Contextual;
use crate::locale::L10n;
use crate::response::ResponseError;
use crate::service::http::{header::ContentType, StatusCode};
use crate::service::{HttpRequest, HttpResponse};
use crate::util;

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
    /// Función auxiliar para renderizar una página de error genérica usando el tema activo.
    ///
    /// Construye una [`Page`] a partir de la petición y un prefijo de clave basado en el código de
    /// estado (`error<code>`), del que se derivan los textos localizados `error<code>_title`,
    /// `error<code>_alert` y `error<code>_help`.
    ///
    /// Si el renderizado falla, escribe en su lugar el texto plano asociado al código de estado.
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
            f.write_str(&code.to_string())
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
                    f.write_str(&self.status_code().to_string())
                }
            }

            // Error 404.
            Self::NotFound(request) => {
                let mut page = Page::new(request.clone());
                page.theme().error_404(&mut page);
                if let Ok(rendered) = page.render() {
                    write!(f, "{}", rendered.into_string())
                } else {
                    f.write_str(&self.status_code().to_string())
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

impl ResponseError for ErrorPage {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    #[rustfmt::skip]
    fn status_code(&self) -> StatusCode {
        match self {
            ErrorPage::BadRequest(_)         => StatusCode::BAD_REQUEST,
            ErrorPage::AccessDenied(_)       => StatusCode::FORBIDDEN,
            ErrorPage::NotFound(_)           => StatusCode::NOT_FOUND,
            ErrorPage::InternalError(_)      => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorPage::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ErrorPage::GatewayTimeout(_)     => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
