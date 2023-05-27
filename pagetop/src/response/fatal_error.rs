mod error403;
pub use error403::ERROR_403;
mod error404;
pub use error404::ERROR_404;

use crate::core::component::L10n;
use crate::response::{page::Page, ResponseError};
use crate::server::http::{header::ContentType, StatusCode};
use crate::server::{HttpRequest, HttpResponse};

use std::fmt;

#[derive(Debug)]
pub enum FatalError {
    NotModified(HttpRequest),
    BadRequest(HttpRequest),
    AccessDenied(HttpRequest),
    NotFound(HttpRequest),
    PreconditionFailed(HttpRequest),
    InternalError(HttpRequest),
    Timeout(HttpRequest),
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Error 304.
            FatalError::NotModified(_) => write!(f, "Not Modified"),
            // Error 400.
            FatalError::BadRequest(_) => write!(f, "Bad Client Data"),
            // Error 403.
            FatalError::AccessDenied(request) => {
                let error_page = Page::new(request.clone());
                if let Ok(page) = error_page
                    .with_title(L10n::n("Error FORBIDDEN"))
                    .with_this_in("region-content", error403::Error403)
                    .with_template("error")
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Access Denied")
                }
            }
            // Error 404.
            FatalError::NotFound(request) => {
                let error_page = Page::new(request.clone());
                if let Ok(page) = error_page
                    .with_title(L10n::n("Error RESOURCE NOT FOUND"))
                    .with_this_in("region-content", error404::Error404)
                    .with_template("error")
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Not Found")
                }
            }
            // Error 412.
            FatalError::PreconditionFailed(_) => write!(f, "Precondition Failed"),
            // Error 500.
            FatalError::InternalError(_) => write!(f, "Internal Error"),
            // Error 504.
            FatalError::Timeout(_) => write!(f, "Timeout"),
        }
    }
}

impl ResponseError for FatalError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    #[rustfmt::skip]
    fn status_code(&self) -> StatusCode {
        match self {
            FatalError::NotModified(_)        => StatusCode::NOT_MODIFIED,
            FatalError::BadRequest(_)         => StatusCode::BAD_REQUEST,
            FatalError::AccessDenied(_)       => StatusCode::FORBIDDEN,
            FatalError::NotFound(_)           => StatusCode::NOT_FOUND,
            FatalError::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            FatalError::InternalError(_)      => StatusCode::INTERNAL_SERVER_ERROR,
            FatalError::Timeout(_)            => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
