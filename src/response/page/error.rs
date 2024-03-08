use crate::base::component::{Error403, Error404};
use crate::locale::L10n;
use crate::response::ResponseError;
use crate::service::http::{header::ContentType, StatusCode};
use crate::service::{HttpRequest, HttpResponse};

use super::Page;

use std::fmt;

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
            ErrorPage::NotModified(_) => write!(f, "Not Modified"),
            // Error 400.
            ErrorPage::BadRequest(_) => write!(f, "Bad Client Data"),
            // Error 403.
            ErrorPage::AccessDenied(request) => {
                let error_page = Page::new(request.clone());
                if let Ok(page) = error_page
                    .with_title(L10n::n("Error FORBIDDEN"))
                    .with_template("error")
                    .with_component(Error403)
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Access Denied")
                }
            }
            // Error 404.
            ErrorPage::NotFound(request) => {
                let error_page = Page::new(request.clone());
                if let Ok(page) = error_page
                    .with_title(L10n::n("Error RESOURCE NOT FOUND"))
                    .with_template("error")
                    .with_component(Error404)
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Not Found")
                }
            }
            // Error 412.
            ErrorPage::PreconditionFailed(_) => write!(f, "Precondition Failed"),
            // Error 500.
            ErrorPage::InternalError(_) => write!(f, "Internal Error"),
            // Error 504.
            ErrorPage::Timeout(_) => write!(f, "Timeout"),
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
