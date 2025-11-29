use crate::base::component::Html;
use crate::core::component::Contextual;
use crate::core::theme::DefaultTemplate;
use crate::locale::L10n;
use crate::response::ResponseError;
use crate::service::http::{header::ContentType, StatusCode};
use crate::service::{HttpRequest, HttpResponse};

use super::Page;

use std::fmt;

/// Página de error asociada a un código de estado HTTP.
///
/// Este enumerado agrupa los distintos tipos de error que pueden devolverse como página HTML
/// completa. Cada variante encapsula la solicitud original ([`HttpRequest`]) y se corresponde con
/// un código de estado concreto.
///
/// Para algunos errores (como [`ErrorPage::AccessDenied`] y [`ErrorPage::NotFound`]) se construye
/// una [`Page`] usando la plantilla de error del tema activo ([`DefaultTemplate::Error`]), lo que
/// permite personalizar el contenido del mensaje. En el resto de casos se devuelve un cuerpo HTML
/// mínimo basado en una descripción genérica del error.
///
/// `ErrorPage` implementa [`ResponseError`], por lo que puede utilizarse directamente como tipo de
/// error en los controladores HTTP.
#[derive(Debug)]
pub enum ErrorPage {
    NotModified(HttpRequest),
    BadRequest(HttpRequest),
    AccessDenied(HttpRequest),
    NotFound(HttpRequest),
    PreconditionFailed(HttpRequest),
    InternalError(HttpRequest),
    Timeout(HttpRequest),
}

impl fmt::Display for ErrorPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Error 304.
            ErrorPage::NotModified(_) => f.write_str("Not Modified"),
            // Error 400.
            ErrorPage::BadRequest(_) => f.write_str("Bad Client Data"),
            // Error 403.
            ErrorPage::AccessDenied(request) => {
                let mut error_page = Page::new(request.clone());
                let error403 = error_page.theme().error403(&mut error_page);
                if let Ok(page) = error_page
                    .with_title(L10n::n("Error FORBIDDEN"))
                    .with_template(&DefaultTemplate::Error)
                    .add_child(Html::with(move |_| error403.clone()))
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    f.write_str("Access Denied")
                }
            }
            // Error 404.
            ErrorPage::NotFound(request) => {
                let mut error_page = Page::new(request.clone());
                let error404 = error_page.theme().error404(&mut error_page);
                if let Ok(page) = error_page
                    .with_title(L10n::n("Error RESOURCE NOT FOUND"))
                    .with_template(&DefaultTemplate::Error)
                    .add_child(Html::with(move |_| error404.clone()))
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    f.write_str("Not Found")
                }
            }
            // Error 412.
            ErrorPage::PreconditionFailed(_) => f.write_str("Precondition Failed"),
            // Error 500.
            ErrorPage::InternalError(_) => f.write_str("Internal Error"),
            // Error 504.
            ErrorPage::Timeout(_) => f.write_str("Timeout"),
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
            ErrorPage::NotModified(_)        => StatusCode::NOT_MODIFIED,
            ErrorPage::BadRequest(_)         => StatusCode::BAD_REQUEST,
            ErrorPage::AccessDenied(_)       => StatusCode::FORBIDDEN,
            ErrorPage::NotFound(_)           => StatusCode::NOT_FOUND,
            ErrorPage::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            ErrorPage::InternalError(_)      => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorPage::Timeout(_)            => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
